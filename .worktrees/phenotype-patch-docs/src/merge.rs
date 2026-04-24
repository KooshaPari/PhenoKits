//! Merge conflicting diffs.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MergeChoice {
    Ours,
    Theirs,
    Both,
}

#[derive(Debug, Clone)]
pub struct Conflict {
    pub ours: String,
    pub theirs: String,
}

#[derive(Debug, Clone)]
pub struct MergeSuccess {
    pub merged: String,
    pub has_conflicts: bool,
}

/// Merge multiple patches, handling conflicts.
pub fn merge_patches(base: &str, patches: &[(&str, &str)]) -> Result<MergeSuccess, String> {
    let mut result = base.to_string();

    for (_, patch) in patches {
        let has_conflicts =
            patch.contains("<<<<<<<") || patch.contains("=======") || patch.contains(">>>>>>>");

        result.push_str(patch);

        if has_conflicts {
            return Ok(MergeSuccess { merged: result, has_conflicts: true });
        }
    }

    Ok(MergeSuccess { merged: result, has_conflicts: false })
}

impl fmt::Display for MergeChoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MergeChoice::Ours => write!(f, "ours"),
            MergeChoice::Theirs => write!(f, "theirs"),
            MergeChoice::Both => write!(f, "both"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_no_conflicts() {
        let base = "line1\nline2\n";
        let patches = vec![("", "+line3\n")];
        let result = merge_patches(base, &patches).unwrap();
        assert!(!result.has_conflicts);
        assert!(result.merged.contains("line3"));
    }
}
