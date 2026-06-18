mod cce;
mod config;
#[doc(hidden)]
pub mod duan_test_func;
mod error;
mod population;
mod rng;
mod sce;

#[cfg(test)]
mod duan_tests;

pub use config::Config;
pub use error::SceuaError;
#[cfg(feature = "parallel")]
pub use sce::minimize_parallel;
pub use sce::{minimize, HistoryEntry, OptimizationResult, TerminationReason};
