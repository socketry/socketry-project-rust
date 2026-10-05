// Released under the MIT License.
// Copyright, 2026, by Samuel Williams.

use bake::Registry;
use bake_markdown as _;
use std::process::ExitCode;

fn main() -> ExitCode {
    match Registry::discover().and_then(|registry| registry.run().map(|_| ())) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("bake: {error}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use bake::Registry;

    #[test]
    fn includes_the_standard_project_tasks() {
        let registry = Registry::discover().unwrap();
        let names: Vec<_> = registry.tasks().map(|task| task.name()).collect();

        assert!(names.contains(&"agent:context:install"));
        assert!(names.contains(&"cargo:release"));
        assert!(names.contains(&"cargo:after_version_bump"));
        assert!(names.contains(&"license:update"));
        assert!(names.contains(&"markdown:normalize"));
        assert!(names.contains(&"readme:update"));
        assert!(names.contains(&"releases:update"));
    }
}

#[path = "bake_generated_tasks/mod.rs"]
mod bake_generated_tasks;
