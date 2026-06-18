mod cce;
mod config;
mod error;
mod population;
mod rng;
mod sce;

#[cfg(test)]
mod benchmarks;

pub use config::Config;
pub use error::SceuaError;
#[cfg(feature = "parallel")]
pub use sce::minimize_parallel;
pub use sce::{minimize, HistoryEntry, OptimizationResult, TerminationReason};
