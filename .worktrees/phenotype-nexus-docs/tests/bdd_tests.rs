//! BDD integration tests for nexus service registry
//! Traces to: E1.1, E1.2, E1.3, E2.1, E3.1, E4.1

use phenotype_bdd::{Feature, ReportFormat, Runner};

mod steps;

#[tokio::test]
async fn test_service_registry_bdd() {
    // Load feature files
    let features = vec![
        Feature::load_file("tests/features/service_registry.feature")
            .expect("Failed to load service_registry.feature"),
        Feature::load_file("tests/features/service_discovery.feature")
            .expect("Failed to load service_discovery.feature"),
    ];

    // Create runner
    let runner = Runner::new().with_features(features).with_report_format(ReportFormat::Pretty);

    // Run all scenarios
    let results = runner.run().await;

    // Generate report
    let report = runner.generate_report(&results);
    println!("{}", report);

    // Assert all passed
    assert!(results.iter().all(|r| r.success), "Some BDD scenarios failed");
}

#[tokio::test]
async fn test_service_registration_scenarios() {
    let feature = Feature::load_file("tests/features/service_registry.feature")
        .expect("Failed to load service_registry.feature");

    let runner = Runner::new().with_feature(feature).filter_by_tag("@registration");

    let results = runner.run().await;
    assert!(results.iter().all(|r| r.success), "Registration scenarios failed");
}

#[tokio::test]
async fn test_service_discovery_scenarios() {
    let feature = Feature::load_file("tests/features/service_discovery.feature")
        .expect("Failed to load service_discovery.feature");

    let runner = Runner::new().with_feature(feature).filter_by_tag("@discovery");

    let results = runner.run().await;
    assert!(results.iter().all(|r| r.success), "Discovery scenarios failed");
}
