use crate::instruction::Instruction;
use crate::opcodes::OpCode;
use crate::registers::Register;

pub fn read_next(buf: &Vec<u8>, ix: usize) -> Instruction {
    let code = buf[ix];

    match code {
        0x00 => Instruction{ opcode: OpCode::NOP, register: vec![], operands: vec![] },
        0x01 => Instruction{ opcode: OpCode::LXI, register: vec![Register::B], operands: vec![buf[ix+2],buf[ix+1]] },
        0x02 => Instruction{ opcode: OpCode::STAX, register: vec![Register::B], operands: vec![] },
        0x03 => Instruction{ opcode: OpCode::INX, register: vec![Register::B], operands: vec![] },
        0x04 => Instruction{ opcode: OpCode::INR, register: vec![Register::B], operands: vec![] },
        0x05 => Instruction{ opcode: OpCode::DCR, register: vec![Register::B], operands: vec![] },
        0x06 => Instruction{ opcode: OpCode::MVI, register: vec![Register::B], operands: vec![buf[ix+1]] },
        0x07 => Instruction{ opcode: OpCode::RLC, register: vec![], operands: vec![] },
        0x08 => Instruction{ opcode: OpCode::NOP, register: vec![], operands: vec![] },
        0x09 => Instruction{ opcode: OpCode::DAD, register: vec![Register::B], operands: vec![] },
        0x0a => Instruction{ opcode: OpCode::LDAX, register: vec![Register::B], operands: vec![] },
        0x0b => Instruction{ opcode: OpCode::DCX, register: vec![Register::B], operands: vec![] },
        0x0c => Instruction{ opcode: OpCode::INR, register: vec![Register::C], operands: vec![] },
        0x0d => Instruction{ opcode: OpCode::DCR, register: vec![Register::C], operands: vec![] },
        0x0e => Instruction{ opcode: OpCode::MVI, register: vec![Register::C], operands: vec![buf[ix+1]] },
        0x0f => Instruction{ opcode: OpCode::RRC, register: vec![], operands: vec![] },
        0x10 => Instruction{ opcode: OpCode::NOP, register: vec![], operands: vec![] },
        0x11 => Instruction{ opcode: OpCode::LXI, register: vec![Register::D], operands: vec![buf[ix+2],buf[ix+1]] },
        0x12 => Instruction{ opcode: OpCode::STAX, register: vec![Register::D], operands: vec![] },
        0x13 => Instruction{ opcode: OpCode::INX, register: vec![Register::D], operands: vec![] },
        0x14 => Instruction{ opcode: OpCode::INR, register: vec![Register::D], operands: vec![] },
        0x15 => Instruction{ opcode: OpCode::DCR, register: vec![Register::D], operands: vec![] },
        0x16 => Instruction{ opcode: OpCode::MVI, register: vec![Register::D], operands: vec![buf[ix+1]] },
        0x17 => Instruction{ opcode: OpCode::RAL, register: vec![], operands: vec![] },
        0x18 => Instruction{ opcode: OpCode::NOP, register: vec![], operands: vec![] },
        0x19 => Instruction{ opcode: OpCode::DAD, register: vec![Register::D], operands: vec![] },
        0x1a => Instruction{ opcode: OpCode::LDAX, register: vec![Register::D], operands: vec![] },
        0x1b => Instruction{ opcode: OpCode::DCX, register: vec![Register::D], operands: vec![] },
        0x1c => Instruction{ opcode: OpCode::INR, register: vec![Register::E], operands: vec![] },
        0x1d => Instruction{ opcode: OpCode::DCR, register: vec![Register::E], operands: vec![] },
        0x1e => Instruction{ opcode: OpCode::MVI, register: vec![Register::E], operands: vec![buf[ix+1]] },
        0x1f => Instruction{ opcode: OpCode::RAR, register: vec![], operands: vec![] },
        0x20 => Instruction{ opcode: OpCode::NOP, register: vec![], operands: vec![] },
        0x21 => Instruction{ opcode: OpCode::LXI, register: vec![Register::H], operands: vec![buf[ix+2],buf[ix+1]] },
        0x22 => Instruction{ opcode: OpCode::SHLD, register: vec![], operands: vec![buf[ix+2],buf[ix+1]] },
        0x23 => Instruction{ opcode: OpCode::INX, register: vec![Register::H], operands: vec![] },
        0x24 => Instruction{ opcode: OpCode::INR, register: vec![Register::H], operands: vec![] },
        0x25 => Instruction{ opcode: OpCode::DCR, register: vec![Register::H], operands: vec![] },
        0x26 => Instruction{ opcode: OpCode::MVI, register: vec![Register::H], operands: vec![buf[ix+1]] },
        0x27 => Instruction{ opcode: OpCode::DAA, register: vec![], operands: vec![] },
        0x28 => Instruction{ opcode: OpCode::NOP, register: vec![], operands: vec![] },
        0x29 => Instruction{ opcode: OpCode::DAD, register: vec![Register::H], operands: vec![] },
        0x2a => Instruction{ opcode: OpCode::LHLD, register: vec![], operands: vec![buf[ix+2],buf[ix+1]] },
        0x2b => Instruction{ opcode: OpCode::DCX, register: vec![Register::H], operands: vec![] },
        0x2c => Instruction{ opcode: OpCode::INR, register: vec![Register::L], operands: vec![] },
        0x2d => Instruction{ opcode: OpCode::DCR, register: vec![Register::L], operands: vec![] },
        0x2e => Instruction{ opcode: OpCode::MVI, register: vec![Register::L], operands: vec![buf[ix+1]] },
        0x2f => Instruction{ opcode: OpCode::CMA, register: vec![], operands: vec![] },
        0x30 => Instruction{ opcode: OpCode::NOP, register: vec![], operands: vec![] },
        0x31 => Instruction{ opcode: OpCode::LXI, register: vec![Register::SP], operands: vec![buf[ix+2],buf[ix+1]] },
        0x32 => Instruction{ opcode: OpCode::STA, register: vec![], operands: vec![buf[ix+2],buf[ix+1]] },
        0x33 => Instruction{ opcode: OpCode::INX, register: vec![Register::SP], operands: vec![] },
        0x34 => Instruction{ opcode: OpCode::INR, register: vec![Register::M], operands: vec![] },
        0x35 => Instruction{ opcode: OpCode::DCR, register: vec![Register::M], operands: vec![] },
        0x36 => Instruction{ opcode: OpCode::MVI, register: vec![Register::M], operands: vec![buf[ix+1]] },
        0x37 => Instruction{ opcode: OpCode::STC, register: vec![], operands: vec![] },
        0x38 => Instruction{ opcode: OpCode::NOP, register: vec![], operands: vec![] },
        0x39 => Instruction{ opcode: OpCode::DAD, register: vec![Register::SP], operands: vec![] },
        0x3a => Instruction{ opcode: OpCode::LDA, register: vec![], operands: vec![buf[ix+2],buf[ix+1]] },
        0x3b => Instruction{ opcode: OpCode::DCX, register: vec![Register::SP], operands: vec![] },
        0x3c => Instruction{ opcode: OpCode::INR, register: vec![Register::A], operands: vec![] },
        0x3d => Instruction{ opcode: OpCode::DCR, register: vec![Register::A], operands: vec![] },
        0x3e => Instruction{ opcode: OpCode::MVI, register: vec![Register::A], operands: vec![buf[ix+1]] },
        0x3f => Instruction{ opcode: OpCode::CMC, register: vec![], operands: vec![] },
        0x40 => Instruction{ opcode: OpCode::MOV, register: vec![Register::B,Register::B], operands: vec![] },
        0x41 => Instruction{ opcode: OpCode::MOV, register: vec![Register::B,Register::C], operands: vec![] },
        0x42 => Instruction{ opcode: OpCode::MOV, register: vec![Register::B,Register::D], operands: vec![] },
        0x43 => Instruction{ opcode: OpCode::MOV, register: vec![Register::B,Register::E], operands: vec![] },
        0x44 => Instruction{ opcode: OpCode::MOV, register: vec![Register::B,Register::H], operands: vec![] },
        0x45 => Instruction{ opcode: OpCode::MOV, register: vec![Register::B,Register::L], operands: vec![] },
        0x46 => Instruction{ opcode: OpCode::MOV, register: vec![Register::B,Register::M], operands: vec![] },
        0x47 => Instruction{ opcode: OpCode::MOV, register: vec![Register::B,Register::A], operands: vec![] },
        0x48 => Instruction{ opcode: OpCode::MOV, register: vec![Register::C,Register::B], operands: vec![] },
        0x49 => Instruction{ opcode: OpCode::MOV, register: vec![Register::C,Register::C], operands: vec![] },
        0x4a => Instruction{ opcode: OpCode::MOV, register: vec![Register::C,Register::D], operands: vec![] },
        0x4b => Instruction{ opcode: OpCode::MOV, register: vec![Register::C,Register::E], operands: vec![] },
        0x4c => Instruction{ opcode: OpCode::MOV, register: vec![Register::C,Register::H], operands: vec![] },
        0x4d => Instruction{ opcode: OpCode::MOV, register: vec![Register::C,Register::L], operands: vec![] },
        0x4e => Instruction{ opcode: OpCode::MOV, register: vec![Register::C,Register::M], operands: vec![] },
        0x4f => Instruction{ opcode: OpCode::MOV, register: vec![Register::C,Register::A], operands: vec![] },
        0x50 => Instruction{ opcode: OpCode::MOV, register: vec![Register::D,Register::B], operands: vec![] },
        0x51 => Instruction{ opcode: OpCode::MOV, register: vec![Register::D,Register::C], operands: vec![] },
        0x52 => Instruction{ opcode: OpCode::MOV, register: vec![Register::D,Register::D], operands: vec![] },
        0x53 => Instruction{ opcode: OpCode::MOV, register: vec![Register::D,Register::E], operands: vec![] },
        0x54 => Instruction{ opcode: OpCode::MOV, register: vec![Register::D,Register::H], operands: vec![] },
        0x55 => Instruction{ opcode: OpCode::MOV, register: vec![Register::D,Register::L], operands: vec![] },
        0x56 => Instruction{ opcode: OpCode::MOV, register: vec![Register::D,Register::M], operands: vec![] },
        0x57 => Instruction{ opcode: OpCode::MOV, register: vec![Register::D,Register::A], operands: vec![] },
        0x58 => Instruction{ opcode: OpCode::MOV, register: vec![Register::E,Register::B], operands: vec![] },
        0x59 => Instruction{ opcode: OpCode::MOV, register: vec![Register::E,Register::C], operands: vec![] },
        0x5a => Instruction{ opcode: OpCode::MOV, register: vec![Register::E,Register::D], operands: vec![] },
        0x5b => Instruction{ opcode: OpCode::MOV, register: vec![Register::E,Register::E], operands: vec![] },
        0x5c => Instruction{ opcode: OpCode::MOV, register: vec![Register::E,Register::H], operands: vec![] },
        0x5d => Instruction{ opcode: OpCode::MOV, register: vec![Register::E,Register::L], operands: vec![] },
        0x5e => Instruction{ opcode: OpCode::MOV, register: vec![Register::E,Register::M], operands: vec![] },
        0x5f => Instruction{ opcode: OpCode::MOV, register: vec![Register::E,Register::A], operands: vec![] },
        0x60 => Instruction{ opcode: OpCode::MOV, register: vec![Register::H,Register::B], operands: vec![] },
        0x61 => Instruction{ opcode: OpCode::MOV, register: vec![Register::H,Register::C], operands: vec![] },
        0x62 => Instruction{ opcode: OpCode::MOV, register: vec![Register::H,Register::D], operands: vec![] },
        0x63 => Instruction{ opcode: OpCode::MOV, register: vec![Register::H,Register::E], operands: vec![] },
        0x64 => Instruction{ opcode: OpCode::MOV, register: vec![Register::H,Register::H], operands: vec![] },
        0x65 => Instruction{ opcode: OpCode::MOV, register: vec![Register::H,Register::L], operands: vec![] },
        0x66 => Instruction{ opcode: OpCode::MOV, register: vec![Register::H,Register::M], operands: vec![] },
        0x67 => Instruction{ opcode: OpCode::MOV, register: vec![Register::H,Register::A], operands: vec![] },
        0x68 => Instruction{ opcode: OpCode::MOV, register: vec![Register::L,Register::B], operands: vec![] },
        0x69 => Instruction{ opcode: OpCode::MOV, register: vec![Register::L,Register::C], operands: vec![] },
        0x6a => Instruction{ opcode: OpCode::MOV, register: vec![Register::L,Register::D], operands: vec![] },
        0x6b => Instruction{ opcode: OpCode::MOV, register: vec![Register::L,Register::E], operands: vec![] },
        0x6c => Instruction{ opcode: OpCode::MOV, register: vec![Register::L,Register::H], operands: vec![] },
        0x6d => Instruction{ opcode: OpCode::MOV, register: vec![Register::L,Register::L], operands: vec![] },
        0x6e => Instruction{ opcode: OpCode::MOV, register: vec![Register::L,Register::M], operands: vec![] },
        0x6f => Instruction{ opcode: OpCode::MOV, register: vec![Register::L,Register::A], operands: vec![] },
        0x70 => Instruction{ opcode: OpCode::MOV, register: vec![Register::M,Register::B], operands: vec![] },
        0x71 => Instruction{ opcode: OpCode::MOV, register: vec![Register::M,Register::C], operands: vec![] },
        0x72 => Instruction{ opcode: OpCode::MOV, register: vec![Register::M,Register::D], operands: vec![] },
        0x73 => Instruction{ opcode: OpCode::MOV, register: vec![Register::M,Register::E], operands: vec![] },
        0x74 => Instruction{ opcode: OpCode::MOV, register: vec![Register::M,Register::H], operands: vec![] },
        0x75 => Instruction{ opcode: OpCode::MOV, register: vec![Register::M,Register::L], operands: vec![] },
        0x76 => Instruction{ opcode: OpCode::HLT, register: vec![], operands: vec![] },
        0x77 => Instruction{ opcode: OpCode::MOV, register: vec![Register::M,Register::A], operands: vec![] },
        0x78 => Instruction{ opcode: OpCode::MOV, register: vec![Register::A,Register::B], operands: vec![] },
        0x79 => Instruction{ opcode: OpCode::MOV, register: vec![Register::A,Register::C], operands: vec![] },
        0x7a => Instruction{ opcode: OpCode::MOV, register: vec![Register::A,Register::D], operands: vec![] },
        0x7b => Instruction{ opcode: OpCode::MOV, register: vec![Register::A,Register::E], operands: vec![] },
        0x7c => Instruction{ opcode: OpCode::MOV, register: vec![Register::A,Register::H], operands: vec![] },
        0x7d => Instruction{ opcode: OpCode::MOV, register: vec![Register::A,Register::L], operands: vec![] },
        0x7e => Instruction{ opcode: OpCode::MOV, register: vec![Register::A,Register::M], operands: vec![] },
        0x7f => Instruction{ opcode: OpCode::MOV, register: vec![Register::A,Register::A], operands: vec![] },
        0x80 => Instruction{ opcode: OpCode::ADD, register: vec![Register::B], operands: vec![] },
        0x81 => Instruction{ opcode: OpCode::ADD, register: vec![Register::C], operands: vec![] },
        0x82 => Instruction{ opcode: OpCode::ADD, register: vec![Register::D], operands: vec![] },
        0x83 => Instruction{ opcode: OpCode::ADD, register: vec![Register::E], operands: vec![] },
        0x84 => Instruction{ opcode: OpCode::ADD, register: vec![Register::H], operands: vec![] },
        0x85 => Instruction{ opcode: OpCode::ADD, register: vec![Register::L], operands: vec![] },
        0x86 => Instruction{ opcode: OpCode::ADD, register: vec![Register::M], operands: vec![] },
        0x87 => Instruction{ opcode: OpCode::ADD, register: vec![Register::A], operands: vec![] },
        0x88 => Instruction{ opcode: OpCode::ADC, register: vec![Register::B], operands: vec![] },
        0x89 => Instruction{ opcode: OpCode::ADC, register: vec![Register::C], operands: vec![] },
        0x8a => Instruction{ opcode: OpCode::ADC, register: vec![Register::D], operands: vec![] },
        0x8b => Instruction{ opcode: OpCode::ADC, register: vec![Register::E], operands: vec![] },
        0x8c => Instruction{ opcode: OpCode::ADC, register: vec![Register::H], operands: vec![] },
        0x8d => Instruction{ opcode: OpCode::ADC, register: vec![Register::L], operands: vec![] },
        0x8e => Instruction{ opcode: OpCode::ADC, register: vec![Register::M], operands: vec![] },
        0x8f => Instruction{ opcode: OpCode::ADC, register: vec![Register::A], operands: vec![] },
        0x90 => Instruction{ opcode: OpCode::SUB, register: vec![Register::B], operands: vec![] },
        0x91 => Instruction{ opcode: OpCode::SUB, register: vec![Register::C], operands: vec![] },
        0x92 => Instruction{ opcode: OpCode::SUB, register: vec![Register::D], operands: vec![] },
        0x93 => Instruction{ opcode: OpCode::SUB, register: vec![Register::E], operands: vec![] },
        0x94 => Instruction{ opcode: OpCode::SUB, register: vec![Register::H], operands: vec![] },
        0x95 => Instruction{ opcode: OpCode::SUB, register: vec![Register::L], operands: vec![] },
        0x96 => Instruction{ opcode: OpCode::SUB, register: vec![Register::M], operands: vec![] },
        0x97 => Instruction{ opcode: OpCode::SUB, register: vec![Register::A], operands: vec![] },
        0x98 => Instruction{ opcode: OpCode::SBB, register: vec![Register::B], operands: vec![] },
        0x99 => Instruction{ opcode: OpCode::SBB, register: vec![Register::C], operands: vec![] },
        0x9a => Instruction{ opcode: OpCode::SBB, register: vec![Register::D], operands: vec![] },
        0x9b => Instruction{ opcode: OpCode::SBB, register: vec![Register::E], operands: vec![] },
        0x9c => Instruction{ opcode: OpCode::SBB, register: vec![Register::H], operands: vec![] },
        0x9d => Instruction{ opcode: OpCode::SBB, register: vec![Register::L], operands: vec![] },
        0x9e => Instruction{ opcode: OpCode::SBB, register: vec![Register::M], operands: vec![] },
        0x9f => Instruction{ opcode: OpCode::SBB, register: vec![Register::A], operands: vec![] },
        0xa0 => Instruction{ opcode: OpCode::ANA, register: vec![Register::B], operands: vec![] },
        0xa1 => Instruction{ opcode: OpCode::ANA, register: vec![Register::C], operands: vec![] },
        0xa2 => Instruction{ opcode: OpCode::ANA, register: vec![Register::D], operands: vec![] },
        0xa3 => Instruction{ opcode: OpCode::ANA, register: vec![Register::E], operands: vec![] },
        0xa4 => Instruction{ opcode: OpCode::ANA, register: vec![Register::H], operands: vec![] },
        0xa5 => Instruction{ opcode: OpCode::ANA, register: vec![Register::L], operands: vec![] },
        0xa6 => Instruction{ opcode: OpCode::ANA, register: vec![Register::M], operands: vec![] },
        0xa7 => Instruction{ opcode: OpCode::ANA, register: vec![Register::A], operands: vec![] },
        0xa8 => Instruction{ opcode: OpCode::XRA, register: vec![Register::B], operands: vec![] },
        0xa9 => Instruction{ opcode: OpCode::XRA, register: vec![Register::C], operands: vec![] },
        0xaa => Instruction{ opcode: OpCode::XRA, register: vec![Register::D], operands: vec![] },
        0xab => Instruction{ opcode: OpCode::XRA, register: vec![Register::E], operands: vec![] },
        0xac => Instruction{ opcode: OpCode::XRA, register: vec![Register::H], operands: vec![] },
        0xad => Instruction{ opcode: OpCode::XRA, register: vec![Register::L], operands: vec![] },
        0xae => Instruction{ opcode: OpCode::XRA, register: vec![Register::M], operands: vec![] },
        0xaf => Instruction{ opcode: OpCode::XRA, register: vec![Register::A], operands: vec![] },
        0xb0 => Instruction{ opcode: OpCode::ORA, register: vec![Register::B], operands: vec![] },
        0xb1 => Instruction{ opcode: OpCode::ORA, register: vec![Register::C], operands: vec![] },
        0xb2 => Instruction{ opcode: OpCode::ORA, register: vec![Register::D], operands: vec![] },
        0xb3 => Instruction{ opcode: OpCode::ORA, register: vec![Register::E], operands: vec![] },
        0xb4 => Instruction{ opcode: OpCode::ORA, register: vec![Register::H], operands: vec![] },
        0xb5 => Instruction{ opcode: OpCode::ORA, register: vec![Register::L], operands: vec![] },
        0xb6 => Instruction{ opcode: OpCode::ORA, register: vec![Register::M], operands: vec![] },
        0xb7 => Instruction{ opcode: OpCode::ORA, register: vec![Register::A], operands: vec![] },
        0xb8 => Instruction{ opcode: OpCode::CMP, register: vec![Register::B], operands: vec![] },
        0xb9 => Instruction{ opcode: OpCode::CMP, register: vec![Register::C], operands: vec![] },
        0xba => Instruction{ opcode: OpCode::CMP, register: vec![Register::D], operands: vec![] },
        0xbb => Instruction{ opcode: OpCode::CMP, register: vec![Register::E], operands: vec![] },
        0xbc => Instruction{ opcode: OpCode::CMP, register: vec![Register::H], operands: vec![] },
        0xbd => Instruction{ opcode: OpCode::CMP, register: vec![Register::L], operands: vec![] },
        0xbe => Instruction{ opcode: OpCode::CMP, register: vec![Register::M], operands: vec![] },
        0xbf => Instruction{ opcode: OpCode::CMP, register: vec![Register::A], operands: vec![] },
        0xc0 => Instruction{ opcode: OpCode::RNZ, register: vec![Register::B], operands: vec![] },
        0xc1 => Instruction{ opcode: OpCode::POP, register: vec![Register::B], operands: vec![] },
        0xc2 => Instruction{ opcode: OpCode::JNZ, register: vec![], operands: vec![buf[ix+2],buf[ix+1]] },
        0xc3 => Instruction{ opcode: OpCode::JMP, register: vec![], operands: vec![buf[ix+2],buf[ix+1]] },
        0xc4 => Instruction{ opcode: OpCode::CNZ, register: vec![], operands: vec![buf[ix+2],buf[ix+1]] },
        0xc5 => Instruction{ opcode: OpCode::PUSH, register: vec![Register::B], operands: vec![] },
        0xc6 => Instruction{ opcode: OpCode::ADI, register: vec![], operands: vec![buf[ix+1]] },
        0xc7 => Instruction{ opcode: OpCode::RST, register: vec![Register::_0], operands: vec![] },
        0xc8 => Instruction{ opcode: OpCode::RZ, register: vec![], operands: vec![] },
        0xc9 => Instruction{ opcode: OpCode::RET, register: vec![], operands: vec![] },
        0xca => Instruction{ opcode: OpCode::JZ, register: vec![], operands: vec![buf[ix+2],buf[ix+1]] },
        0xcb => Instruction{ opcode: OpCode::NOP, register: vec![], operands: vec![] },
        0xcc => Instruction{ opcode: OpCode::CZ, register: vec![], operands: vec![buf[ix+2],buf[ix+1]] },
        0xcd => Instruction{ opcode: OpCode::CALL, register: vec![], operands: vec![buf[ix+2],buf[ix+1]] },
        0xce => Instruction{ opcode: OpCode::ACI, register: vec![], operands: vec![buf[ix+1]] },
        0xcf => Instruction{ opcode: OpCode::RST, register: vec![Register::_1], operands: vec![] },
        0xd0 => Instruction{ opcode: OpCode::RNC, register: vec![], operands: vec![] },
        0xd1 => Instruction{ opcode: OpCode::POP, register: vec![Register::D], operands: vec![] },
        0xd2 => Instruction{ opcode: OpCode::JNC, register: vec![], operands: vec![buf[ix+2],buf[ix+1]] },
        0xd3 => Instruction{ opcode: OpCode::OUT, register: vec![], operands: vec![buf[ix+1]] },
        0xd4 => Instruction{ opcode: OpCode::CNC, register: vec![], operands: vec![buf[ix+2],buf[ix+1]] },
        0xd5 => Instruction{ opcode: OpCode::PUSH, register: vec![Register::D], operands: vec![] },
        0xd6 => Instruction{ opcode: OpCode::SUI, register: vec![], operands: vec![buf[ix+1]] },
        0xd7 => Instruction{ opcode: OpCode::RST, register: vec![Register::_2], operands: vec![] },
        0xd8 => Instruction{ opcode: OpCode::RC, register: vec![], operands: vec![] },
        0xd9 => Instruction{ opcode: OpCode::NOP, register: vec![], operands: vec![] },
        0xda => Instruction{ opcode: OpCode::JC, register: vec![], operands: vec![buf[ix+2],buf[ix+1]] },
        0xdb => Instruction{ opcode: OpCode::IN, register: vec![], operands: vec![buf[ix+1]] },
        0xdc => Instruction{ opcode: OpCode::CC, register: vec![], operands: vec![buf[ix+2],buf[ix+1]] },
        0xdd => Instruction{ opcode: OpCode::NOP, register: vec![], operands: vec![] },
        0xde => Instruction{ opcode: OpCode::SBI, register: vec![], operands: vec![buf[ix+1]] },
        0xdf => Instruction{ opcode: OpCode::RST, register: vec![Register::_3], operands: vec![] },
        0xe0 => Instruction{ opcode: OpCode::RPO, register: vec![], operands: vec![] },
        0xe1 => Instruction{ opcode: OpCode::POP, register: vec![Register::H], operands: vec![] },
        0xe2 => Instruction{ opcode: OpCode::JPO, register: vec![], operands: vec![buf[ix+2],buf[ix+1]] },
        0xe3 => Instruction{ opcode: OpCode::XTHL, register: vec![], operands: vec![] },
        0xe4 => Instruction{ opcode: OpCode::CPO, register: vec![], operands: vec![buf[ix+2],buf[ix+1]] },
        0xe5 => Instruction{ opcode: OpCode::PUSH, register: vec![Register::H], operands: vec![] },
        0xe6 => Instruction{ opcode: OpCode::ANI, register: vec![], operands: vec![buf[ix+1]] },
        0xe7 => Instruction{ opcode: OpCode::RST, register: vec![Register::_4], operands: vec![] },
        0xe8 => Instruction{ opcode: OpCode::RPE, register: vec![], operands: vec![] },
        0xe9 => Instruction{ opcode: OpCode::PCHL, register: vec![], operands: vec![] },
        0xea => Instruction{ opcode: OpCode::JPE, register: vec![], operands: vec![buf[ix+2],buf[ix+1]] },
        0xeb => Instruction{ opcode: OpCode::XCHG, register: vec![], operands: vec![] },
        0xec => Instruction{ opcode: OpCode::CPE, register: vec![], operands: vec![buf[ix+2],buf[ix+1]] },
        0xed => Instruction{ opcode: OpCode::NOP, register: vec![], operands: vec![] },
        0xee => Instruction{ opcode: OpCode::XRI, register: vec![], operands: vec![buf[ix+1]] },
        0xef => Instruction{ opcode: OpCode::RST, register: vec![Register::_5], operands: vec![] },
        0xf0 => Instruction{ opcode: OpCode::RP, register: vec![], operands: vec![] },
        0xf1 => Instruction{ opcode: OpCode::POP, register: vec![Register::PSW], operands: vec![] },
        0xf2 => Instruction{ opcode: OpCode::JP, register: vec![], operands: vec![buf[ix+2],buf[ix+1]] },
        0xf3 => Instruction{ opcode: OpCode::DI, register: vec![], operands: vec![] },
        0xf4 => Instruction{ opcode: OpCode::CP, register: vec![], operands: vec![buf[ix+2],buf[ix+1]] },
        0xf5 => Instruction{ opcode: OpCode::PUSH, register: vec![Register::PSW], operands: vec![] },
        0xf6 => Instruction{ opcode: OpCode::ORI, register: vec![], operands: vec![buf[ix+1]] },
        0xf7 => Instruction{ opcode: OpCode::RST, register: vec![Register::_6], operands: vec![] },
        0xf8 => Instruction{ opcode: OpCode::RM, register: vec![], operands: vec![] },
        0xf9 => Instruction{ opcode: OpCode::SPHL, register: vec![], operands: vec![] },
        0xfa => Instruction{ opcode: OpCode::JM, register: vec![], operands: vec![buf[ix+2],buf[ix+1]] },
        0xfb => Instruction{ opcode: OpCode::EI, register: vec![], operands: vec![] },
        0xfc => Instruction{ opcode: OpCode::CM, register: vec![], operands: vec![buf[ix+2],buf[ix+1]] },
        0xfd => Instruction{ opcode: OpCode::NOP, register: vec![], operands: vec![] },
        0xfe => Instruction{ opcode: OpCode::CPI, register: vec![], operands: vec![buf[ix+1]] },
        0xff => Instruction{ opcode: OpCode::RST, register: vec![Register::_7], operands: vec![] },
    }
}