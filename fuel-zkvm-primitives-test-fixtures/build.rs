use std::path::Path;
use forc::cli::BuildCommand;
use forc::cli::shared::{Build, BuildOutput, Pkg};
use forc::ops::forc_build;

pub fn main() {
    let base_path = Path::new("src/fixtures/counter_contract");
    let out_dir = base_path.join("out");
    let out_bin = out_dir.join("counter_contract.bin");

    // create out_dir if it doesn't exist
    std::fs::create_dir_all(out_dir).expect("Failed to create out directory");

    let build_command = BuildCommand {
        build: Build {
            pkg: Pkg {
                path: Some(base_path.display().to_string()),
                offline: false,
                terse: false,
                output_directory: None,
                locked: false,
                ipfs_node: None,
            },
            print: Default::default(),
            minify: Default::default(),
            output: BuildOutput {
                bin_file: Some(out_bin.display().to_string()),
                debug_file: None,
            },
            profile: Default::default(),
            build_target: Default::default(),
        },
        tests: false,
        experimental: Default::default(),
    };

    forc_build::build(build_command).expect("Failed to build contract");
}
