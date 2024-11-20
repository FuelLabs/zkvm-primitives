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
    MLDV,
    NOOP,
    NOT,
    OR,
    ORI,
    SLL,
    SLLI,
    SRL,
    SRLI,
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
            Instruction::MLDV => alu::mldv(),
            Instruction::NOOP => alu::noop(),
            Instruction::NOT => alu::not(),
            Instruction::OR => alu::or(),
            Instruction::ORI => alu::ori(),
            Instruction::SLL => alu::sll(),
            Instruction::SLLI => alu::slli(),
            Instruction::SRL => alu::srl(),
            Instruction::SRLI => alu::srli(),
        }
    }
}
