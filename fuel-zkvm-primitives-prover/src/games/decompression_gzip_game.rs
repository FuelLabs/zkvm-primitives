use alloy_sol_types::{private::U256, sol};
use fuel_block_committer_encoding::{
    blob::{self},
    bundle,
};
use fuel_core_compression::VersionedCompressedBlock;
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
        serializer.serialize_bytes(self._inner.as_slice())
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

    #[cfg(test)]
    fn from_inner(inner: [u8; 131072]) -> Self {
        Self {
            _inner: Box::new(inner),
        }
    }
}

impl Default for Blob {
    fn default() -> Self {
        Self {
            _inner: Box::new([0; 131072]),
        }
    }
}

impl From<Blob> for Box<[u8; 131072]> {
    fn from(blob: Blob) -> Self {
        blob.into_inner()
    }
}

impl From<Box<[u8; 131072]>> for Blob {
    fn from(inner: Box<[u8; 131072]>) -> Self {
        Self { _inner: inner }
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

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid input provided")]
    BadInput,
    #[error("Failed to decode raw bytes into bundle")]
    FailedDecodeIntoBundle,
    #[error("Failed to decode bundle into blocks: {0}")]
    FailedDecodeIntoBlocks(String),
    #[error("Failed to decode blocks into single block: {0}")]
    FailedDecodeIntoSingleBlock(String),
    #[error("Failed to get first block")]
    FailedToGetFirstBlock,
    #[error("Failed to get last block")]
    FailedToGetLastBlock,
}

pub type DecompressionGameResult<T> = core::result::Result<T, Error>;

pub fn prove(input_bytes: &[u8]) -> DecompressionGameResult<PublicValuesStruct> {
    let input: Input = bincode::deserialize_from(input_bytes).map_err(|_| Error::BadInput)?;

    let Input { raw_da_blobs } = input;

    let blob_decoder = blob::Decoder::default();

    let raw_da_blobs = raw_da_blobs
        .into_iter()
        .map(Blob::into_inner)
        .collect::<Vec<_>>();

    let compressed_bundle = blob_decoder
        .decode(raw_da_blobs.as_slice())
        .map_err(|_| Error::FailedDecodeIntoBundle)?;

    let bundle_decoder = bundle::Decoder::default();

    let bundle = bundle_decoder
        .decode(compressed_bundle.as_slice())
        .map_err(|e| Error::FailedDecodeIntoBlocks(e.to_string()))?;

    let blocks = match bundle {
        bundle::Bundle::V1(v1_bundle) => {
            let raw_blocks = v1_bundle.blocks;
            raw_blocks
                .iter()
                .map(|raw_block| postcard::from_bytes::<VersionedCompressedBlock>(raw_block))
                .collect::<Result<Vec<_>, _>>()
        }
    }
    .map_err(|e| Error::FailedDecodeIntoSingleBlock(e.to_string()))?;

    let VersionedCompressedBlock::V0(first_block) =
        blocks.first().ok_or(Error::FailedToGetFirstBlock)?;
    let VersionedCompressedBlock::V0(last_block) =
        blocks.last().ok_or(Error::FailedToGetLastBlock)?;

    let first_block_height = u32::from(*first_block.header.height());
    let last_block_height = u32::from(*last_block.header.height());

    Ok(PublicValuesStruct {
        first_block_height: U256::from(first_block_height),
        last_block_height: U256::from(last_block_height),
    })
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use super::*;

    fn set_height(block: &mut VersionedCompressedBlock, height: u32) {
        match block {
            VersionedCompressedBlock::V0(block) => {
                block.header.consensus.height = height.into();
            }
        }
    }

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

    #[test]
    fn prove_fails__if_invalid_block_exists_in_bundle() {
        use rand::Rng;

        let block_size = 1024;
        let block_count = 10;
        let bundle_id = 10;

        let rng = &mut rand::rng();

        let blocks = std::iter::repeat_with(|| {
            let mut buf = vec![0; block_size as usize];
            rng.fill(&mut buf[..]);
            buf
        })
        .take(block_count)
        .collect::<Vec<_>>();

        let blocks = bundle::Bundle::V1(bundle::BundleV1 { blocks });

        let blocks_encoded = bundle::Encoder::default().encode(blocks.clone()).unwrap();

        let blobs = blob::Encoder::default()
            .encode(&blocks_encoded, bundle_id)
            .unwrap();

        let input = Input {
            raw_da_blobs: blobs.into_iter().map(Blob::from).collect(),
        };

        let input_bytes = bincode::serialize(&input).unwrap();

        let result = prove(&input_bytes);

        assert!(matches!(result, Err(Error::FailedDecodeIntoSingleBlock(_))));
    }

    #[test]
    fn prove_fails__if_no_blocks_in_bundle() {
        let blocks = bundle::Bundle::V1(bundle::BundleV1 { blocks: vec![] });

        let blocks_encoded = bundle::Encoder::default().encode(blocks.clone()).unwrap();

        let blobs = blob::Encoder::default().encode(&blocks_encoded, 0).unwrap();

        let input = Input {
            raw_da_blobs: blobs.into_iter().map(Blob::from).collect(),
        };

        let input_bytes = bincode::serialize(&input).unwrap();

        let result = prove(&input_bytes);

        assert!(matches!(result, Err(Error::FailedToGetFirstBlock)));
    }

    #[test]
    fn prove_succeeds__if_valid_blocks_are_provided() {
        let first_height = 5;
        let last_height = 10;

        let mut block_a = VersionedCompressedBlock::V0(Default::default());
        set_height(&mut block_a, first_height);
        let block_a = postcard::to_allocvec(&block_a).unwrap();

        let mut block_b = VersionedCompressedBlock::default();
        set_height(&mut block_b, last_height);
        let block_b = postcard::to_allocvec(&block_b).unwrap();

        let blocks = bundle::Bundle::V1(bundle::BundleV1 {
            blocks: vec![block_a, block_b],
        });

        let blocks_encoded = bundle::Encoder::default().encode(blocks.clone()).unwrap();

        let blobs = blob::Encoder::default().encode(&blocks_encoded, 0).unwrap();

        let input = Input {
            raw_da_blobs: blobs.into_iter().map(Blob::from).collect(),
        };

        let input_bytes = bincode::serialize(&input).unwrap();

        let result = prove(&input_bytes).unwrap();

        assert_eq!(result.first_block_height, U256::from(first_height));
        assert_eq!(result.last_block_height, U256::from(last_height));
    }

    #[test]
    fn prove_succeeds__with_real_data() {
        let blobs = include_bytes!("decompression_gzip_game/mainnet_blobs.bin");
        let blobs: Vec<Vec<u8>> = bincode::deserialize(blobs).unwrap();

        let blobs = blobs
            .iter()
            .map(|blob| {
                let mut blob_array = [0; 131072];
                blob_array.copy_from_slice(&blob[8..]);
                Blob::from_inner(blob_array)
            })
            .collect::<Vec<_>>();

        let input = Input {
            raw_da_blobs: blobs,
        };

        let input_bytes = bincode::serialize(&input).unwrap();

        let result = prove(&input_bytes).unwrap();

        assert!(result.first_block_height < result.last_block_height);
    }
}
