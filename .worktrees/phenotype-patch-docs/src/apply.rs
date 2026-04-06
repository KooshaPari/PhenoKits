//! Apply unified diffs to content.

#[allow(unused_imports)]
use crate::parse::{parse_diff, Hunk};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApplyError {
    #[error("Failed to apply hunk at line {line}: {message}")]
    HunkFailed { line: usize, message: String },
    #[error("File not found: {0}")]
    FileNotFound(String),
    #[error("Context mismatch at line {line}: expected {expected}, got {actual}")]
    ContextMismatch { line: usize, expected: String, actual: String },
}

/// Apply a unified diff to content.
pub fn apply_patch(content: &str, diff: &str) -> Result<String, ApplyError> {
    let hunks =
        parse_diff(diff).map_err(|e| ApplyError::HunkFailed { line: 0, message: e.to_string() })?;

    let lines: Vec<&str> = content.lines().collect();
    let mut result = Vec::new();
    let mut pos = 0;

    for hunk in hunks {
        let end = (hunk.old_start.saturating_sub(1)).min(lines.len());
        while pos < end {
            result.push(lines[pos].to_string());
            pos += 1;
        }

        for line in &hunk.lines {
            if let Some(stripped) = line.strip_prefix('+') {
                // Addition - add the new content
                result.push(stripped.to_string());
            } else if let Some(context) = line.strip_prefix(' ') {
                // Context line - verify and copy
                if pos < lines.len() {
                    if context != lines[pos] {
                        return Err(ApplyError::ContextMismatch {
                            line: pos + 1,
                            expected: context.to_string(),
                            actual: lines[pos].to_string(),
                        });
                    }
                    result.push(context.to_string());
                    pos += 1;
                }
            } else if let Some(removed) = line.strip_prefix('-') {
                // Deletion - verify and skip
                if pos < lines.len() {
                    if removed != lines[pos] {
                        return Err(ApplyError::ContextMismatch {
                            line: pos + 1,
                            expected: removed.to_string(),
                            actual: lines[pos].to_string(),
                        });
                    }
                    pos += 1;
                }
            }
        }
    }

    while pos < lines.len() {
        result.push(lines[pos].to_string());
        pos += 1;
    }

    Ok(result.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_simple() {
        let content = "line1\nold\nline3\n";
        let diff = "@@ -1,3 +1,3 @@\n line1\n-old\n+new\n line3\n";
        let result = apply_patch(content, diff).unwrap();
        assert!(result.contains("new"));
        assert!(!result.contains("old"));
    }
}
