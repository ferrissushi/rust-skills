use std::{fs, path::Path};

pub fn execute_create_dir(path: &Path) -> std::io::Result<()> {
    fs::create_dir(path)?;
    Ok(())
}

pub fn execute_delete_dir(path: &Path) -> std::io::Result<()> {
    fs::remove_dir(path)?;
    Ok(())
}

pub fn execute_create_file<T: AsRef<Path>, C: AsRef<[u8]>>(
    path: T,
    content: C,
) -> std::io::Result<()> {
    fs::write(path, content)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use std::{fs, path::Path};

    use crate::local_std::local_fs::{execute_create_dir, execute_create_file, execute_delete_dir};

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

    #[test]
    fn should_create_test_file() {
        let path = Path::new("./test.txt");
        execute_create_file(path, "Hello world").expect("");
        assert!(fs::exists(path).expect("Cannot check if file is there"));
    }

    #[test]
    fn should_read_file_content() {
        let path = Path::new("./test.txt");
        let expected_file_content: String = "Hello world".to_string();
        let actual_file_content: String =
            fs::read_to_string(path).expect("Cannot read file content");
        assert_eq!(expected_file_content, actual_file_content);
    }
}
