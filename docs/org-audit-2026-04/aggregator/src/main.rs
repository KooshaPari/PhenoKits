use anyhow::Result;
use colored::Colorize;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Debug, Clone)]
struct RepoAudit {
    repo: String,
    build: String,
    tests: String,
    ci: String,
    docs: String,
    debt: String,
    fr_trace: String,
    velocity: String,
    governance: String,
    deps: String,
    honest: String,
    status: String,
}

impl Default for RepoAudit {
    fn default() -> Self {
        Self {
            repo: String::new(),
            build: "UNKNOWN".to_string(),
            tests: "UNKNOWN".to_string(),
            ci: "UNKNOWN".to_string(),
            docs: "UNKNOWN".to_string(),
            debt: "UNKNOWN".to_string(),
            fr_trace: "UNKNOWN".to_string(),
            velocity: "UNKNOWN".to_string(),
            governance: "UNKNOWN".to_string(),
            deps: "UNKNOWN".to_string(),
            honest: "UNKNOWN".to_string(),
            status: "UNKNOWN".to_string(),
        }
    }
}

fn extract_status(text: &str) -> String {
    let text_lower = text.to_lowercase();
    if text_lower.contains("shipped") {
        "SHIPPED".to_string()
    } else if text_lower.contains("scaffold") {
        "SCAFFOLD".to_string()
    } else if text_lower.contains("broken") {
        "BROKEN".to_string()
    } else {
        "UNKNOWN".to_string()
    }
}

fn extract_dimension_status(content: &str, dimension: &str) -> String {
    let pattern = format!(
        r"##\s+\d+\.\s+\*\*{}\*\*:?(.+?)(?=##\s+\d+\.\s+\*\*|$)",
        regex::escape(dimension)
    );
    let re = Regex::new(&pattern).unwrap_or_else(|_| {
        Regex::new(&format!(r"{}(.+?)(?=##|$)", regex::escape(dimension)))
            .unwrap_or_else(|_| Regex::new(r".*").unwrap())
    });

    if let Some(caps) = re.captures(content) {
        if let Some(m) = caps.get(1) {
            return extract_status(m.as_str());
        }
    }
    "UNKNOWN".to_string()
}

fn parse_audit_file(path: &PathBuf) -> Result<RepoAudit> {
    let content = fs::read_to_string(path)?;
    let repo_name = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .to_string();

    let mut audit = RepoAudit {
        repo: repo_name,
        ..Default::default()
    };

    audit.build = extract_dimension_status(&content, "Build & Compilation");
    audit.tests = extract_dimension_status(&content, "Test Coverage");
    audit.ci = extract_dimension_status(&content, "CI/CD Pipeline");
    audit.docs = extract_dimension_status(&content, "Documentation");
    audit.debt = extract_dimension_status(&content, "Architectural Debt");
    audit.fr_trace = extract_dimension_status(&content, "FR Traceability");
    audit.velocity = extract_dimension_status(&content, "Velocity & Release");
    audit.governance = extract_dimension_status(&content, "Governance");
    audit.deps = extract_dimension_status(&content, "Dependencies");
    audit.honest = extract_dimension_status(&content, "Honest Assessment");

    audit.status = extract_status(&content);

    Ok(audit)
}

fn status_to_emoji(status: &str) -> &'static str {
    match status {
        "SHIPPED" => "🟢",
        "SCAFFOLD" => "🟡",
        "BROKEN" => "🔴",
        _ => "⚪",
    }
}

fn main() -> Result<()> {
    let audit_dir = PathBuf::from(".");

    let mut audits: Vec<RepoAudit> = Vec::new();
    let systemic_issues: HashMap<String, Vec<String>> = HashMap::new();

    for entry in WalkDir::new(&audit_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|ext| ext == "md").unwrap_or(false))
        .filter(|e| e.file_name() != "README.md" && e.file_name() != "INDEX.md")
    {
        let path = entry.path();
        match parse_audit_file(&path.to_path_buf()) {
            Ok(audit) => audits.push(audit),
            Err(e) => eprintln!("Error parsing {:?}: {}", path, e),
        }
    }

    audits.sort_by(|a, b| a.repo.cmp(&b.repo));

    let mut index = String::from(
        "# Org Audit 2026-04 — INDEX\n\n\
         Generated from audit reports in `docs/org-audit-2026-04/`.\n\n\
         ## Status Matrix\n\n\
         | Repo | Build | Tests | CI | Docs | Debt | FR | Velocity | Gov | Deps | Honest | Status |\n\
         |------|-------|-------|----|----|------|----|----|----|----|--------|--------|\n",
    );

    for audit in &audits {
        let row = format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | **{}** |\n",
            audit.repo,
            status_to_emoji(&audit.build),
            status_to_emoji(&audit.tests),
            status_to_emoji(&audit.ci),
            status_to_emoji(&audit.docs),
            status_to_emoji(&audit.debt),
            status_to_emoji(&audit.fr_trace),
            status_to_emoji(&audit.velocity),
            status_to_emoji(&audit.governance),
            status_to_emoji(&audit.deps),
            status_to_emoji(&audit.honest),
            audit.status
        );
        index.push_str(&row);
    }

    index.push_str("\n## Legend\n\n");
    index.push_str("- 🟢 **SHIPPED**: Production-ready, well-maintained\n");
    index.push_str("- 🟡 **SCAFFOLD**: Partial implementation, under development\n");
    index.push_str("- 🔴 **BROKEN**: Non-functional or critical gaps\n");
    index.push_str("- ⚪ **UNKNOWN**: Insufficient data\n\n");

    if !systemic_issues.is_empty() {
        index.push_str("## Systemic Issues\n\n");
        for (issue, repos) in systemic_issues {
            index.push_str(&format!(
                "- **{}**: Affects {} repo(s): {}\n",
                issue,
                repos.len(),
                repos.join(", ")
            ));
        }
    } else {
        index.push_str("## Systemic Issues\n\n_No systemic issues identified yet. More audits needed._\n\n");
    }

    index.push_str("## Summary\n\n");
    let total = audits.len();
    let shipped = audits.iter().filter(|a| a.status == "SHIPPED").count();
    let scaffold = audits.iter().filter(|a| a.status == "SCAFFOLD").count();
    let broken = audits.iter().filter(|a| a.status == "BROKEN").count();

    index.push_str(&format!(
        "- **Total Audited**: {}\n\
         - **SHIPPED**: {} ({:.1}%)\n\
         - **SCAFFOLD**: {} ({:.1}%)\n\
         - **BROKEN**: {} ({:.1}%)\n",
        total,
        shipped,
        (shipped as f64 / total as f64) * 100.0,
        scaffold,
        (scaffold as f64 / total as f64) * 100.0,
        broken,
        (broken as f64 / total as f64) * 100.0
    ));

    fs::write("INDEX.md", &index)?;
    println!("{}", "✓ INDEX.md generated".green());
    println!("  {} repos audited", audits.len());
    println!("  {} SHIPPED, {} SCAFFOLD, {} BROKEN", shipped, scaffold, broken);

    Ok(())
}
