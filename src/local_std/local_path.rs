use std::{
    fs::{self, DirEntry},
    path::{Path, PathBuf},
};

#[derive(Debug)]
struct FileSystem {
    parent_directory: PathBuf,
    child_directory: PathBuf,
    current_directory_content: Vec<PathBuf>,
}

pub fn path_module_main() -> std::io::Result<()> {
    let path = Path::new("./src/local_std");

    let current_directory_content: Vec<DirEntry> =
        fs::read_dir(path)?.collect::<Result<Vec<DirEntry>, _>>()?;

    let file_system: FileSystem = FileSystem {
        parent_directory: path.parent().unwrap().to_path_buf(),
        child_directory: path.to_path_buf(),
        current_directory_content: map_dir_entry_to_path_buf(current_directory_content),
    };

    println!("{:?}", file_system);
    Ok(())
}

pub fn map_dir_entry_to_path_buf(current_dictory_entry: Vec<DirEntry>) -> Vec<PathBuf> {
    let mut current_directory_entry_in_path_buf: Vec<PathBuf> = Vec::new();
    for current_entry in current_dictory_entry {
        current_directory_entry_in_path_buf.push(current_entry.path());
    }
    current_directory_entry_in_path_buf
}
