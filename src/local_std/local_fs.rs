

#[cfg(test)]
mod test {
    use std::{
        fs::{self, Permissions},
        path::Path,
        time::SystemTime,
    };

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

    #[test]
    fn should_create_every_dir() {
        let path = Path::new("./parent-dir/child-dir");
        fs::create_dir_all(path).expect("Cannot create every dir");
        assert!(fs::exists(path).unwrap());
        fs::remove_dir_all(path).unwrap();
    }

    #[test]
    fn file_metadata_test() {
        let path = Path::new("./another-dir/another-child-dir/the-current-dir");
        fs::create_dir_all(path).expect("Cannot create every dir");

        let file_path = path.join("test-file.txt");
        fs::write(file_path.as_path(), b"This is the test file").unwrap();

        assert_eq!(
            "This is the test file".to_string(),
            fs::read_to_string(&file_path).unwrap()
        );

        let file_data = fs::metadata(&file_path).unwrap();

        let file_length = file_data.len();
        let file_created_date: SystemTime = file_data.created().unwrap();
        let file_moodified_date: SystemTime = file_data.modified().unwrap();
        let is_file_data_a_dir: bool = file_data.is_dir();
        let is_file_data_a_file: bool = file_data.is_file();
        let file_data_permissions: Permissions = file_data.permissions();

        println!("len: {}", file_data.len());
        println!("is_file: {}", file_data.is_file());
        println!("is_dir: {}", file_data.is_dir());
        println!("is_symlink: {}", file_data.is_symlink());
        println!("created: {:?}", file_data.created());
        println!("modified: {:?}", file_data.modified());
        println!("accessed: {:?}", file_data.accessed());
        println!("file_type: {:?}", file_data.file_type());
        println!("permissions: {:?}", file_data.permissions());
        println!("readonly: {}", file_data.permissions().readonly());

        assert!(file_data.is_file(), "Should be a file");
        assert!(!file_data.is_dir(), "Should not be a directory");
        assert_eq!(file_data.len(), 21, "File length should be 21");
        assert!(file_data.created().is_ok(), "Should have created time");
        assert!(file_data.modified().is_ok(), "Should have modified time");
        assert!(file_data.accessed().is_ok(), "Should have accessed time");

        assert!(
            !file_data.permissions().readonly(),
            "Should not be readonly"
        );
        assert!(
            file_data.file_type().is_file(),
            "file_type should be a file"
        );
    }
}
