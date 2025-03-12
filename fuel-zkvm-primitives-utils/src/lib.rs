//! Utility crate so it can be used in the build script in `fuel-zkvm-primitives-test-fixtures`

#![deny(clippy::arithmetic_side_effects)]
#![deny(clippy::cast_possible_truncation)]
#![deny(unused_crate_dependencies)]
#![deny(missing_docs)]
#![deny(warnings)]

pub mod vm;

use tai64 as _;
