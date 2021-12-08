#![deny(clippy::all, clippy::perf, clippy::correctness)]
#![feature(async_await)]
#![feature(rustc_private)]

#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;

pub use filecoin_proofs::types::*;

pub use crate::builder::*;
pub use crate::constants::*;
pub use crate::error::*;
pub use crate::metadata::*;
pub use crate::store::*;

// Exported for benchmarks
pub use crate::helpers::checksum::calculate_checksum;
extern crate uuid;

extern crate nix;
pub mod builder;
pub mod constants;
pub mod disk_backed_storage;
pub mod error;
pub mod helpers;
pub mod kv_store;
pub mod metadata;
pub mod scheduler;
pub mod sealer;
pub mod state;
pub mod store;
