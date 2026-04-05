// src/agent/auto/mod.rs
// Auto mode modules for automated agent orchestration

pub mod agent;
pub mod build;
pub mod ingest;
pub mod lock;
pub mod minigit;
pub mod pr;
pub mod test;
pub mod verify;

pub use agent::run_agent;
pub use build::run_auto_build;
pub use ingest::run_auto_ingest;
pub use lock::{clear_lock, create_lock, get_lock_for_topic, list_locks};
pub use minigit::{add_diff, create_commit, get_commits, get_topic_tree};
pub use test::run_auto_test;
pub use verify::run_auto_verify;
