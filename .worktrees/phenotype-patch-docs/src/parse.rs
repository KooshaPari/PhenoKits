//! Unified diff parsing.

use thiserror::Error;

#[derive(Debug, Clone, PartialEq)]
pub struct Hunk {
    pub old_start: usize,
    pub new_start: usize,
    pub old_lines: usize,
    pub new_lines: usize,
    pub lines: Vec<String>,
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Invalid hunk header: {0}")]
    InvalidHeader(String),
    #[error("Unexpected end of input")]
    UnexpectedEof,
    #[error("Missing context line: {0}")]
    MissingContext(String),
}

/// Parse a unified diff string into hunks.
pub fn parse_diff(diff: &str) -> Result<Vec<Hunk>, ParseError> {
    let mut hunks = Vec::new();
    let lines = diff.lines().peekable();
    let mut current_hunk: Option<Hunk> = None;

    for line in lines {
        if line.starts_with("@@") {
            if let Some(h) = current_hunk.take() {
                hunks.push(h);
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 3 {
                return Err(ParseError::InvalidHeader(line.to_string()));
            }

            let old_part = parts[1].trim_start_matches('-');
            let new_part = parts[2].trim_start_matches('+');

            let (old_start, old_lines) = parse_range(old_part)?;
            let (new_start, new_lines) = parse_range(new_part)?;

            current_hunk =
                Some(Hunk { old_start, new_start, old_lines, new_lines, lines: Vec::new() });
        } else if let Some(ref mut hunk) = current_hunk {
            hunk.lines.push(line.to_string());
        }
    }

    if let Some(h) = current_hunk {
        hunks.push(h);
    }

    Ok(hunks)
}

fn parse_range(s: &str) -> Result<(usize, usize), ParseError> {
    let parts: Vec<&str> = s.split(',').collect();
    match parts.len() {
        1 => {
            let start: usize = parts[0].parse().unwrap_or(1);
            Ok((start, 1))
        }
        2 => {
            let start: usize = parts[0].parse().unwrap_or(1);
            let count: usize = parts[1].parse().unwrap_or(1);
            Ok((start, count))
        }
        _ => Err(ParseError::InvalidHeader(s.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_diff() {
        let diff = r#"--- a/file.txt
+++ b/file.txt
@@ -1,3 +1,4 @@
 line1
-old
+new
 line3
+added
@@ -10,2 +12,1 @@
 context
-old2
+new2
"#;
        let hunks = parse_diff(diff).unwrap();
        assert_eq!(hunks.len(), 2);
        assert_eq!(hunks[0].old_start, 1);
        assert_eq!(hunks[0].new_start, 1);
    }
}
