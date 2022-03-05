// #[derive(Copy, Clone, Eq, PartialEq)]
// pub struct BinaryInstruction(u8);
// impl fmt::Debug for BinaryInstruction {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         f.write_fmt(format_args!("'{:X}'", self.0))
//     }
// }

// impl From<u8> for BinaryInstruction {
//     fn from(ins: u8) -> Self {
//         Self(ins)
//     }
// }

// impl From<BinaryInstruction> for u8 {
//     fn from(ins: BinaryInstruction) -> u8 {
//         ins.0
//     }
// }

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Instruction {
    Select,
    GetData,
    Verify,
    ChangeReferenceData,
    ResetRetryCounter,
    GeneralAuthenticate,
    PutData,
    GenerateAsymmetricKeyPair,
    GetResponse,
    ReadBinary,
    WriteBinary,
    // Unknown(BinaryInstruction),
    Unknown(u8),
}

pub struct UnknownInstruction {}

impl From<u8> for Instruction {
    fn from(ins: u8) -> Self {
        match ins {
            0x20 => Instruction::Verify,
            0x24 => Instruction::ChangeReferenceData,
            0x2c => Instruction::ResetRetryCounter,
            0x47 => Instruction::GenerateAsymmetricKeyPair,
            0x87 => Instruction::GeneralAuthenticate,
            0xa4 => Instruction::Select,
            0xc0 => Instruction::GetResponse,
            0xcb => Instruction::GetData,
            0xdb => Instruction::PutData,
            0xb0 => Instruction::ReadBinary,
            0xd0 => Instruction::WriteBinary,
            ins => Instruction::Unknown(ins),
        }
    }
}

impl From<Instruction> for u8 {
    fn from(instruction: Instruction) -> u8 {
        match instruction {
            Instruction::Verify => 0x20,
            Instruction::ChangeReferenceData => 0x24,
            Instruction::ResetRetryCounter => 0x2c,
            Instruction::GenerateAsymmetricKeyPair => 0x47,
            Instruction::GeneralAuthenticate => 0x87,
            Instruction::Select => 0xa4,
            Instruction::GetResponse => 0xc0,
            Instruction::GetData => 0xcb,
            Instruction::PutData => 0xdb,
            Instruction::ReadBinary => 0xb0,
            Instruction::WriteBinary => 0xd0,
            Instruction::Unknown(ins) => ins,
        }
    }
}

// impl TryFrom<u8> for Instruction {
//     type Error = UnknownInstruction;

//     fn try_from(ins: u8) -> Result<Self, Self::Error> {
//         let instruction = match ins {
//             0x20 => Instruction::Verify,
//             0x24 => Instruction::ChangeReferenceData,
//             0x2c => Instruction::ResetRetryCounter,
//             0x47 => Instruction::GenerateAsymmetricKeyPair,
//             0x87 => Instruction::GeneralAuthenticate,
//             0xa4 => Instruction::Select,
//             0xc0 => Instruction::GetResponse,
//             0xcb => Instruction::GetData,
//             0xdb => Instruction::PutData,
//             _ => return Instruction::Unknown(ins),
//             Err(UnknownInstruction {})
//         };

//         Ok(instruction)
//     }
// }
