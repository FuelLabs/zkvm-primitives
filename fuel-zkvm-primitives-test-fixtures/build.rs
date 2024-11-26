use std::process::Command;

pub fn main() {
    Command::new("forc")
        .arg("build")
        .arg("--path")
        .arg("src/fixtures/counter_contract")
        .spawn()
        .expect("failed to build contract");
}