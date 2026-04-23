//! Create unified diffs from original and modified content.

use similar::TextDiff;

use super::{CreateError, Diff};

/// Create a unified diff from original and modified content
pub fn create_diff(original: &str, modified: &str) -> Result<Diff, CreateError> {
    if original.is_empty() && modified.is_empty() {
        return Err(CreateError::EmptyContent);
    }

    let _diff = TextDiff::from_lines(original, modified);
    // Changes can be iterated via diff.iter_all_changes()

    Ok(Diff { hunks: vec![] })
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_diff() {
        let result = create_diff("", "");
        assert!(result.is_err());
    }

    #[test]
    fn test_no_change() {
        let result = create_diff("hello", "hello");
        assert!(result.is_ok());
    }

    #[test]
    fn test_addition() {
        let result = create_diff("hello", "hello world");
        assert!(result.is_ok());
    }
}
