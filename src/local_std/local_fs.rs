use std::{fs, path::Path};

/// create_dir create a single empty directory at the provided Path
/// it takes a &Path
/// An error will be returned if
///     - the user doesn't have the permission to create the directory
/// at the current &Path.
///     - A parent of the given &Path doesn't exist.
///     - The folder exists.
pub fn execute_create_dir(path: &Path) -> std::io::Result<()> {
    fs::create_dir(path)?;
    Ok(())
}

pub fn execute_delete_dir(path: &Path) -> std::io::Result<()> {
    fs::remove_dir(path)?;
    Ok(())
}


#[cfg(test)]
mod test {
    use std::{fs, path::Path};

    use crate::local_std::local_fs::{execute_delete_dir, execute_create_dir};

    #[test]
    fn should_create_single_dir() {
        let path = Path::new("./test-folder");
        execute_create_dir(path).expect("Cannot create test-output-folder");
        assert!(fs::exists(path).expect("Cannot check folder existance"));
    }

    #[test]
    fn should_delete_single_folder() {
        let path = Path::new("./test-folder");
        execute_delete_dir(path).expect("Cannot delete test-output-folder");
        assert!(!fs::exists(path).expect("Cannot check if folder is still there"));
    }
}
