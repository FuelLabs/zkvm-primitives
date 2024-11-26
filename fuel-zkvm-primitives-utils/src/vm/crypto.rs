use crate::vm::AsRepr;
use ed25519_dalek::Signer;
use fuel_core_storage::rand::prelude::StdRng;
use fuel_core_storage::rand::SeedableRng;
use fuel_core_types::fuel_asm::{op, GTFArgs, Instruction, RegId};
use fuel_core_types::fuel_crypto::{secp256r1, Message, PublicKey, SecretKey, Signature};
use std::sync::OnceLock;

static ECK1_SCRIPT_DATA: OnceLock<Vec<u8>> = OnceLock::new();

fn eck1_script_data() -> &'static Vec<u8> {
    ECK1_SCRIPT_DATA.get_or_init(|| {
        let rng = &mut StdRng::seed_from_u64(2322u64);
        let message = Message::new(b"foo");
        let eck1_secret = SecretKey::random(rng);
        let signature = Signature::sign(&eck1_secret, &message);

        signature.iter().chain(message.iter()).copied().collect()
    })
}

static ECR1_SCRIPT_DATA: OnceLock<Vec<u8>> = OnceLock::new();

fn ecr1_script_data() -> &'static Vec<u8> {
    ECR1_SCRIPT_DATA.get_or_init(|| {
        let rng = &mut StdRng::seed_from_u64(2322u64);
        let message = Message::new(b"foo");
        let ecr1_secret = p256::ecdsa::SigningKey::random(rng);
        let signature = secp256r1::sign_prehashed(&ecr1_secret, &message)
            .expect("Failed to sign with secp256r1");

        signature.iter().chain(message.iter()).copied().collect()
    })
}

static ED19_SCRIPT_DATA: OnceLock<(
    ed25519_dalek::SigningKey,
    ed25519_dalek::Signature,
    Vec<u8>,
    Message,
)> = OnceLock::new();

fn ed19_script_data() -> &'static (
    ed25519_dalek::SigningKey,
    ed25519_dalek::Signature,
    Vec<u8>,
    Message,
) {
    ED19_SCRIPT_DATA.get_or_init(|| {
        let mut rng = &mut StdRng::seed_from_u64(2322u64);
        let message = Message::new(b"foo");

        let ed19_secret = ed25519_dalek::SigningKey::generate(&mut rng);
        let signature = ed19_secret.sign(&*message);

        let script_data = ed19_secret
            .as_bytes()
            .iter()
            .chain(ed19_secret.verifying_key().to_bytes().iter())
            .chain(message.iter())
            .copied()
            .collect();

        (ed19_secret, signature, script_data, message)
    })
}

#[cfg_attr(
    feature = "enhanced_enums",
    derive(clap::ValueEnum, enum_iterator::Sequence)
)]
#[derive(Debug, Copy, Clone)]
pub enum CryptoInstruction {
    ECK1,
    ECR1,
    ED19,
    K256,
    S256,
}

impl AsRepr for CryptoInstruction {
    fn repr(&self) -> Vec<u8> {
        match &self {
            CryptoInstruction::ECK1 => eck1(),
            CryptoInstruction::ECR1 => ecr1(),
            CryptoInstruction::ED19 => ed19(),
            CryptoInstruction::K256 => k256(),
            CryptoInstruction::S256 => s256(),
        }
        .into_iter()
        .collect()
    }

    fn script_data(&self) -> Option<Vec<u8>> {
        match &self {
            CryptoInstruction::ECK1 => Some(eck1_script_data().clone()),
            CryptoInstruction::ECR1 => Some(ecr1_script_data().clone()),
            CryptoInstruction::ED19 => Some(ed19_script_data().2.clone()),
            _ => None,
        }
    }
}

fn eck1() -> Vec<Instruction> {
    let eck1_signature = eck1_script_data();

    vec![
        op::gtf_args(0x20, 0x00, GTFArgs::ScriptData),
        op::addi(0x21, 0x20, eck1_signature.len() as u16),
        op::movi(0x10, PublicKey::LEN.try_into().unwrap()),
        op::aloc(0x10),
        op::eck1(RegId::HP, 0x20, 0x21),
        op::jmpb(RegId::ZERO, 0),
    ]
}

fn ecr1() -> Vec<Instruction> {
    let ecr1_signature = ecr1_script_data();

    vec![
        op::gtf_args(0x20, 0x00, GTFArgs::ScriptData),
        op::addi(0x21, 0x20, ecr1_signature.len() as u16),
        op::movi(0x10, PublicKey::LEN.try_into().unwrap()),
        op::aloc(0x10),
        op::ecr1(RegId::HP, 0x20, 0x21),
        op::jmpb(RegId::ZERO, 0),
    ]
}

fn ed19() -> Vec<Instruction> {
    let ed19_params = ed19_script_data();

    vec![
        op::gtf_args(0x20, 0x00, GTFArgs::ScriptData),
        op::addi(
            0x21,
            0x20,
            ed19_params
                .0
                .verifying_key()
                .as_bytes()
                .len()
                .try_into()
                .unwrap(),
        ),
        op::addi(
            0x22,
            0x21,
            ed19_params.1.to_bytes().len().try_into().unwrap(),
        ),
        op::movi(0x10, ed19_params.3.len() as u32),
        op::ed19(0x20, 0x21, 0x22, 0x23),
        op::jmpb(RegId::ZERO, 0),
    ]
}

fn k256() -> Vec<Instruction> {
    vec![
        op::movi(0x11, 32),
        op::aloc(0x11),
        op::movi(0x10, 1000),
        op::cfe(0x10),
        op::k256(RegId::HP, RegId::ZERO, 0x10),
        op::jmpb(RegId::ZERO, 0),
    ]
}

fn s256() -> Vec<Instruction> {
    vec![
        op::movi(0x11, 32),
        op::aloc(0x11),
        op::movi(0x10, 1000),
        op::cfe(0x10),
        op::s256(RegId::HP, RegId::ZERO, 0x10),
        op::jmpb(RegId::ZERO, 0),
    ]
}
