// SPDX-License-Identifier: MIT
// src/utils/slug.rs

//! Utility to create unique slugs based on labels and contexts.

use blake3;

/// Create a deterministic, collision-resistant slug based on a label and a unique context.
/// Example output: "music-6f03a21c".
///
/// `label`: A human-friendly name (e.g. target folder)
/// `context`: A string that makes the slug unique (e.g. device name)
pub fn unique_slug(label: &str, context: &str) -> String {
    let input = format!("{}::{}", label, context);
    let hash = blake3::hash(input.as_bytes());

    // Use first 8 hex characters for readability and enough uniqueness
    let short = &hash.to_hex()[..8];

    // Keep the label readable and slug-like
    let slug = label
        .to_lowercase()
        .replace(' ', "-")
        .replace('/', "-");

    format!("{}-{}", slug, short)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_slug() {
        let s1 = unique_slug("Music", "Pixel7a");
        let s2 = unique_slug("Music", "Pixel7a");
        assert_eq!(s1, s2, "Same inputs should produce same slug");
        assert!(s1.starts_with("music-"), "Should start with lowercase label");
        assert_eq!(s1.len(), "music-".len() + 8, "Should have 8-char hash");
        
        // Different inputs produce different slugs
        let s3 = unique_slug("Music", "Device2");
        assert_ne!(s1, s3, "Different context should produce different slug");
    }
}
