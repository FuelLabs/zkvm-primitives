use fuels::types::input::Input as TxInput;
use fuels::types::output::Output as TxOutput;

pub trait AsRepr {
    fn repr(&self) -> Vec<u8>;
    fn script_data(&self) -> Option<Vec<u8>> {
        None
    }
    fn additional_inputs(&self) -> Option<Vec<TxInput>> {
        None
    }
    fn additional_outputs(&self) -> Option<Vec<TxOutput>> {
        None
    }
}
