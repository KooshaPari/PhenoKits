//! AgilePlus P2P — Git-backed state export
//!
//! @trace SKILL-001: Skill Definition
//! @trace SKILL-002: Skill Registry
//! @trace SKILL-003: Hot Reload
//! @trace SKILL-004: Native Skill Support
//! @trace SKILL-005: Python Skill Support
//! @trace SKILL-006: WASM Skill Support
//! @trace SKILL-007: Dependency Resolution
//! @trace SKILL-008: Sandbox Execution
//!
//! This crate provides functionality to export SQLite state to deterministic,
//! git-friendly JSON files for P2P synchronization.
//!
//! # Example
//!
//! ```ignore
//! use agileplus_p2p::export::export_state;
//!
//! let stats = export_state(&event_store, &snapshot_store, &device_store,
//!     &entities, output_dir).await?;
//! println!("Exported {} events", stats.events_exported);
//! ```

pub mod device;
pub mod domain;
pub mod error;
pub mod events;
pub mod export;

pub use device::{DeviceNode, DeviceStore, InMemoryDeviceStore};
pub use export::{export_state, ExportError, ExportStats};
