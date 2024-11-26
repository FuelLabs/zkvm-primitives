use std::env;
use std::path::Path;
use std::process::Command;

pub fn main() {
    // Check if `forc` is available in the system PATH
    let forc_binary = which::which("forc").unwrap_or_else(|_| {
        // If `forc` is not found in PATH, check $HOME/.fuelup/bin/forc
        let home_dir = env::var("HOME").expect("Failed to get $HOME directory");
        let fallback_path = Path::new(&home_dir).join(".fuelup/bin/forc");
        if fallback_path.exists() {
            fallback_path
        } else {
            panic!("`forc` binary not found in PATH or at $HOME/.fuelup/bin/forc");
        }
    });

    // Use the determined `forc` binary to build the contract
    Command::new(forc_binary)
        .arg("build")
        .arg("--path")
        .arg("src/fixtures/counter_contract")
        .spawn()
        .expect("Failed to build contract");
}
