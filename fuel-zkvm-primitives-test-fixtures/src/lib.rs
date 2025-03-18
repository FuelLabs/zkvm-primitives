//! Test fixtures to be tested within a zkvm context

#![deny(clippy::arithmetic_side_effects)]
#![deny(clippy::cast_possible_truncation)]
#![deny(unused_crate_dependencies)]
#![deny(missing_docs)]
#![deny(warnings)]

use tai64 as _;

pub mod block_execution_fixtures;
pub mod decompression_fixtures;
