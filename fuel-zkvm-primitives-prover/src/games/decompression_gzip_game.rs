use alloy_sol_types::{private::U256, sol};
use fuel_block_committer_encoding::bundle::Decoder;
extern crate alloc;

/// This is the input to the decompression game.
/// We perform the following validation:
/// 1. gzip decompress the blob into a set of compressed blocks
/// 2. TBD
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Input {
    raw_da_blob: Vec<u8>,
}

sol! {
    /// The public values encoded as a struct that can be easily deserialized inside Solidity.
    struct PublicValuesStruct {
        uint256 first_block_height;
        uint256 last_block_height;
    }
}

pub enum Error {
    BadInput,
    FailedDecompression,
}

pub type DecompressionGameResult<T> = core::result::Result<T, Error>;

pub fn prove(input_bytes: &[u8]) -> DecompressionGameResult<PublicValuesStruct> {
    let input: Input = bincode::deserialize_from(input_bytes).map_err(|_| Error::BadInput)?;

    let Input { raw_da_blob } = input;

    let decoder = Decoder::default();

    let _bundle = decoder
        .decode(raw_da_blob.as_slice())
        .map_err(|_| Error::FailedDecompression)?;

    Ok(PublicValuesStruct {
        first_block_height: U256::from_be_bytes(0u32.to_be_bytes()),
        last_block_height: U256::from_be_bytes(0u32.to_be_bytes()),
    })
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prove_fails__if_bad_input_provided() {
        #[derive(serde::Serialize)]
        struct BadInput {
            foo: u32,
        }

        let bad_input = BadInput { foo: 10 };

        let input_bytes = bincode::serialize(&bad_input).unwrap();

        let result = prove(&input_bytes);

        assert!(matches!(result, Err(Error::BadInput)));
    }

    #[test]
    fn prove_fails__if_invalid_bundle_provided() {
        let input = Input {
            raw_da_blob: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
        };

        let input_bytes = bincode::serialize(&input).unwrap();

        let result = prove(&input_bytes);

        assert!(matches!(result, Err(Error::FailedDecompression)));
    }
}
