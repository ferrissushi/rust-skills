#[cfg(test)]
mod test {
    use std::{fs, path::Path};

    #[test]
    fn should_create_single_dir() {
        let path = Path::new("./first-dir-test");
        fs::create_dir(path).expect("Cannot create test-output-folder");
        assert!(fs::exists(path).expect("Cannot check folder existance"));
        fs::remove_dir(path).expect("Cannot delete dir");
    }

    #[test]
    fn should_delete_single_folder() {
        let path = Path::new("./second-dir-test");
        fs::create_dir(path).unwrap();
        fs::remove_dir(path).expect("Cannot delete second-test-dir");
        assert!(!fs::exists(path).expect("Cannot check if folder is still there"));
    }

    #[test]
    fn should_create_test_file() {
        let path = Path::new("./test.txt");
        fs::write(path, "Hello world").expect("");
        assert!(fs::exists(path).expect("Cannot check if file is there"));
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn should_read_file_content() {
        let path = Path::new("./test.txt");
        fs::write(path, "Hello world").expect("");
        let path = Path::new("./test.txt");
        let expected_file_content: String = "Hello world".to_string();
        let actual_file_content: String =
            fs::read_to_string(path).expect("Cannot read file content");
        assert_eq!(expected_file_content, actual_file_content);
    }

    #[test]
    fn should_delete_the_test_file() {
        let path = Path::new("./test.txt");
        fs::write(path, "").unwrap();
        fs::remove_file(path).expect("Cannot remove file");
        assert!(!fs::exists(path).expect("Cannot check if file is still there"));
    }
}
