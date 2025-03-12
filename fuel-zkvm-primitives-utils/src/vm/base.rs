//! the Base trait that should be implemented by all instructions

use fuels::types::input::Input as TxInput;
use fuels::types::output::Output as TxOutput;

/// Trait for converting instructions to byte representation
pub trait AsRepr {
    /// Convert instruction to byte representation
    fn repr(&self) -> Vec<u8>;

    /// Get the script data for the instruction
    fn script_data(&self) -> Option<Vec<u8>> {
        None
    }

    /// Get the additional inputs for the instruction
    fn additional_inputs(&self) -> Option<Vec<TxInput>> {
        None
    }

    /// Get the additional outputs for the instruction
    fn additional_outputs(&self) -> Option<Vec<TxOutput>> {
        None
    }
}
