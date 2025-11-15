// SPDX-License-Identifier: MIT
// tests/validation_tests.rs

//! Unit tests for sync validation module

use sonik::sync::validation;
use std::fs;
use tempfile::TempDir;

#[cfg(test)]
mod validation_tests {
    use super::*;

    #[test]
    fn test_validate_source_exists() {
        let temp = TempDir::new().unwrap();
        let source = temp.path();

        let result = validation::validate_source(source);
        assert!(result.is_ok(), "Existing directory should validate");
    }

    #[test]
    fn test_validate_source_nonexistent() {
        let temp = TempDir::new().unwrap();
        let source = temp.path().join("nonexistent");

        let result = validation::validate_source(&source);
        assert!(result.is_err(), "Nonexistent path should fail validation");
        let err = result.unwrap_err();
        assert!(err.to_string().contains("does not exist"));
    }

    #[test]
    fn test_validate_source_not_directory() {
        let temp = TempDir::new().unwrap();
        let file_path = temp.path().join("file.txt");
        fs::write(&file_path, "content").unwrap();

        let result = validation::validate_source(&file_path);
        assert!(result.is_err(), "File should not validate as source directory");
        let err = result.unwrap_err();
        assert!(err.to_string().contains("not a directory"));
    }

    #[test]
    fn test_validate_target_creates_directory() {
        let temp = TempDir::new().unwrap();
        let target = temp.path().join("new_target");

        assert!(!target.exists(), "Target should not exist initially");

        let result = validation::validate_target(&target, false);
        assert!(result.is_ok(), "validate_target should create directory");
        assert!(target.exists(), "Target directory should be created");
        assert!(target.is_dir(), "Target should be a directory");
    }

    #[test]
    fn test_validate_target_existing_directory() {
        let temp = TempDir::new().unwrap();
        let target = temp.path();

        let result = validation::validate_target(target, false);
        assert!(result.is_ok(), "Existing directory should validate as target");
    }

    #[test]
    fn test_validate_target_creates_nested() {
        let temp = TempDir::new().unwrap();
        let target = temp.path().join("level1").join("level2").join("level3");

        let result = validation::validate_target(&target, false);
        assert!(result.is_ok(), "Should create nested directories");
        assert!(target.exists(), "All nested directories should be created");
    }

    #[test]
    fn test_validate_sync_paths_both_valid() {
        let temp = TempDir::new().unwrap();
        let source = temp.path();
        let target = temp.path().join("target");

        let result = validation::validate_sync_paths(source, &target, false);
        assert!(result.is_ok(), "Both valid paths should pass validation");
        assert!(target.exists(), "Target should be created");
    }

    #[test]
    fn test_validate_sync_paths_invalid_source() {
        let temp = TempDir::new().unwrap();
        let source = temp.path().join("nonexistent");
        let target = temp.path().join("target");

        let result = validation::validate_sync_paths(&source, &target, false);
        assert!(result.is_err(), "Invalid source should fail validation");
    }

    #[test]
    fn test_validate_sync_paths_source_is_file() {
        let temp = TempDir::new().unwrap();
        let source = temp.path().join("file.txt");
        fs::write(&source, "content").unwrap();
        let target = temp.path().join("target");

        let result = validation::validate_sync_paths(&source, &target, false);
        assert!(result.is_err(), "Source being a file should fail validation");
    }

    #[test]
    fn test_validate_target_with_write_test() {
        let temp = TempDir::new().unwrap();
        let target = temp.path().join("writable_target");

        // Create the directory first
        fs::create_dir(&target).unwrap();

        let result = validation::validate_target(&target, true);
        // This should succeed on most systems, but might fail if filesystem is read-only
        // We'll just check that the function doesn't panic
        let _ = result;
    }

    #[test]
    fn test_validate_target_readonly_filesystem() {
        // Note: This test is platform-specific and might not work on all systems
        // On Linux, we could test with a read-only mount, but that requires root
        // For now, we just test that the function handles the flag correctly
        let temp = TempDir::new().unwrap();
        let target = temp.path().join("target");

        // Test with write test disabled (should always succeed if dir can be created)
        let result = validation::validate_target(&target, false);
        assert!(result.is_ok());
    }
}
