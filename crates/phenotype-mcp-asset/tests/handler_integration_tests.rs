//! Integration tests for AssetHandler

use phenotype_mcp_asset::*;
use tempfile::TempDir;
use tokio::fs;

async fn create_test_pack(dir: &TempDir, name: &str, version: &str) -> std::path::PathBuf {
    let pack_dir = dir.path().join(name);
    fs::create_dir(&pack_dir).await.unwrap();
    
    // Create manifest
    let manifest = format!(r#"
name = "{}"
version = "{}"
author = "Test Author"
description = "A test pack"

[[assets]]
name = "main"
path = "main.py"
type = "python_script"

[[assets]]
name = "utils"
path = "lib/utils.py"
type = "python_script"

[[dependencies]]
name = "base-pack"
version_constraint = ">=1.0.0"
"#, name, version);
    
    fs::write(pack_dir.join("phenotype.toml"), manifest).await.unwrap();
    
    // Create asset files
    fs::write(pack_dir.join("main.py"), "# main script").await.unwrap();
    fs::create_dir(pack_dir.join("lib")).await.unwrap();
    fs::write(pack_dir.join("lib/utils.py"), "# utils").await.unwrap();
    
    pack_dir
}

#[tokio::test]
async fn test_handler_discover_finds_packs() {
    let temp_dir = tempfile::tempdir().unwrap();
    
    // Create test packs
    create_test_pack(&temp_dir, "pack1", "1.0.0").await;
    create_test_pack(&temp_dir, "pack2", "2.0.0").await;
    
    let handler = AssetHandler::new(temp_dir.path());
    let result = handler.discover(".", true).await;
    
    // Should find at least the manifests and Python files
    assert!(!result.assets.is_empty());
    
    // Should find the manifests
    let manifests: Vec<_> = result.assets.iter()
        .filter(|a| a.asset_type == AssetType::ContentPack)
        .collect();
    assert!(!manifests.is_empty());
}

#[tokio::test]
async fn test_handler_build_pack() {
    let temp_dir = tempfile::tempdir().unwrap();
    let pack_dir = create_test_pack(&temp_dir, "my-pack", "1.0.0").await;
    let output_dir = temp_dir.path().join("output");
    
    let handler = AssetHandler::new(temp_dir.path());
    let result = handler.build(&pack_dir, &output_dir).await;
    
    assert!(result.success, "Build failed: {:?}", result.errors);
    assert!(output_dir.exists());
    assert!(output_dir.join("phenotype.toml").exists());
    assert!(output_dir.join("main.py").exists());
    assert!(output_dir.join("lib/utils.py").exists());
    assert!(output_dir.join("CHECKSUMS.sha256").exists());
}

#[tokio::test]
async fn test_handler_build_invalid_pack() {
    let temp_dir = tempfile::tempdir().unwrap();
    let pack_dir = temp_dir.path().join("invalid");
    fs::create_dir(&pack_dir).await.unwrap();
    
    // Create invalid manifest (missing required fields)
    fs::write(
        pack_dir.join("phenotype.toml"),
        "version = '1.0.0'"  // Missing name
    ).await.unwrap();
    
    let handler = AssetHandler::new(temp_dir.path());
    let result = handler.build(&pack_dir, temp_dir.path().join("output")).await;
    
    assert!(!result.success);
}

#[tokio::test]
async fn test_handler_validate_valid() {
    let temp_dir = tempfile::tempdir().unwrap();
    let pack_dir = create_test_pack(&temp_dir, "valid-pack", "1.0.0").await;
    
    let handler = AssetHandler::new(temp_dir.path());
    let result = handler.validate(&pack_dir).await;
    
    assert!(result.valid, "Validation failed: {:?}", result.errors);
}

#[tokio::test]
async fn test_handler_validate_invalid() {
    let temp_dir = tempfile::tempdir().unwrap();
    let pack_dir = temp_dir.path().join("invalid");
    fs::create_dir(&pack_dir).await.unwrap();
    
    fs::write(
        pack_dir.join("phenotype.toml"),
        "version = 'not-semver'"  // Missing name and invalid version
    ).await.unwrap();
    
    let handler = AssetHandler::new(temp_dir.path());
    let result = handler.validate(&pack_dir).await;
    
    assert!(!result.valid);
    assert!(!result.errors.is_empty());
}

#[tokio::test]
async fn test_handler_get_info() {
    let temp_dir = tempfile::tempdir().unwrap();
    let pack_dir = create_test_pack(&temp_dir, "info-pack", "3.0.0").await;
    
    let handler = AssetHandler::new(temp_dir.path());
    let info = handler.get_info(&pack_dir).await;
    
    assert!(info.is_some());
    let info = info.unwrap();
    assert_eq!(info.name, "info-pack");
    assert_eq!(info.version, "3.0.0");
    assert_eq!(info.author, Some("Test Author".to_string()));
    assert_eq!(info.asset_count, 2);
    assert_eq!(info.dependency_count, 1);
    assert!(info.total_size_bytes > 0);
}

#[tokio::test]
async fn test_handler_get_info_not_found() {
    let temp_dir = tempfile::tempdir().unwrap();
    let handler = AssetHandler::new(temp_dir.path());
    
    let info = handler.get_info("nonexistent").await;
    assert!(info.is_none());
}

#[tokio::test]
async fn test_handler_resolve_dependencies() {
    let temp_dir = tempfile::tempdir().unwrap();
    let pack_dir = create_test_pack(&temp_dir, "dependent", "1.0.0").await;
    
    let handler = AssetHandler::new(temp_dir.path());
    let result = handler.resolve_dependencies(&pack_dir).await;
    
    // The mock resolver resolves dependencies (extracts version from constraint)
    assert_eq!(result.resolved.len(), 1);  // base-pack from create_test_pack
    assert_eq!(result.resolved[0].name, "base-pack");
}

#[tokio::test]
async fn test_handler_resolve_no_dependencies() {
    let temp_dir = tempfile::tempdir().unwrap();
    let pack_dir = temp_dir.path().join("no-deps");
    fs::create_dir(&pack_dir).await.unwrap();
    
    fs::write(
        pack_dir.join("phenotype.toml"),
        "name = 'no-deps'\nversion = '1.0.0'"
    ).await.unwrap();
    
    let handler = AssetHandler::new(temp_dir.path());
    let result = handler.resolve_dependencies(&pack_dir).await;
    
    // No dependencies = fully resolved
    assert!(result.fully_resolved());
}

#[tokio::test]
async fn test_discovery_result_get_by_type() {
    let temp_dir = tempfile::tempdir().unwrap();
    
    // Create files of different types
    fs::write(temp_dir.path().join("script.py"), "# python").await.unwrap();
    fs::write(temp_dir.path().join("module.js"), "// js").await.unwrap();
    fs::write(temp_dir.path().join("manifest.toml"), "name = 'test'").await.unwrap();
    
    let handler = AssetHandler::new(temp_dir.path());
    let result = handler.discover(".", false).await;
    
    let python_assets = result.get_by_type(AssetType::PythonScript);
    let js_assets = result.get_by_type(AssetType::JavaScriptModule);
    
    assert!(!python_assets.is_empty() || !js_assets.is_empty());
}

#[tokio::test]
async fn test_discovery_result_count_by_type() {
    let temp_dir = tempfile::tempdir().unwrap();
    
    // Create multiple files of same type
    fs::write(temp_dir.path().join("a.py"), "# a").await.unwrap();
    fs::write(temp_dir.path().join("b.py"), "# b").await.unwrap();
    fs::write(temp_dir.path().join("c.js"), "// c").await.unwrap();
    
    let handler = AssetHandler::new(temp_dir.path());
    let result = handler.discover(".", false).await;
    
    let counts = result.count_by_type();
    
    // Should have counts for found types
    assert!(!counts.is_empty() || result.assets.is_empty());
}

#[tokio::test]
async fn test_pack_info_size_formatting() {
    let temp_dir = tempfile::tempdir().unwrap();
    let pack_dir = create_test_pack(&temp_dir, "size-pack", "1.0.0").await;
    
    let handler = AssetHandler::new(temp_dir.path());
    let info = handler.get_info(&pack_dir).await.unwrap();
    
    let size_str = info.size_human_readable();
    assert!(size_str.contains("B") || size_str.contains("KB"));
}

#[tokio::test]
async fn test_pack_info_markdown() {
    let temp_dir = tempfile::tempdir().unwrap();
    let pack_dir = create_test_pack(&temp_dir, "md-pack", "1.0.0").await;
    
    let handler = AssetHandler::new(temp_dir.path());
    let info = handler.get_info(&pack_dir).await.unwrap();
    
    let markdown = info.to_markdown();
    assert!(markdown.contains("md-pack"));
    assert!(markdown.contains("1.0.0"));
    assert!(markdown.contains("Test Author"));
}

#[tokio::test]
async fn test_build_result_add_artifact() {
    let result = BuildResult::success("/output").add_artifact("file1.py").add_artifact("file2.py");
    
    assert!(result.success);
    assert_eq!(result.artifacts.len(), 2);
}

#[tokio::test]
async fn test_validation_result_merge() {
    let mut result1 = ValidationResult::success();
    result1.add_warning("Warning 1");
    
    let mut result2 = ValidationResult::success();
    result2.add_warning("Warning 2");
    
    result1.merge(result2);
    
    assert!(result1.valid);
    assert_eq!(result1.warnings.len(), 2);
}

#[tokio::test]
async fn test_validation_result_merge_with_error() {
    let mut result1 = ValidationResult::success();
    
    let result2 = ValidationResult::error("Error occurred");
    
    result1.merge(result2);
    
    assert!(!result1.valid);
    assert_eq!(result1.errors.len(), 1);
}
