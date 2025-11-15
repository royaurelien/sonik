// SPDX-License-Identifier: MIT
// tests/core_tests.rs

//! Unit tests for core modules (scanner, diff, index)

use sonik::core::{diff, index::IndexedFile, scanner};
use std::fs;
use tempfile::TempDir;

#[cfg(test)]
mod scanner_tests {
    use super::*;

    #[test]
    fn test_scan_empty_directory() {
        let temp = TempDir::new().unwrap();
        let result = scanner::scan_local(temp.path());

        assert!(result.is_ok(), "Scanning empty directory should succeed");
        let files = result.unwrap();
        assert_eq!(files.len(), 0, "Empty directory should have no files");
    }

    #[test]
    fn test_scan_single_file() {
        let temp = TempDir::new().unwrap();
        let file_path = temp.path().join("test.txt");
        fs::write(&file_path, "test content").unwrap();

        let result = scanner::scan_local(temp.path());
        assert!(result.is_ok());
        let files = result.unwrap();

        assert_eq!(files.len(), 1, "Should find one file");
        assert_eq!(files[0].path, "test.txt");
        assert_eq!(files[0].size, 12); // "test content" = 12 bytes
        assert!(files[0].mtime > 0, "Should have valid mtime");
    }

    #[test]
    fn test_scan_multiple_files() {
        let temp = TempDir::new().unwrap();
        fs::write(temp.path().join("file1.txt"), "content1").unwrap();
        fs::write(temp.path().join("file2.txt"), "content2").unwrap();
        fs::write(temp.path().join("file3.txt"), "content3").unwrap();

        let result = scanner::scan_local(temp.path());
        assert!(result.is_ok());
        let files = result.unwrap();

        assert_eq!(files.len(), 3, "Should find three files");
    }

    #[test]
    fn test_scan_nested_directories() {
        let temp = TempDir::new().unwrap();
        let subdir = temp.path().join("subdir");
        fs::create_dir(&subdir).unwrap();
        fs::write(temp.path().join("root.txt"), "root").unwrap();
        fs::write(subdir.join("nested.txt"), "nested").unwrap();

        let result = scanner::scan_local(temp.path());
        assert!(result.is_ok());
        let files = result.unwrap();

        assert_eq!(files.len(), 2, "Should find files in root and subdirectory");
        
        let paths: Vec<&str> = files.iter().map(|f| f.path.as_str()).collect();
        assert!(paths.contains(&"root.txt"));
        assert!(paths.contains(&"subdir/nested.txt") || paths.contains(&"subdir\\nested.txt"));
    }

    #[test]
    fn test_scan_ignores_directories() {
        let temp = TempDir::new().unwrap();
        fs::create_dir(temp.path().join("emptydir")).unwrap();
        fs::write(temp.path().join("file.txt"), "content").unwrap();

        let result = scanner::scan_local(temp.path());
        assert!(result.is_ok());
        let files = result.unwrap();

        // Should only find the file, not the empty directory
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].path, "file.txt");
    }
}

#[cfg(test)]
mod diff_tests {
    use super::*;

    fn create_file(path: &str, size: u64, mtime: i64) -> IndexedFile {
        IndexedFile {
            path: path.to_string(),
            size,
            mtime,
        }
    }

    #[test]
    fn test_diff_empty_to_empty() {
        let local: Vec<IndexedFile> = vec![];
        let previous: Vec<IndexedFile> = vec![];

        let result = diff::compute_diff(&local, &previous);
        assert_eq!(result.to_upload.len(), 0, "No uploads for empty diff");
        assert_eq!(result.to_delete.len(), 0, "No deletions for empty diff");
    }

    #[test]
    fn test_diff_new_files() {
        let local = vec![
            create_file("file1.txt", 100, 1000),
            create_file("file2.txt", 200, 2000),
        ];
        let previous: Vec<IndexedFile> = vec![];

        let result = diff::compute_diff(&local, &previous);
        assert_eq!(result.to_upload.len(), 2, "All files should be uploaded");
        assert_eq!(result.to_delete.len(), 0, "No deletions");
    }

    #[test]
    fn test_diff_deleted_files() {
        let local: Vec<IndexedFile> = vec![];
        let previous = vec![
            create_file("old1.txt", 100, 1000),
            create_file("old2.txt", 200, 2000),
        ];

        let result = diff::compute_diff(&local, &previous);
        assert_eq!(result.to_upload.len(), 0, "No uploads");
        assert_eq!(result.to_delete.len(), 2, "All previous files should be deleted");
        assert!(result.to_delete.contains(&"old1.txt".to_string()));
        assert!(result.to_delete.contains(&"old2.txt".to_string()));
    }

    #[test]
    fn test_diff_unchanged_files() {
        let file = create_file("unchanged.txt", 100, 1000);
        let local = vec![file.clone()];
        let previous = vec![file];

        let result = diff::compute_diff(&local, &previous);
        assert_eq!(result.to_upload.len(), 0, "Unchanged files should not be uploaded");
        assert_eq!(result.to_delete.len(), 0, "Unchanged files should not be deleted");
    }

    #[test]
    fn test_diff_modified_size() {
        let local = vec![create_file("modified.txt", 200, 1000)];
        let previous = vec![create_file("modified.txt", 100, 1000)];

        let result = diff::compute_diff(&local, &previous);
        assert_eq!(result.to_upload.len(), 1, "Size change should trigger upload");
        assert_eq!(result.to_delete.len(), 0);
        assert_eq!(result.to_upload[0].size, 200);
    }

    #[test]
    fn test_diff_modified_mtime() {
        let local = vec![create_file("modified.txt", 100, 2000)];
        let previous = vec![create_file("modified.txt", 100, 1000)];

        let result = diff::compute_diff(&local, &previous);
        assert_eq!(result.to_upload.len(), 1, "Mtime change should trigger upload");
        assert_eq!(result.to_delete.len(), 0);
        assert_eq!(result.to_upload[0].mtime, 2000);
    }

    #[test]
    fn test_diff_complex_scenario() {
        let local = vec![
            create_file("unchanged.txt", 100, 1000),
            create_file("modified.txt", 200, 2000),
            create_file("new.txt", 300, 3000),
        ];

        let previous = vec![
            create_file("unchanged.txt", 100, 1000),
            create_file("modified.txt", 100, 1000),
            create_file("deleted.txt", 150, 1500),
        ];

        let result = diff::compute_diff(&local, &previous);

        // Should upload: modified.txt (changed) + new.txt
        assert_eq!(result.to_upload.len(), 2);
        let upload_paths: Vec<&str> = result.to_upload.iter().map(|f| f.path.as_str()).collect();
        assert!(upload_paths.contains(&"modified.txt"));
        assert!(upload_paths.contains(&"new.txt"));

        // Should delete: deleted.txt
        assert_eq!(result.to_delete.len(), 1);
        assert!(result.to_delete.contains(&"deleted.txt".to_string()));
    }
}

#[cfg(test)]
mod index_tests {
    use super::*;
    use sonik::core::index::Index;

    #[test]
    fn test_index_save_and_load() {
        let temp = TempDir::new().unwrap();
        let index_path = temp.path().join("test_index.bin");

        let files = vec![
            IndexedFile {
                path: "file1.txt".to_string(),
                size: 100,
                mtime: 1000,
            },
            IndexedFile {
                path: "file2.txt".to_string(),
                size: 200,
                mtime: 2000,
            },
        ];

        // Create and save index
        let mut index = Index::load(&index_path).unwrap();
        index.files = files;
        let save_result = index.save();
        assert!(save_result.is_ok(), "Saving index should succeed");
        assert!(index_path.exists(), "Index file should be created");

        // Load index
        let loaded_index = Index::load(&index_path).unwrap();
        let loaded = &loaded_index.files;

        assert_eq!(loaded.len(), 2, "Should load all files");
        assert_eq!(loaded[0].path, "file1.txt");
        assert_eq!(loaded[0].size, 100);
        assert_eq!(loaded[1].path, "file2.txt");
        assert_eq!(loaded[1].size, 200);
    }

    #[test]
    fn test_load_nonexistent_index() {
        let temp = TempDir::new().unwrap();
        let index_path = temp.path().join("nonexistent.bin");

        let index = Index::load(&index_path).unwrap();
        assert_eq!(index.files.len(), 0, "Should return empty vector");
    }

    #[test]
    fn test_index_empty_list() {
        let temp = TempDir::new().unwrap();
        let index_path = temp.path().join("empty_index.bin");

        let mut index = Index::load(&index_path).unwrap();
        index.files = vec![];
        let save_result = index.save();
        assert!(save_result.is_ok());

        let loaded_index = Index::load(&index_path).unwrap();
        assert_eq!(loaded_index.files.len(), 0);
    }

    #[test]
    fn test_index_with_special_characters() {
        let temp = TempDir::new().unwrap();
        let index_path = temp.path().join("special_index.bin");

        let files = vec![
            IndexedFile {
                path: "path/with/slashes.txt".to_string(),
                size: 100,
                mtime: 1000,
            },
            IndexedFile {
                path: "name with spaces.txt".to_string(),
                size: 200,
                mtime: 2000,
            },
        ];

        let mut index = Index::load(&index_path).unwrap();
        index.files = files;
        index.save().unwrap();
        
        let loaded_index = Index::load(&index_path).unwrap();
        let loaded = &loaded_index.files;

        assert_eq!(loaded.len(), 2);
        assert_eq!(loaded[0].path, "path/with/slashes.txt");
        assert_eq!(loaded[1].path, "name with spaces.txt");
    }
}
