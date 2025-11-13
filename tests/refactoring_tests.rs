/// Tests for the refactored modules

#[cfg(test)]
mod tests {
    use sonik::utils::human::{human_size, SyncStats};

    #[test]
    fn test_format_bytes() {
        assert_eq!(human_size(0), "0 B");
        assert_eq!(human_size(500), "500 B");
        assert_eq!(human_size(1024), "1.0 KB");
        assert_eq!(human_size(1536), "1.5 KB");
        assert_eq!(human_size(1048576), "1.0 MB");
        assert_eq!(human_size(1073741824), "1.0 GB");
    }

    #[test]
    fn test_sync_stats_creation() {
        let stats = SyncStats::new(10, 5, 1024000, 512000);
        
        assert_eq!(stats.upload_count, 10);
        assert_eq!(stats.delete_count, 5);
        assert_eq!(stats.upload_bytes, 1024000);
        assert_eq!(stats.delete_bytes, 512000);
    }

    #[test]
    fn test_sync_stats_has_changes() {
        let empty = SyncStats::default();
        assert!(!empty.has_changes());

        let with_upload = SyncStats::new(1, 0, 100, 0);
        assert!(with_upload.has_changes());

        let with_delete = SyncStats::new(0, 1, 0, 100);
        assert!(with_delete.has_changes());

        let with_both = SyncStats::new(5, 3, 1000, 500);
        assert!(with_both.has_changes());
    }

    #[test]
    fn test_sync_stats_format_summary() {
        let stats = SyncStats::new(5, 2, 5000000, 1000000);
        let summary = stats.format_summary();
        
        assert!(summary.contains("5 upload"));
        assert!(summary.contains("2 delete"));
        assert!(summary.contains("4.8 MB"));
        assert!(summary.contains("976.6 KB"));
    }

    #[test]
    fn test_sync_stats_zero_operations() {
        let stats = SyncStats::new(0, 0, 0, 0);
        let summary = stats.format_summary();
        
        assert!(summary.contains("0 upload"));
        assert!(summary.contains("0 delete"));
        assert!(summary.contains("0 B"));
    }
}
