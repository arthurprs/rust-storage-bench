use clap::{Parser, ValueEnum};
use serde::Serialize;

#[derive(Copy, Eq, PartialEq, Debug, Clone, ValueEnum, Serialize)]
#[clap(rename_all = "kebab_case")]
pub enum Backend {
    Sled,
    Bloodstone,
    LsmTree,
    Persy,
    JammDb,
    Redb,
}

impl std::fmt::Display for Backend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Sled => "sled 0.34.7",
                Self::Bloodstone => "sled 1.0.0-alpha.118",
                Self::LsmTree => "lsm-tree 0.2.2",
                Self::Persy => "persy 1.4.6",
                Self::JammDb => "jammdb 0.11.0",
                Self::Redb => "redb 1.4.0",
            }
        )
    }
}

#[derive(Copy, Debug, Clone, ValueEnum, Serialize)]
#[clap(rename_all = "kebab_case")]
pub enum Workload {
    /// Workload A: Update heavy workload
    ///
    /// Application example: Session store recording recent actions
    TaskA,

    /// Workload B: Read mostly workload
    ///
    /// Application example: photo tagging; add a tag is an update, but most operations are to read tags
    TaskB,

    /// Workload C: Read only
    ///
    /// Application example: user profile cache, where profiles are constructed elsewhere (e.g., Hadoop)
    TaskC,

    /// Workload D: Read latest workload
    ///
    /// Application example: user status updates; people want to read the latest
    TaskD,

    /// Workload E: Read latest workload with heavy inserts
    ///
    /// Application example: Event logging, getting the latest events
    TaskE,
}

#[derive(Clone, Debug, Eq, PartialEq, clap::ValueEnum)]
pub enum LsmCompaction {
    Leveled,
    Tiered,
}

impl std::fmt::Display for LsmCompaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Leveled => "Leveled",
                Self::Tiered => "Tiered",
            }
        )
    }
}

/// CLI argument parse
#[derive(Parser, Debug)]
#[command(author = "marvin-j97", version = env!("CARGO_PKG_VERSION"), about = "Rust DB Profiler")]
#[command(propagate_version = true)]
pub struct Args {
    #[arg(long, value_enum)]
    pub backend: Backend,

    #[arg(long, value_enum)]
    pub workload: Workload,

    #[arg(long, default_value_t = 1)]
    pub threads: u8,

    #[arg(long)]
    pub items: u32,

    #[arg(long)]
    pub key_size: u8,

    #[arg(long)]
    pub value_size: u16,

    /// Block size for LSM-trees
    #[arg(long, default_value_t = 4_096)]
    pub lsm_block_size: u16,

    /// Compaction for LSM-trees
    #[arg(long, value_enum, default_value_t = LsmCompaction::Leveled)]
    pub lsm_compaction: LsmCompaction,

    /// Intermittenly flush sled to keep memory usage sane
    /// This is hopefully a temporary workaround
    #[arg(long, default_value_t = false)]
    pub sled_flush: bool,

    #[arg(long, default_value_t = 16_000_000)]
    pub cache_size: u32,

    #[arg(long, default_value = "log.jsonl")]
    pub out: String,

    #[arg(long, default_value_t = false)]
    pub snapshot_heap: bool,

    #[arg(long, default_value_t = false)]
    pub fsync: bool,

    #[arg(long, default_value_t = 1)]
    pub minutes: u16,
}
