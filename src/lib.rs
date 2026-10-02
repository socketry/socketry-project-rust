// Released under the MIT License.
// Copyright, 2026, by Samuel Williams.

//! Shared Socketry Rust project conventions and development tasks.
//!
//! Add this crate to a private `bake/` package to link the standard project
//! tasks and their published agent context into a repository.
use bake::{Context, Result};
use bake_agent_context as _;
use bake_cargo as _;
use bake_license as _;
use bake_readme as _;
use bake_releases as _;
use bake_test_rust as _;

/// Refresh the standard project files after a Cargo version bump.
#[bake::task(name = "cargo:after_version_bump")]
pub fn after_version_bump(context: &mut Context, version: String) -> Result<()> {
    context.call("license:update", &[])?;
    context.call("releases:update", &[&format!("v{version}")])?;
    context.call("readme:update", &[])?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use bake::{Arguments, Parameter, Registry, Result, Task, Value};

    #[derive(Default)]
    struct Calls(Vec<(String, Option<String>)>);

    fn record(context: &mut bake::Context, name: &str, value: Option<String>) -> Result<Value> {
        context
            .get_mut::<Calls>()
            .expect("the test records calls")
            .0
            .push((name.to_owned(), value));
        Ok(Value::Null)
    }

    fn update_license(context: &mut bake::Context, _: &Arguments) -> Result<Value> {
        record(context, "license:update", None)
    }

    fn update_releases(context: &mut bake::Context, arguments: &Arguments) -> Result<Value> {
        record(
            context,
            "releases:update",
            Some(arguments.required::<String>("version")?),
        )
    }

    fn update_readme(context: &mut bake::Context, _: &Arguments) -> Result<Value> {
        record(context, "readme:update", None)
    }

    #[test]
    fn refreshes_project_files_in_order_for_the_requested_version() {
        let mut registry = Registry::discover().unwrap();
        registry
            .replace(
                "license:update",
                Task::new("license:update", "", vec![], update_license),
            )
            .unwrap();
        registry
            .replace(
                "releases:update",
                Task::new(
                    "releases:update",
                    "",
                    vec![Parameter::new::<String>("version")],
                    update_releases,
                ),
            )
            .unwrap();
        registry
            .replace(
                "readme:update",
                Task::new("readme:update", "", vec![], update_readme),
            )
            .unwrap();

        let mut context = registry.context(".");
        context.insert(Calls::default());

        super::after_version_bump(&mut context, "0.3.0".to_owned()).unwrap();

        assert_eq!(
            context.get::<Calls>().unwrap().0,
            [
                ("license:update".to_owned(), None),
                ("releases:update".to_owned(), Some("v0.3.0".to_owned())),
                ("readme:update".to_owned(), None),
            ]
        );
    }

    #[test]
    fn registers_the_project_version_bump_hook() {
        let registry = Registry::discover().unwrap();
        assert!(
            registry
                .tasks()
                .any(|task| task.name() == "cargo:after_version_bump")
        );
    }
}
