mod alu;

// Implemented instructions for the VM
#[derive(Debug)]
pub enum Instruction {
    ADD,
    ADDI,
    AND,
    ANDI,
    DIV,
    DIVI,
    EQ,
    EXP,
    EXPI,
    GT,
    LT,
    MLOG,
    MOD,
    MODI,
    MOVE,
    MOVI,
    MROO,
    MUL,
    MULI,
    SUB,
    SUBI,
}

impl Instruction {
    pub fn repr(&self) -> Vec<u8> {
        match &self {
            Instruction::ADD => alu::add(),
            Instruction::ADDI => alu::addi(),
            Instruction::AND => alu::and(),
            Instruction::ANDI => alu::andi(),
            Instruction::DIV => alu::div(),
            Instruction::DIVI => alu::divi(),
            Instruction::EQ => alu::eq(),
            Instruction::EXP => alu::exp(),
            Instruction::EXPI => alu::expi(),
            Instruction::GT => alu::gt(),
            Instruction::LT => alu::lt(),
            Instruction::MLOG => alu::mlog(),
            Instruction::MOD => alu::mod_(),
            Instruction::MODI => alu::modi(),
            Instruction::MOVE => alu::move_(),
            Instruction::MOVI => alu::movi(),
            Instruction::MROO => alu::mroo(),
            Instruction::MUL => alu::mul(),
            Instruction::MULI => alu::muli(),
            Instruction::SUB => alu::sub(),
            Instruction::SUBI => alu::subi(),
        }
    }
}
