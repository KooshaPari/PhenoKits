//! Report generation for test results

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::types::{SuiteResult, TestResult, TestStatus};

/// Test report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestReport {
    /// Report ID
    pub id: String,
    /// Report title
    pub title: String,
    /// Report generation time
    pub generated_at: DateTime<Utc>,
    /// Suite results included in report
    pub suites: Vec<SuiteResult>,
    /// Summary statistics
    pub summary: ReportSummary,
    /// Metadata
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub metadata: HashMap<String, String>,
}

/// Report summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportSummary {
    /// Total number of suites
    pub total_suites: usize,
    /// Total number of tests
    pub total_tests: usize,
    /// Number of passed tests
    pub passed: usize,
    /// Number of failed tests
    pub failed: usize,
    /// Number of tests with errors
    pub errors: usize,
    /// Number of skipped tests
    pub skipped: usize,
    /// Number of timed out tests
    pub timeouts: usize,
    /// Overall pass rate (0-100)
    pub pass_rate: f64,
    /// Total duration in milliseconds
    pub total_duration_ms: u64,
}

impl ReportSummary {
    /// Create a summary from a list of suite results
    pub fn from_suites(suites: &[SuiteResult]) -> Self {
        let total_suites = suites.len();
        let total_tests: usize = suites.iter().map(|s| s.total_count()).sum();
        let passed: usize = suites.iter().map(|s| s.passed_count()).sum();
        let failed: usize = suites.iter().map(|s| s.failed_count()).sum();
        let skipped: usize = suites.iter().map(|s| s.skipped_count()).sum();
        let errors: usize = suites
            .iter()
            .map(|s| {
                s.results
                    .iter()
                    .filter(|r| r.status == TestStatus::Error)
                    .count()
            })
            .sum();
        let timeouts: usize = suites
            .iter()
            .map(|s| {
                s.results
                    .iter()
                    .filter(|r| r.status == TestStatus::Timeout)
                    .count()
            })
            .sum();

        let total_duration_ms: u64 = suites
            .iter()
            .map(|s| s.results.iter().map(|r| r.duration_ms).sum::<u64>())
            .sum();

        let pass_rate = if total_tests == 0 {
            0.0
        } else {
            (passed as f64 / total_tests as f64) * 100.0
        };

        Self {
            total_suites,
            total_tests,
            passed,
            failed,
            errors,
            skipped,
            timeouts,
            pass_rate,
            total_duration_ms,
        }
    }
}

/// Report generator
pub struct ReportGenerator;

impl ReportGenerator {
    /// Create a new report generator
    pub fn new() -> Self {
        Self
    }

    /// Generate a report from suite results
    pub fn generate(&self, title: impl Into<String>, suites: Vec<SuiteResult>) -> TestReport {
        let summary = ReportSummary::from_suites(&suites);

        TestReport {
            id: uuid::Uuid::new_v4().to_string(),
            title: title.into(),
            generated_at: Utc::now(),
            suites,
            summary,
            metadata: HashMap::new(),
        }
    }

    /// Generate JSON report
    pub fn to_json(&self, report: &TestReport) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(report)
    }

    /// Generate HTML report
    pub fn to_html(&self, report: &TestReport) -> String {
        let mut html = String::new();

        // HTML header
        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html>\n<head>\n");
        html.push_str(&format!("<title>{}</title>\n", report.title));
        html.push_str("<style>\n");
        html.push_str(include_str!("report_styles.css"));
        html.push_str("</style>\n");
        html.push_str("</head>\n<body>\n");

        // Title and summary
        html.push_str(&format!("<h1>{}</h1>\n", report.title));
        html.push_str(&format!(
            "<p>Generated: {}</p>\n",
            report.generated_at.to_rfc3339()
        ));

        // Summary section
        html.push_str("<div class=\"summary\">\n");
        html.push_str("<h2>Summary</h2>\n");
        html.push_str(&self.summary_to_html(&report.summary));
        html.push_str("</div>\n");

        // Suite details
        html.push_str("<div class=\"suites\">\n");
        html.push_str("<h2>Test Suites</h2>\n");
        for suite in &report.suites {
            html.push_str(&self.suite_to_html(suite));
        }
        html.push_str("</div>\n");

        // Footer
        html.push_str("</body>\n</html>");

        html
    }

    /// Generate Markdown report
    pub fn to_markdown(&self, report: &TestReport) -> String {
        let mut md = String::new();

        md.push_str(&format!("# {}\n\n", report.title));
        md.push_str(&format!(
            "Generated: {}\n\n",
            report.generated_at.to_rfc3339()
        ));

        // Summary
        md.push_str("## Summary\n\n");
        md.push_str(&self.summary_to_markdown(&report.summary));
        md.push('\n');

        // Suites
        md.push_str("## Test Suites\n\n");
        for suite in &report.suites {
            md.push_str(&self.suite_to_markdown(suite));
            md.push('\n');
        }

        md
    }

    /// Generate JUnit XML report
    pub fn to_junit(&self, report: &TestReport) -> String {
        let mut xml = String::new();

        xml.push_str(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
        xml.push('\n');
        xml.push_str(&format!(
            r#"<testsuites name="{}" tests="{}" failures="{}" errors="{}" time="{}" timestamp="{}">"#,
            report.title,
            report.summary.total_tests,
            report.summary.failed,
            report.summary.errors,
            report.summary.total_duration_ms as f64 / 1000.0,
            report.generated_at.to_rfc3339()
        ));
        xml.push('\n');

        for suite in &report.suites {
            xml.push_str(&self.suite_to_junit(suite));
        }

        xml.push_str("</testsuites>\n");
        xml
    }

    // Helper methods for HTML generation
    fn summary_to_html(&self, summary: &ReportSummary) -> String {
        format!(
            r#"
            <table>
                <tr><td>Total Suites</td><td>{}</td></tr>
                <tr><td>Total Tests</td><td>{}</td></tr>
                <tr><td class="passed">Passed</td><td>{}</td></tr>
                <tr><td class="failed">Failed</td><td>{}</td></tr>
                <tr><td class="error">Errors</td><td>{}</td></tr>
                <tr><td class="skipped">Skipped</td><td>{}</td></tr>
                <tr><td class="timeout">Timeouts</td><td>{}</td></tr>
                <tr><td>Pass Rate</td><td>{:.1}%</td></tr>
                <tr><td>Total Duration</td><td>{}ms</td></tr>
            </table>
            "#,
            summary.total_suites,
            summary.total_tests,
            summary.passed,
            summary.failed,
            summary.errors,
            summary.skipped,
            summary.timeouts,
            summary.pass_rate,
            summary.total_duration_ms
        )
    }

    fn suite_to_html(&self, suite: &SuiteResult) -> String {
        let mut html = String::new();

        html.push_str(&format!(
            "<div class=\"suite\">\n<h3>{}</h3>\n",
            suite.suite_name
        ));
        html.push_str(&format!(
            "<p>{} passed, {} failed, {} total</p>\n",
            suite.passed_count(),
            suite.failed_count(),
            suite.total_count()
        ));

        html.push_str("<table class=\"tests\">\n");
        html.push_str("<tr><th>Test</th><th>Status</th><th>Duration</th></tr>\n");

        for test in &suite.results {
            let status_class = match test.status {
                TestStatus::Passed => "passed",
                TestStatus::Failed => "failed",
                TestStatus::Error => "error",
                TestStatus::Skipped => "skipped",
                TestStatus::Timeout => "timeout",
                _ => "",
            };

            html.push_str(&format!(
                r#"<tr class="{}"><td>{}</td><td>{}</td><td>{}ms</td></tr>"#,
                status_class,
                test.test_name,
                test.status.display_name(),
                test.duration_ms
            ));

            if let Some(ref error) = test.error_message {
                html.push_str(&format!(
                    r#"<tr class="{}"><td colspan="3" class="error-message">{}</td></tr>"#,
                    status_class,
                    html_escape(error)
                ));
            }
        }

        html.push_str("</table>\n");
        html.push_str("</div>\n");

        html
    }

    // Helper methods for Markdown generation
    fn summary_to_markdown(&self, summary: &ReportSummary) -> String {
        format!(
            "- **Total Suites:** {}\n\
             - **Total Tests:** {}\n\
             - **Passed:** {}\n\
             - **Failed:** {}\n\
             - **Errors:** {}\n\
             - **Skipped:** {}\n\
             - **Timeouts:** {}\n\
             - **Pass Rate:** {:.1}%\n\
             - **Total Duration:** {}ms",
            summary.total_suites,
            summary.total_tests,
            summary.passed,
            summary.failed,
            summary.errors,
            summary.skipped,
            summary.timeouts,
            summary.pass_rate,
            summary.total_duration_ms
        )
    }

    fn suite_to_markdown(&self, suite: &SuiteResult) -> String {
        let mut md = format!("### {}\n\n", suite.suite_name);

        md.push_str(&format!(
            "- Passed: {}\n\
             - Failed: {}\n\
             - Total: {}\n",
            suite.passed_count(),
            suite.failed_count(),
            suite.total_count()
        ));

        md.push('\n');

        // Test results table
        md.push_str("| Test | Status | Duration |\n");
        md.push_str("|------|--------|----------|\n");

        for test in &suite.results {
            let status_emoji = match test.status {
                TestStatus::Passed => "✅",
                TestStatus::Failed => "❌",
                TestStatus::Error => "⚠️",
                TestStatus::Skipped => "⏭️",
                TestStatus::Timeout => "⏱️",
                _ => "❓",
            };

            md.push_str(&format!(
                "| {} | {} {} | {}ms |\n",
                test.test_name,
                status_emoji,
                test.status.display_name(),
                test.duration_ms
            ));
        }

        md
    }

    // Helper methods for JUnit generation
    fn suite_to_junit(&self, suite: &SuiteResult) -> String {
        let duration_sec = suite.results.iter().map(|r| r.duration_ms).sum::<u64>() as f64 / 1000.0;

        let mut xml = format!(
            r#"<testsuite name="{}" tests="{}" failures="{}" errors="{}" time="{}">"#,
            escape_xml(&suite.suite_name),
            suite.total_count(),
            suite.failed_count(),
            suite
                .results
                .iter()
                .filter(|r| r.status == TestStatus::Error)
                .count(),
            duration_sec
        );
        xml.push('\n');

        for test in &suite.results {
            xml.push_str(&self.test_to_junit(test));
        }

        xml.push_str("</testsuite>\n");
        xml
    }

    fn test_to_junit(&self, test: &TestResult) -> String {
        let duration_sec = test.duration_ms as f64 / 1000.0;

        let mut xml = format!(
            r#"<testcase name="{}" time="{}" classname="{}">"#,
            escape_xml(&test.test_name),
            duration_sec,
            escape_xml(&test.test_id)
        );

        match test.status {
            TestStatus::Failed => {
                xml.push('\n');
                xml.push_str(&format!(
                    r#"<failure message="{}">{}</failure>"#,
                    escape_xml(test.error_message.as_deref().unwrap_or("Test failed")),
                    escape_xml(test.error_message.as_deref().unwrap_or(""))
                ));
                xml.push('\n');
            }
            TestStatus::Error => {
                xml.push('\n');
                xml.push_str(&format!(
                    r#"<error message="{}">{}</error>"#,
                    escape_xml(test.error_message.as_deref().unwrap_or("Test error")),
                    escape_xml(test.error_message.as_deref().unwrap_or(""))
                ));
                xml.push('\n');
            }
            TestStatus::Skipped => {
                xml.push('\n');
                xml.push_str("<skipped/>\n");
            }
            _ => {}
        }

        xml.push_str("</testcase>\n");
        xml
    }
}

impl Default for ReportGenerator {
    fn default() -> Self {
        Self::new()
    }
}

// Utility functions
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

// CSS styles for HTML report
const REPORT_STYLES: &str = r#"
body {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
    max-width: 1200px;
    margin: 0 auto;
    padding: 20px;
    background: #f5f5f5;
}
h1, h2, h3 {
    color: #333;
}
.summary {
    background: white;
    padding: 20px;
    border-radius: 8px;
    margin-bottom: 20px;
}
.suites {
    background: white;
    padding: 20px;
    border-radius: 8px;
}
table {
    width: 100%;
    border-collapse: collapse;
    margin: 10px 0;
}
th, td {
    text-align: left;
    padding: 8px;
    border-bottom: 1px solid #ddd;
}
.passed { color: #28a745; }
.failed { color: #dc3545; }
.error { color: #fd7e14; }
.skipped { color: #6c757d; }
.timeout { color: #ffc107; }
.error-message {
    font-size: 0.9em;
    color: #666;
    padding-left: 20px;
}
.suite {
    margin-bottom: 30px;
    border: 1px solid #ddd;
    border-radius: 8px;
    padding: 15px;
}
"#;

// This makes the CSS available
#[doc(hidden)]
pub const fn get_report_styles() -> &'static str {
    REPORT_STYLES
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::*;

    fn create_test_result(id: &str, name: &str, status: TestStatus) -> TestResult {
        let mut result = TestResult::new(id, name);
        result.status = status;
        result.duration_ms = 100;
        result
    }

    fn create_suite_result(id: &str, name: &str) -> SuiteResult {
        let mut suite = SuiteResult::new(id, name);
        suite.add_result(create_test_result("t1", "Test 1", TestStatus::Passed));
        suite.add_result(create_test_result("t2", "Test 2", TestStatus::Failed));
        suite.add_result(create_test_result("t3", "Test 3", TestStatus::Skipped));
        suite.complete()
    }

    #[test]
    fn test_report_generator_new() {
        let gen = ReportGenerator::new();
        // Just verify it creates
        assert!(true);
    }

    #[test]
    fn test_generate_report() {
        let gen = ReportGenerator::new();
        let suite = create_suite_result("s1", "Suite 1");

        let report = gen.generate("Test Report", vec![suite]);

        assert_eq!(report.title, "Test Report");
        assert_eq!(report.suites.len(), 1);
        assert_eq!(report.summary.total_tests, 3);
    }

    #[test]
    fn test_report_summary_from_suites() {
        let suite1 = create_suite_result("s1", "Suite 1");
        let suite2 = create_suite_result("s2", "Suite 2");

        let summary = ReportSummary::from_suites(&[suite1, suite2]);

        assert_eq!(summary.total_suites, 2);
        assert_eq!(summary.total_tests, 6);
        assert_eq!(summary.passed, 2);
        assert_eq!(summary.failed, 2);
        assert_eq!(summary.skipped, 2);
    }

    #[test]
    fn test_to_json() {
        let gen = ReportGenerator::new();
        let suite = create_suite_result("s1", "Suite 1");
        let report = gen.generate("Test Report", vec![suite]);

        let json = gen.to_json(&report).unwrap();
        assert!(json.contains("Test Report"));
        assert!(json.contains("Suite 1"));
    }

    #[test]
    fn test_to_html() {
        let gen = ReportGenerator::new();
        let suite = create_suite_result("s1", "Suite 1");
        let report = gen.generate("Test Report", vec![suite]);

        let html = gen.to_html(&report);
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("Test Report"));
        assert!(html.contains("Suite 1"));
        assert!(html.contains("passed"));
    }

    #[test]
    fn test_to_markdown() {
        let gen = ReportGenerator::new();
        let suite = create_suite_result("s1", "Suite 1");
        let report = gen.generate("Test Report", vec![suite]);

        let md = gen.to_markdown(&report);
        assert!(md.contains("# Test Report"));
        assert!(md.contains("Suite 1"));
        assert!(md.contains("| Test | Status | Duration |"));
    }

    #[test]
    fn test_to_junit() {
        let gen = ReportGenerator::new();
        let suite = create_suite_result("s1", "Suite 1");
        let report = gen.generate("Test Report", vec![suite]);

        let xml = gen.to_junit(&report);
        assert!(xml.contains(r#"<?xml version="1.0" encoding="UTF-8"?>"#));
        assert!(xml.contains("<testsuites"));
        assert!(xml.contains("<testsuite"));
        assert!(xml.contains("<testcase"));
    }

    #[test]
    fn test_html_escape() {
        assert_eq!(html_escape("<script>"), "&lt;script&gt;");
        assert_eq!(html_escape("test & test"), "test &amp; test");
        assert_eq!(html_escape(r#""quoted""#), "&quot;quoted&quot;");
    }

    #[test]
    fn test_escape_xml() {
        assert_eq!(escape_xml("<test>"), "&lt;test&gt;");
        assert_eq!(escape_xml("'test'"), "&apos;test&apos;");
    }

    #[test]
    fn test_pass_rate_calculation() {
        let mut suite = SuiteResult::new("s1", "Suite");
        suite.add_result(create_test_result("t1", "Test 1", TestStatus::Passed));
        suite.add_result(create_test_result("t2", "Test 2", TestStatus::Passed));
        suite.add_result(create_test_result("t3", "Test 3", TestStatus::Failed));
        let suite = suite.complete();

        let summary = ReportSummary::from_suites(&[suite]);
        assert!((summary.pass_rate - 66.66666666666667).abs() < 0.0001);
    }
}
