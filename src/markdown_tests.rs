// Released under the MIT License.
// Copyright, 2026, by Samuel Williams.

use super::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;

#[test]
fn selects_standard_markdown_and_recursive_public_context_files() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path();
    fs::create_dir_all(root.join("context/nested")).unwrap();
    fs::create_dir_all(root.join(".agents/context")).unwrap();
    fs::write(root.join("context/guide.md"), "Guide.\n").unwrap();
    fs::write(root.join("context/nested/skill.md"), "Skill.\n").unwrap();
    fs::write(root.join("context/nested/notes.txt"), "Not Markdown.\n").unwrap();
    fs::write(root.join(".agents/context/local.md"), "Local only.\n").unwrap();
    #[cfg(unix)]
    std::os::unix::fs::symlink(
        root.join("context/guide.md"),
        root.join("context/nested/link.md"),
    )
    .unwrap();

    let paths = markdown_paths(root).unwrap();

    assert_eq!(
        paths,
        [
            PathBuf::from("context/guide.md"),
            PathBuf::from("context/nested/skill.md"),
            PathBuf::from("license.md"),
            PathBuf::from("readme.md"),
            PathBuf::from("releases.md"),
        ]
    );
}

#[test]
fn skips_context_when_the_public_context_directory_is_absent() {
    let directory = tempfile::tempdir().unwrap();

    let paths = markdown_paths(directory.path()).unwrap();

    assert_eq!(
        paths,
        [
            PathBuf::from("license.md"),
            PathBuf::from("readme.md"),
            PathBuf::from("releases.md"),
        ]
    );
}

#[test]
fn reports_when_context_is_not_a_directory() {
    let directory = tempfile::tempdir().unwrap();
    fs::write(directory.path().join("context"), "Not a directory.\n").unwrap();

    let error = markdown_paths(directory.path()).unwrap_err();

    assert!(error.to_string().contains("failed to read"));
}

#[test]
fn propagates_context_walk_errors_from_the_normalizer() {
    let directory = tempfile::tempdir().unwrap();
    fs::write(directory.path().join("context"), "Not a directory.\n").unwrap();
    let mut context = bake::Registry::new().context(directory.path());

    let error = normalize(&mut context).unwrap_err();

    assert!(error.to_string().contains("failed to read"));
}

#[test]
fn reports_when_a_directory_entry_cannot_be_read() {
    let file_system = FakeFileSystem::new([(
        PathBuf::from("project/context"),
        FakeDirectory::Entries(vec![Err(io::Error::other("entry unavailable"))]),
    )]);
    let mut paths = Vec::new();

    let error = collect_context_markdown_paths(
        &file_system,
        Path::new("project"),
        Path::new("context"),
        &mut paths,
        true,
    )
    .unwrap_err();

    assert!(error.to_string().contains("entry unavailable"));
}

#[test]
fn reports_when_entry_metadata_cannot_be_read() {
    let file_system = FakeFileSystem::new([(
        PathBuf::from("project/context"),
        FakeDirectory::Entries(vec![Ok(FakeEntry {
            name: OsString::from("guide.md"),
            kind: Err(io::Error::other("metadata unavailable")),
        })]),
    )]);
    let mut paths = Vec::new();

    let error = collect_context_markdown_paths(
        &file_system,
        Path::new("project"),
        Path::new("context"),
        &mut paths,
        true,
    )
    .unwrap_err();

    assert!(error.to_string().contains("metadata unavailable"));
}

#[test]
fn reports_when_a_nested_directory_cannot_be_read() {
    let file_system = FakeFileSystem::new([
        (
            PathBuf::from("project/context"),
            FakeDirectory::Entries(vec![Ok(FakeEntry::new("nested", EntryKind::Directory))]),
        ),
        (
            PathBuf::from("project/context/nested"),
            FakeDirectory::Error(io::ErrorKind::PermissionDenied),
        ),
    ]);
    let mut paths = Vec::new();

    let error = collect_context_markdown_paths(
        &file_system,
        Path::new("project"),
        Path::new("context"),
        &mut paths,
        true,
    )
    .unwrap_err();

    assert!(error.to_string().contains("project/context/nested"));
}

#[test]
fn ignores_other_filesystem_entries() {
    let file_system = FakeFileSystem::new([(
        PathBuf::from("project/context"),
        FakeDirectory::Entries(vec![Ok(FakeEntry::new("link.md", EntryKind::Other))]),
    )]);
    let mut paths = Vec::new();

    collect_context_markdown_paths(
        &file_system,
        Path::new("project"),
        Path::new("context"),
        &mut paths,
        true,
    )
    .unwrap();

    assert!(paths.is_empty());
}

#[test]
fn reports_when_the_normalize_task_is_not_registered() {
    let directory = tempfile::tempdir().unwrap();
    let mut context = bake::Registry::new().context(directory.path());

    let error = normalize(&mut context).unwrap_err();

    assert!(error.to_string().contains("markdown:normalize"));
}

#[cfg(unix)]
#[test]
fn propagates_path_conversion_errors_from_the_normalizer() {
    use std::os::unix::ffi::OsStrExt;

    let directory = tempfile::tempdir().unwrap();
    let mut context = bake::Registry::new().context(directory.path());
    let path = PathBuf::from(OsStr::from_bytes(&[0xff, b'.', b'm', b'd']));

    let error = normalize_paths(&mut context, vec![path]).unwrap_err();

    assert_eq!(error.to_string(), "Markdown paths must be valid UTF-8");
}

#[test]
fn reports_file_type_errors() {
    let error = entry_kind(Err(io::Error::other("metadata unavailable"))).unwrap_err();

    assert_eq!(error.to_string(), "metadata unavailable");
}

enum FakeDirectory {
    Entries(Vec<io::Result<FakeEntry>>),
    Error(io::ErrorKind),
}

struct FakeFileSystem {
    directories: RefCell<HashMap<PathBuf, FakeDirectory>>,
}

impl FakeFileSystem {
    fn new(directories: impl IntoIterator<Item = (PathBuf, FakeDirectory)>) -> Self {
        Self {
            directories: RefCell::new(directories.into_iter().collect()),
        }
    }
}

impl FileSystem for FakeFileSystem {
    type Entry = FakeEntry;
    type Entries = std::vec::IntoIter<io::Result<FakeEntry>>;

    fn read_dir(&self, path: &Path) -> io::Result<Self::Entries> {
        let response = self
            .directories
            .borrow_mut()
            .remove(path)
            .unwrap_or(FakeDirectory::Entries(Vec::new()));

        match response {
            FakeDirectory::Entries(entries) => Ok(entries.into_iter()),
            FakeDirectory::Error(kind) => Err(io::Error::from(kind)),
        }
    }
}

struct FakeEntry {
    name: OsString,
    kind: io::Result<EntryKind>,
}

impl FakeEntry {
    fn new(name: &str, kind: EntryKind) -> Self {
        Self {
            name: OsString::from(name),
            kind: Ok(kind),
        }
    }
}

impl DirectoryEntry for FakeEntry {
    fn name(&self) -> OsString {
        self.name.clone()
    }

    fn kind(&self) -> io::Result<EntryKind> {
        match &self.kind {
            Ok(kind) => Ok(*kind),
            Err(error) => Err(io::Error::new(error.kind(), error.to_string())),
        }
    }
}
