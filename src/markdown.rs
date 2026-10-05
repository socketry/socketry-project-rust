// Released under the MIT License.
// Copyright, 2026, by Samuel Williams.

use bake::{Context, Error, Result};
use std::ffi::{OsStr, OsString};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const STANDARD_FILES: [&str; 3] = ["license.md", "readme.md", "releases.md"];

pub(super) fn normalize(context: &mut Context) -> Result<()> {
    let paths = markdown_paths(context.root())?;
    normalize_paths(context, paths)
}

fn normalize_paths(context: &mut Context, paths: Vec<PathBuf>) -> Result<()> {
    let mut owned_arguments = Vec::with_capacity(paths.len() * 2);

    for path in paths {
        owned_arguments.push("--path".to_owned());
        owned_arguments.push(path_argument(&path)?);
    }

    let arguments: Vec<_> = owned_arguments.iter().map(String::as_str).collect();
    context.call("markdown:normalize", &arguments)?;

    Ok(())
}

fn path_argument(path: &Path) -> Result<String> {
    path.to_str()
        .map(str::to_owned)
        .ok_or_else(|| Error::new("Markdown paths must be valid UTF-8"))
}

fn markdown_paths(root: &Path) -> Result<Vec<PathBuf>> {
    markdown_paths_with(&LocalFileSystem, root)
}

fn markdown_paths_with<FileSystemType: FileSystem>(
    file_system: &FileSystemType,
    root: &Path,
) -> Result<Vec<PathBuf>> {
    let mut paths: Vec<_> = STANDARD_FILES.iter().map(PathBuf::from).collect();
    collect_context_markdown_paths(file_system, root, Path::new("context"), &mut paths, true)?;
    paths.sort();

    Ok(paths)
}

fn collect_context_markdown_paths<FileSystemType: FileSystem>(
    file_system: &FileSystemType,
    root: &Path,
    directory: &Path,
    paths: &mut Vec<PathBuf>,
    is_context_root: bool,
) -> Result<()> {
    let full_directory = root.join(directory);
    let entries = match file_system.read_dir(&full_directory) {
        Ok(entries) => entries,
        Err(error) if is_context_root && error.kind() == io::ErrorKind::NotFound => {
            return Ok(());
        }
        Err(error) => {
            return Err(Error::new(format!(
                "failed to read {}: {error}",
                full_directory.display()
            )));
        }
    };

    for entry in entries {
        let entry = entry.map_err(|error| {
            Error::new(format!(
                "failed to read an entry in {}: {error}",
                full_directory.display()
            ))
        })?;
        let path = directory.join(entry.name());
        let kind = entry.kind().map_err(|error| {
            Error::new(format!("failed to inspect {}: {error}", path.display()))
        })?;

        if kind == EntryKind::Directory {
            collect_context_markdown_paths(file_system, root, &path, paths, false)?;
        } else if kind == EntryKind::File && path.extension() == Some(OsStr::new("md")) {
            paths.push(path);
        }
    }

    Ok(())
}

trait FileSystem {
    type Entry: DirectoryEntry;
    type Entries: Iterator<Item = io::Result<Self::Entry>>;

    fn read_dir(&self, path: &Path) -> io::Result<Self::Entries>;
}

trait DirectoryEntry {
    fn name(&self) -> OsString;
    fn kind(&self) -> io::Result<EntryKind>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum EntryKind {
    Directory,
    File,
    Other,
}

struct LocalFileSystem;

impl FileSystem for LocalFileSystem {
    type Entry = fs::DirEntry;
    type Entries = fs::ReadDir;

    fn read_dir(&self, path: &Path) -> io::Result<Self::Entries> {
        fs::read_dir(path)
    }
}

impl DirectoryEntry for fs::DirEntry {
    fn name(&self) -> OsString {
        self.file_name()
    }

    fn kind(&self) -> io::Result<EntryKind> {
        entry_kind(self.file_type())
    }
}

fn entry_kind(file_type: io::Result<fs::FileType>) -> io::Result<EntryKind> {
    let file_type = file_type?;

    if file_type.is_dir() {
        Ok(EntryKind::Directory)
    } else if file_type.is_file() {
        Ok(EntryKind::File)
    } else {
        Ok(EntryKind::Other)
    }
}

#[cfg(test)]
#[path = "markdown_tests.rs"]
mod tests;
