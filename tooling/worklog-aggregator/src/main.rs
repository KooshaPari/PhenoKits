use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use regex::Regex;

#[derive(Debug, Clone)]
struct WorklogEntry {
    repo_path: String,
    file_path: String,
    last_date: String,
    category: String,
    snippet: String,
    is_template: bool,
}

fn extract_date(content: &str) -> String {
    // Try to find a date in ISO 8601 format (YYYY-MM-DD)
    if let Some(cap) = Regex::new(r"\d{4}-\d{2}-\d{2}")
        .unwrap()
        .find(content)
    {
        return cap.as_str().to_string();
    }

    // Try to extract from "Generated:" lines
    if let Some(cap) = Regex::new(r"Generated:\s*(\d{4}-\d{2}-\d{2})")
        .unwrap()
        .captures(content)
    {
        if let Some(date) = cap.get(1) {
            return date.as_str().to_string();
        }
    }

    "unknown".to_string()
}

fn extract_category(content: &str) -> String {
    // Check for explicit category markers
    let categories = vec![
        ("ARCHITECTURE", "ARCHITECTURE"),
        ("DUPLICATION", "DUPLICATION"),
        ("DEPENDENCIES", "DEPENDENCIES"),
        ("INTEGRATION", "INTEGRATION"),
        ("PERFORMANCE", "PERFORMANCE"),
        ("RESEARCH", "RESEARCH"),
        ("GOVERNANCE", "GOVERNANCE"),
    ];

    for (marker, cat) in categories {
        if content.contains(marker) {
            return cat.to_string();
        }
    }

    "GENERAL".to_string()
}

fn extract_snippet(content: &str) -> String {
    // Get first ~80 characters of meaningful content after headers
    let lines: Vec<&str> = content
        .lines()
        .filter(|l| !l.trim().is_empty() && !l.starts_with('#') && !l.starts_with('>'))
        .collect();

    if let Some(first_line) = lines.first() {
        let snippet = first_line
            .trim()
            .chars()
            .take(80)
            .collect::<String>();
        if snippet.len() == 80 && !first_line.ends_with('.') {
            return format!("{}…", snippet);
        }
        snippet
    } else {
        "(empty)".to_string()
    }
}

fn is_template_only(content: &str) -> bool {
    let lines = content.lines().count();
    // Templates are typically very short with just headers and empty sections
    lines < 20 && content.matches("##").count() > 3
}

fn collect_worklogs(root: &Path) -> anyhow::Result<Vec<WorklogEntry>> {
    let mut entries = Vec::new();

    for entry in WalkDir::new(root)
        .max_depth(4)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            let path = e.path();
            // Skip .archive and .worktrees
            let path_str = path.to_string_lossy();
            if path_str.contains(".archive") || path_str.contains(".worktrees") {
                return false;
            }

            // Match worklog.md files or docs/worklogs/*.md
            if e.file_name().to_string_lossy() == "worklog.md" {
                return true;
            }

            let is_in_worklogs_dir = path
                .parent()
                .and_then(|p| p.file_name())
                .map(|n| n.to_string_lossy() == "worklogs")
                .unwrap_or(false);

            is_in_worklogs_dir && path.extension().map(|e| e == "md").unwrap_or(false)
        })
    {
        let path = entry.path();
        if let Ok(content) = fs::read_to_string(path) {
            // Extract repo path: repos/<repo-name>/<rest>
            let relative = path.strip_prefix(root).unwrap_or(path);
            let repo_name = relative
                .components()
                .next()
                .map(|c| c.as_os_str().to_string_lossy().to_string())
                .unwrap_or_else(|| "repos".to_string());

            let file_name = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("worklog.md");

            entries.push(WorklogEntry {
                repo_path: repo_name,
                file_path: relative.to_string_lossy().to_string(),
                last_date: extract_date(&content),
                category: extract_category(&content),
                snippet: extract_snippet(&content),
                is_template: is_template_only(&content),
            });
        }
    }

    // Sort by date descending
    entries.sort_by(|a, b| b.last_date.cmp(&a.last_date));

    Ok(entries)
}

fn generate_index(entries: &[WorklogEntry]) -> String {
    let mut output = String::from(
        r#"# Worklog Index — Phenotype Organization

> Auto-generated from cross-repo worklog aggregation
> Run `tooling/worklog-aggregator/target/release/worklog-aggregator` to regenerate

## Quick Navigation

| Repos Indexed | Categories | Empty Templates | Last Updated |
|---|---|---|---|
"#,
    );

    let total_repos = entries.iter().map(|e| &e.repo_path).collect::<std::collections::HashSet<_>>().len();
    let total_categories = entries.iter().map(|e| &e.category).collect::<std::collections::HashSet<_>>().len();
    let empty_count = entries.iter().filter(|e| e.is_template).count();
    let default_date = "N/A".to_string();
    let most_recent = entries.first().map(|e| &e.last_date).unwrap_or(&default_date);

    output.push_str(&format!(
        "| {} | {} | {} | {} |\n\n",
        total_repos, total_categories, empty_count, most_recent
    ));

    output.push_str("## All Worklogs (Most Recent First)\n\n");
    output.push_str("| Repo | File | Date | Category | Status | Snippet |\n");
    output.push_str("|------|------|------|----------|--------|----------|\n");

    for entry in entries {
        let status = if entry.is_template { "⚠️ Template" } else { "✓ Active" };
        output.push_str(&format!(
            "| `{}` | `{}` | {} | **{}** | {} | {} |\n",
            entry.repo_path,
            entry.file_path.split('/').last().unwrap_or(""),
            entry.last_date,
            entry.category,
            status,
            entry.snippet
        ));
    }

    output.push_str("\n## By Category\n\n");

    // Group by category
    let mut categories: std::collections::BTreeMap<&str, Vec<_>> = std::collections::BTreeMap::new();
    for entry in entries {
        categories
            .entry(&entry.category)
            .or_insert_with(Vec::new)
            .push(entry);
    }

    for (cat, cats) in categories {
        output.push_str(&format!("### {} ({})\n\n", cat, cats.len()));
        for entry in cats {
            let status = if entry.is_template { "template" } else { "active" };
            output.push_str(&format!(
                "- [`{}`]({}) — {} — *{}*\n",
                entry.repo_path, entry.file_path, entry.last_date, status
            ));
        }
        output.push('\n');
    }

    output
}

fn main() -> anyhow::Result<()> {
    let root = Path::new("/Users/kooshapari/CodeProjects/Phenotype/repos");
    let entries = collect_worklogs(root)?;

    let index = generate_index(&entries);
    fs::write(
        "/Users/kooshapari/CodeProjects/Phenotype/repos/worklogs/INDEX.md",
        index,
    )?;

    println!("✓ Indexed {} worklogs from {} repos", entries.len(),
        entries.iter().map(|e| &e.repo_path).collect::<std::collections::HashSet<_>>().len());
    println!("✓ Wrote worklogs/INDEX.md");

    Ok(())
}
