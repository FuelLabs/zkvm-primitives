//! Test fixture for the counter contract. the transaction increments the counter by 1.

/// Returns the serialized input for the counter contract.
pub fn get_counter_contract_input() -> Vec<u8> {
    include_bytes!("fixtures/counter_contract/input.bin").to_vec()
}
