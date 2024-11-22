pub trait AsRepr {
    fn repr(&self) -> Vec<u8>;
    fn script_data(&self) -> Option<Vec<u8>>;
}
