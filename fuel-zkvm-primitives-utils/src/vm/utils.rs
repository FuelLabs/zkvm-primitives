use fuel_core_types::fuel_asm::{op, Instruction, RegId};

/// Copied from https://github.com/FuelLabs/fuel-core/blob/4986d4d034499dafc19b9dcd72458717b6ecdd5b/benches/benches/utils.rs#L38-L57
/// Allocates a byte array from heap and initializes it. Then points `reg` to it.
pub(crate) fn alloc_bytearray<const S: usize>(reg: u8, v: [u8; S]) -> Vec<Instruction> {
    let mut ops = vec![op::movi(reg, u32::try_from(S).expect("qed")), op::aloc(reg)];
    for (i, b) in v.iter().enumerate() {
        if *b != 0 {
            ops.push(op::movi(reg, *b as u32));
            ops.push(op::sb(RegId::HP, reg, u16::try_from(i).expect("qed")));
        }
    }
    ops.push(op::move_(reg, RegId::HP));
    ops
}
