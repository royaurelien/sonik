// SPDX-License-Identifier: MIT
// tests/utils_tests.rs

//! Unit tests for utility modules

use sonik::utils::{slug, human, paths};

#[cfg(test)]
mod slug_tests {
    use super::*;

    #[test]
    fn test_unique_slug_deterministic() {
        let slug1 = slug::unique_slug("Music", "Device1");
        let slug2 = slug::unique_slug("Music", "Device1");
        assert_eq!(slug1, slug2, "Same inputs should produce same slug");
    }

    #[test]
    fn test_unique_slug_different_contexts() {
        let slug1 = slug::unique_slug("Music", "Device1");
        let slug2 = slug::unique_slug("Music", "Device2");
        assert_ne!(slug1, slug2, "Different contexts should produce different slugs");
    }

    #[test]
    fn test_unique_slug_case_sensitive_hash() {
        let slug1 = slug::unique_slug("Music", "Device1");
        let slug2 = slug::unique_slug("MUSIC", "Device1");
        // Hash is computed BEFORE normalization, so different cases produce different hashes
        assert_ne!(slug1, slug2, "Hash is case-sensitive in input");
        // But both should have lowercase normalized labels
        assert!(slug1.starts_with("music-"));
        assert!(slug2.starts_with("music-"));
    }

    #[test]
    fn test_unique_slug_format() {
        let slug = slug::unique_slug("My Photos", "Phone");
        assert!(slug.starts_with("my-photos-"), "Should start with normalized label");
        assert_eq!(slug.len(), "my-photos-".len() + 8, "Should have 8-char hash suffix");
    }

    #[test]
    fn test_unique_slug_special_chars() {
        let slug = slug::unique_slug("Music/Videos", "Device");
        assert!(!slug.contains('/'), "Slashes should be replaced");
        assert!(slug.contains('-'), "Should use hyphens");
    }
}

#[cfg(test)]
mod human_tests {
    use super::*;

    #[test]
    fn test_human_size_bytes() {
        assert_eq!(human::human_size(0), "0 B");
        assert_eq!(human::human_size(1), "1 B");
        assert_eq!(human::human_size(512), "512 B");
        assert_eq!(human::human_size(1023), "1023 B");
    }

    #[test]
    fn test_human_size_kilobytes() {
        assert_eq!(human::human_size(1024), "1.0 KB");
        assert_eq!(human::human_size(2048), "2.0 KB");
        assert_eq!(human::human_size(1536), "1.5 KB");
    }

    #[test]
    fn test_human_size_megabytes() {
        assert_eq!(human::human_size(1024 * 1024), "1.0 MB");
        assert_eq!(human::human_size(5 * 1024 * 1024), "5.0 MB");
        assert_eq!(human::human_size(1536 * 1024), "1.5 MB");
    }

    #[test]
    fn test_human_size_gigabytes() {
        assert_eq!(human::human_size(1024 * 1024 * 1024), "1.0 GB");
        assert_eq!(human::human_size(3 * 1024 * 1024 * 1024), "3.0 GB");
    }

    #[test]
    fn test_sync_stats_new() {
        let stats = human::SyncStats::new(5, 3, 1024000, 512000);
        assert_eq!(stats.upload_count, 5);
        assert_eq!(stats.delete_count, 3);
        assert_eq!(stats.upload_bytes, 1024000);
        assert_eq!(stats.delete_bytes, 512000);
    }

    #[test]
    fn test_sync_stats_has_changes() {
        let stats1 = human::SyncStats::new(5, 0, 1024, 0);
        assert!(stats1.has_changes(), "Upload count > 0 means changes");

        let stats2 = human::SyncStats::new(0, 3, 0, 512);
        assert!(stats2.has_changes(), "Delete count > 0 means changes");

        let stats3 = human::SyncStats::new(0, 0, 0, 0);
        assert!(!stats3.has_changes(), "No changes when all zeros");
    }

    #[test]
    fn test_sync_stats_format_summary() {
        let stats = human::SyncStats::new(3, 2, 2048, 1024);
        let summary = stats.format_summary();
        assert!(summary.contains("3 upload(s)"));
        assert!(summary.contains("2 delete(s)"));
        assert!(summary.contains("KB"));
    }

    #[test]
    fn test_sync_stats_default() {
        let stats = human::SyncStats::default();
        assert_eq!(stats.upload_count, 0);
        assert_eq!(stats.delete_count, 0);
        assert_eq!(stats.upload_bytes, 0);
        assert_eq!(stats.delete_bytes, 0);
        assert!(!stats.has_changes());
    }
}

#[cfg(test)]
mod paths_tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_data_dir_exists() {
        let result = paths::data_dir();
        assert!(result.is_ok(), "data_dir should return a valid path");
        let path = result.unwrap();
        assert!(path.is_absolute(), "Should return absolute path");
    }

    #[test]
    fn test_config_dir_exists() {
        let result = paths::config_dir();
        assert!(result.is_ok(), "config_dir should return a valid path");
        let path = result.unwrap();
        assert!(path.is_absolute(), "Should return absolute path");
    }

    #[test]
    fn test_home_dir_exists() {
        let result = paths::home_dir();
        assert!(result.is_ok(), "home_dir should return a valid path");
        let path = result.unwrap();
        assert!(path.is_absolute(), "Should return absolute path");
    }

    #[test]
    fn test_app_data_dir_structure() {
        let result = paths::app_data_dir();
        assert!(result.is_ok(), "app_data_dir should return a valid path");
        let path = result.unwrap();
        assert!(path.to_string_lossy().contains("sonik"), "Should contain 'sonik'");
    }

    #[test]
    fn test_app_config_dir_structure() {
        let result = paths::app_config_dir();
        assert!(result.is_ok(), "app_config_dir should return a valid path");
        let path = result.unwrap();
        assert!(path.to_string_lossy().contains("sonik"), "Should contain 'sonik'");
    }

    #[test]
    fn test_ensure_dir_creates_directory() {
        use tempfile::TempDir;

        let temp = TempDir::new().unwrap();
        let test_path = temp.path().join("test_subdir");
        let path_buf = PathBuf::from(&test_path);

        assert!(!test_path.exists(), "Directory should not exist initially");

        let result = paths::ensure_dir(&path_buf);
        assert!(result.is_ok(), "ensure_dir should succeed");
        assert!(test_path.exists(), "Directory should be created");
        assert!(test_path.is_dir(), "Should be a directory");
    }

    #[test]
    fn test_ensure_dir_existing_directory() {
        use tempfile::TempDir;

        let temp = TempDir::new().unwrap();
        let test_path = temp.path().join("existing");
        std::fs::create_dir(&test_path).unwrap();
        let path_buf = PathBuf::from(&test_path);

        let result = paths::ensure_dir(&path_buf);
        assert!(result.is_ok(), "ensure_dir should succeed on existing dir");
    }

    #[test]
    fn test_ensure_dir_nested_paths() {
        use tempfile::TempDir;

        let temp = TempDir::new().unwrap();
        let nested_path = temp.path().join("level1").join("level2").join("level3");
        let path_buf = PathBuf::from(&nested_path);

        let result = paths::ensure_dir(&path_buf);
        assert!(result.is_ok(), "ensure_dir should create nested directories");
        assert!(nested_path.exists(), "All nested directories should be created");
    }
}
