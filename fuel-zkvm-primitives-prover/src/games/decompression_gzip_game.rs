use alloy_sol_types::{private::U256, sol};
use fuel_block_committer_encoding::{
    blob::{self},
    bundle,
};
extern crate alloc;

#[derive(Clone)]
pub struct Blob {
    _inner: Box<[u8; 131072]>,
}

impl serde::Serialize for Blob {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(&self._inner.as_slice())
    }
}

impl<'de> serde::Deserialize<'de> for Blob {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let inner = <Vec<u8>>::deserialize(deserializer)?;
        match inner.into_boxed_slice().try_into() {
            Ok(v) => Ok(Self { _inner: v }),
            Err(_) => Err(serde::de::Error::custom("Blob must be 131072 bytes long")),
        }
    }
}

impl Blob {
    pub fn new(raw_blob: Vec<u8>) -> Self {
        let mut inner = [0; 131072];
        inner[..raw_blob.len()].copy_from_slice(&raw_blob);
        Self {
            _inner: Box::new(inner),
        }
    }

    fn into_inner(self) -> Box<[u8; 131072]> {
        self._inner
    }
}

impl Default for Blob {
    fn default() -> Self {
        Self {
            _inner: Box::new([0; 131072]),
        }
    }
}

/// This is the input to the decompression game.
/// We perform the following validation:
/// 1. gzip decompress the blob into a set of compressed blocks
/// 2. TBD
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Input {
    // a set of blobs make up a compressed bundle
    // a compressed bundle is made of several bundles
    // each bundle is made of several da compressed block
    raw_da_blobs: Vec<Blob>,
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
    FailedDecodeIntoBundle,
    FailedDecodeIntoBlocks,
}

pub type DecompressionGameResult<T> = core::result::Result<T, Error>;

pub fn prove(input_bytes: &[u8]) -> DecompressionGameResult<PublicValuesStruct> {
    let input: Input = bincode::deserialize_from(input_bytes).map_err(|_| Error::BadInput)?;

    let Input { raw_da_blobs } = input;

    let blob_decoder = blob::Decoder::default();

    let raw_da_blobs = raw_da_blobs
        .iter()
        .cloned()
        .map(|blob| blob.into_inner())
        .collect::<Vec<_>>();

    let compressed_bundle = blob_decoder
        .decode(raw_da_blobs.as_slice())
        .map_err(|_| Error::FailedDecodeIntoBundle)?;

    let bundle_decoder = bundle::Decoder::default();

    let bundle = bundle_decoder
        .decode(compressed_bundle.as_slice())
        .map_err(|_| Error::FailedDecodeIntoBlocks)?;

    match bundle {
        bundle::Bundle::V1(v1_bundle) => {
            let _blocks = v1_bundle.blocks;
            // postcard deserialize each block into VersionedCompressedBlock
        }
    }

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
    fn prove_fails__if_invalid_blob_provided() {
        let input = Input {
            raw_da_blobs: vec![Blob::default()],
        };

        let input_bytes = bincode::serialize(&input).unwrap();

        let result = prove(&input_bytes);

        assert!(matches!(result, Err(Error::FailedDecodeIntoBundle)));
    }
}
