//! Decompression proving game test fixtures

use clap::builder::PossibleValue;
use std::sync::OnceLock;

/// Random blob sets from mainnet DA
#[cfg_attr(
    feature = "enhanced_enums",
    derive(enum_iterator::Sequence, clap::ValueEnum)
)]
#[cfg_attr(feature = "enhanced_enums", clap(rename_all = "snake_case"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
pub enum BlobSet {
    /// Random blob set 0
    Blob_14133451_14136885,
}

/// Get prover input for a particular blobset
pub fn get_blobset_input(blobset: BlobSet) -> Vec<u8> {
    match blobset {
        BlobSet::Blob_14133451_14136885 => {
            include_bytes!("decompression_fixtures/fixtures/Blob_14133451_14136885.bin").to_vec()
        }
    }
}

static FIXTURE_VARIANTS: OnceLock<Vec<Fixture>> = OnceLock::new();

/// Get all fixtures
pub fn all_fixtures() -> &'static Vec<Fixture> {
    FIXTURE_VARIANTS.get_or_init(|| enum_iterator::all::<Fixture>().collect())
}

/// Fixtures for the prover
#[derive(Debug, Clone, enum_iterator::Sequence)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Fixture {
    /// Blob set
    BlobSet(BlobSet),
}

impl Fixture {
    /// Get the prover input for the fixture
    pub fn get_input_for_fixture(&self) -> Vec<u8> {
        match self {
            Fixture::BlobSet(blobset) => get_blobset_input(*blobset),
        }
    }
}

impl clap::ValueEnum for Fixture {
    fn value_variants<'a>() -> &'a [Self] {
        all_fixtures().as_slice()
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        match self {
            Fixture::BlobSet(blobset) => blobset.to_possible_value(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fuel_zkvm_primitives_prover::games::decompression_game::prove;

    #[test]
    fn test_all_fixtures() -> Result<(), String> {
        for fixture in all_fixtures() {
            let prover_input = fixture.get_input_for_fixture();

            if prover_input.is_empty() {
                return Err(format!("Fixture '{:?}' has empty prover input", fixture));
            }

            let result = prove(&prover_input).unwrap();

            assert!(result.first_block_height < result.last_block_height);
        }

        Ok(())
    }
}
