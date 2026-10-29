//! Operation definition for vm and assembler

/// The instructions known to the CiviC VM
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Instr {
    /// `IADD` instruction
    IntAdd,
    /// `ISUB` instruction
    IntSub,
    /// `IMUL` instruction
    IntMul,
    /// `IDIV` instruction
    IntDiv,
    /// `IREM` instruction
    IntRem,
    /// `INEG` instruction
    IntNeg,
    /// `INEWA` instruction
    IntNewArray,
    /// `IPOP` instruction
    IntPop,

    /// `INEQ` instruction
    IntNotEq,
    /// `IEQ` instruction
    IntEq,
    /// `ILT` instruction
    IntLess,
    /// `ILE` instruction
    IntLessEqual,
    /// `IGT` instruction
    IntGreater,
    /// `IGE` instruction
    IntGreaterEqual,
    /// `I2F` instruction
    IntToFloat,
    /// `IRETURN` instruction
    IntReturn,

    /// `ILOAD_0` instruction
    IntLoadLocal0,
    /// `ILOAD_1` instruction
    IntLoadLocal1,
    /// `ILOAD_2` instruction
    IntLoadLocal2,
    /// `ILOAD_3` instruction
    IntLoadLocal3,
    /// `ILOADC_0` instruction
    IntLoad0,
    /// `ILOADC_1` instruction
    IntLoad1,
    /// `ILOADC_M1` instruction
    IntLoadM1,
    /// `ILOADC` instruction
    IntLoadConstant,

    /// `ILOAD` instruction
    IntLoadLocal,
    /// `ILOADN` instruction
    IntLoadHigherLocal,
    /// `ILOADG` instruction
    IntLoadGlobal,
    /// `ILOADE` instruction
    IntLoadExternal,
    /// `ILOADA` instruction
    IntLoadFromArray,
    /// `ISTORE` instruction
    IntStoreLocal,
    /// `ISTOREN` instruction
    IntStoreHigherLocal,
    /// `ISTOREG` instruction
    IntStoreGlobal,
    /// `ISTOREE` instruction
    IntStoreExternal,
    /// `ISTOREA` instruction
    IntStoreToArray,

    /// `FADD` instruction
    FloatAdd,
    /// `FSUB` instruction
    FloatSub,
    /// `FMUL` instruction
    FloatMul,
    /// `FDIV` instruction
    FloatDiv,
    /// `FNEG` instruction
    FloatNeg,
    /// `FNEWA` instruction
    FloatNewArray,
    /// `FPOP` instruction
    FloatPop,

    /// `FNEQ` instruction
    FloatNotEq,
    /// `FEQ` instruction
    FloatEq,
    /// `FLT` instruction
    FloatLess,
    /// `FLE` instruction
    FloatLessEqual,
    /// `FGT` instruction
    FloatGreater,
    /// `FGE` instruction
    FloatGreaterEqual,
    /// `F2I` instruction
    FloatToInt,
    /// `FRETURN` instruction
    FloatReturn,

    /// `FLOAD_0` instruction
    FloatLoadLocal0,
    /// `FLOAD_1` instruction
    FloatLoadLocal1,
    /// `FLOAD_2` instruction
    FloatLoadLocal2,
    /// `FLOAD_3` instruction
    FloatLoadLocal3,
    /// `FLOADC_0` instruction
    FloatLoad0,
    /// `FLOADC_1` instruction
    FloatLoad1,
    /// `FLOADC` instruction
    FloatLoadConstant,

    /// `FLOAD` instruction
    FloatLoadLocal,
    /// `FLOADN` instruction
    FloatLoadHigherLocal,
    /// `FLOADG` instruction
    FloatLoadGlobal,
    /// `FLOADE` instruction
    FloatLoadExternal,
    /// `FLOADA` instruction
    FloatLoadFromArray,
    /// `FSTORE` instruction
    FloatStoreLocal,
    /// `FSTOREN` instruction
    FloatStoreHigherLocal,
    /// `FSTOREG` instruction
    FloatStoreGlobal,
    /// `FSTOREE` instruction
    FloatStoreExternal,
    /// `FSTOREA` instruction
    FloatStoreToArray,

    /// `BADD` instruction
    BoolAdd,
    /// `BMUL` instruction
    BitMul,
    /// `BNOT` instruction
    BoolNot,
    /// `BNEWA` instruction
    BoolNewArray,
    /// `BPOP` instruction
    BoolPop,
    /// `BNEQ` instruction
    BoolNotEq,
    /// `BEQ` instruction
    BoolEq,
    /// `BRETURN` instruction
    BoolReturn,

    /// `BLOAD_0` instruction
    BoolLoadLocal0,
    /// `BLOAD_1` instruction
    BoolLoadLocal1,
    /// `BLOAD_2` instruction
    BoolLoadLocal2,
    /// `BLOAD_3` instruction
    BoolLoadLocal3,
    /// `BLOADC_T` instruction
    BoolLoadTrue,
    /// `BLOADC_F` instruction
    BoolLoadFalse,
    /// `BLOADC` instruction
    BoolLoadConstant,

    /// `BLOAD` instruction
    BoolLoadLocal,
    /// `BLOADN` instruction
    BoolLoadHigherLocal,
    /// `BLOADG` instruction
    BoolLoadGlobal,
    /// `BLOADE` instruction
    BoolLoadExternal,
    /// `BLOADA` instruction
    BoolLoadFromArray,
    /// `BSTORE` instruction
    BoolStoreLocal,
    /// `BSTOREN` instruction
    BoolStoreHigherLocal,
    /// `BSTOREG` instruction
    BoolStoreGlobal,
    /// `BSTOREE` instruction
    BoolStoreExternal,
    /// `BSTOREA` instruction
    BoolStoreToArray,

    /// `IINC` instruction
    IntIncr,
    /// `IINC_1` instruction
    IntIncrBy1,
    /// `IDEC` instruction
    IntDecr,
    /// `IDEC_1` instruction
    IntDecrBy1,

    /// `JUMP` instruction
    Jump,
    /// `BRANCH_T` instruction
    BranchTrue,
    /// `BRANCH_F` instruction
    BranchFalse,

    /// `ISR` instruction
    InitiateLocalSubRoutine,
    /// `ISRN` instruction
    InitiateHigherLocalSubRoutine,
    /// `ISRL` instruction
    IntiateBelowLocalSubRoutine,
    /// `ISRG` instruction
    InitiateGlobalSubRoutine,
    /// `ESR` instruction
    EnterSubRoutine,
    /// `JSR` instruction
    JumpToSubRoutine,
    /// `JSRE` instruction
    JumpToExternalSubRoutine,
    /// `RETURN` instruction
    Return,

    /// `ALOAD_0` instruction
    ArrayLoadLocal0,
    /// `ALOAD_1` instruction
    ArrayLoadLocal1,
    /// `ALOAD_2` instruction
    ArrayLoadLocal2,
    /// `ALOAD_3` instruction
    ArrayLoadLocal3,

    /*/// `ASIZE` instruction
    ArraySize,*/
    /// `ALOAD` instruction
    ArrayLoadLocal,
    /// `ALOADN` instruction
    ArrayLoadHigherLocal,
    /// `ALOADG` instruction
    ArrayLoadGlobal,
    /// `ALOADE` instruction
    ArrayLoadExternal,
    /// `ASTORE` instruction
    ArrayStoreLocal,
    /// `ASTOREG` instruction
    ArrayStoreGlobal,
    /// `ASTOREE` instruction
    ArrayStoreExternal,
}

impl Instr {
    pub fn operands<T: ToString>(&self) -> [OperandType<T>; 2] {
        match self {
            Instr::IntAdd => [OperandType::None, OperandType::None],
            Instr::IntSub => [OperandType::None, OperandType::None],
            Instr::IntMul => [OperandType::None, OperandType::None],
            Instr::IntDiv => [OperandType::None, OperandType::None],
            Instr::IntRem => [OperandType::None, OperandType::None],
            Instr::IntNeg => [OperandType::None, OperandType::None],
            Instr::IntNewArray => [OperandType::None, OperandType::None],
            Instr::IntPop => [OperandType::None, OperandType::None],
            Instr::IntNotEq => [OperandType::None, OperandType::None],
            Instr::IntEq => [OperandType::None, OperandType::None],
            Instr::IntLess => [OperandType::None, OperandType::None],
            Instr::IntLessEqual => [OperandType::None, OperandType::None],
            Instr::IntGreater => [OperandType::None, OperandType::None],
            Instr::IntGreaterEqual => [OperandType::None, OperandType::None],
            Instr::IntToFloat => [OperandType::None, OperandType::None],
            Instr::IntReturn => [OperandType::None, OperandType::None],
            Instr::IntLoadLocal0 => [OperandType::None, OperandType::None],
            Instr::IntLoadLocal1 => [OperandType::None, OperandType::None],
            Instr::IntLoadLocal2 => [OperandType::None, OperandType::None],
            Instr::IntLoadLocal3 => [OperandType::None, OperandType::None],
            Instr::IntLoad0 => [OperandType::None, OperandType::None],
            Instr::IntLoad1 => [OperandType::None, OperandType::None],
            Instr::IntLoadM1 => [OperandType::None, OperandType::None],
            Instr::IntLoadConstant => [OperandType::ConstantIndex(None), OperandType::None],
            Instr::IntLoadLocal => [OperandType::Local(None), OperandType::None],
            Instr::IntLoadHigherLocal => [OperandType::LinkCount(None), OperandType::Local(None)],
            Instr::IntLoadGlobal => [OperandType::GlobalIndex(None), OperandType::None],
            Instr::IntLoadFromArray => [OperandType::None, OperandType::None],
            Instr::IntStoreLocal => [OperandType::Local(None), OperandType::None],
            Instr::IntStoreHigherLocal => [OperandType::LinkCount(None), OperandType::Local(None)],
            Instr::IntStoreGlobal => [OperandType::GlobalIndex(None), OperandType::None],
            Instr::IntStoreToArray => [OperandType::None, OperandType::None],
            Instr::FloatAdd => [OperandType::None, OperandType::None],
            Instr::FloatSub => [OperandType::None, OperandType::None],
            Instr::FloatMul => [OperandType::None, OperandType::None],
            Instr::FloatDiv => [OperandType::None, OperandType::None],
            Instr::FloatNeg => [OperandType::None, OperandType::None],
            Instr::FloatNewArray => [OperandType::None, OperandType::None],
            Instr::FloatPop => [OperandType::None, OperandType::None],
            Instr::FloatNotEq => [OperandType::None, OperandType::None],
            Instr::FloatEq => [OperandType::None, OperandType::None],
            Instr::FloatLess => [OperandType::None, OperandType::None],
            Instr::FloatLessEqual => [OperandType::None, OperandType::None],
            Instr::FloatGreater => [OperandType::None, OperandType::None],
            Instr::FloatGreaterEqual => [OperandType::None, OperandType::None],
            Instr::FloatToInt => [OperandType::None, OperandType::None],
            Instr::FloatReturn => [OperandType::None, OperandType::None],
            Instr::FloatLoadLocal0 => [OperandType::None, OperandType::None],
            Instr::FloatLoadLocal1 => [OperandType::None, OperandType::None],
            Instr::FloatLoadLocal2 => [OperandType::None, OperandType::None],
            Instr::FloatLoadLocal3 => [OperandType::None, OperandType::None],
            Instr::FloatLoad0 => [OperandType::None, OperandType::None],
            Instr::FloatLoad1 => [OperandType::None, OperandType::None],
            Instr::FloatLoadConstant => [OperandType::ConstantIndex(None), OperandType::None],
            Instr::FloatLoadLocal => [OperandType::Local(None), OperandType::None],
            Instr::FloatLoadHigherLocal => [OperandType::LinkCount(None), OperandType::Local(None)],
            Instr::FloatLoadGlobal => [OperandType::GlobalIndex(None), OperandType::None],
            Instr::FloatLoadFromArray => [OperandType::None, OperandType::None],
            Instr::FloatStoreLocal => [OperandType::Local(None), OperandType::None],
            Instr::FloatStoreHigherLocal => [OperandType::LinkCount(None), OperandType::Local(None)],
            Instr::FloatStoreGlobal => [OperandType::GlobalIndex(None), OperandType::None],
            Instr::FloatStoreToArray => [OperandType::None, OperandType::None],
            Instr::IntLoadExternal => [OperandType::ImportIndex(None), OperandType::None],
            Instr::FloatLoadExternal => [OperandType::ImportIndex(None), OperandType::None],
            Instr::BoolLoadExternal => [OperandType::ImportIndex(None), OperandType::None],
            Instr::ArrayLoadExternal => [OperandType::ImportIndex(None), OperandType::None],
            Instr::BoolAdd => [OperandType::None, OperandType::None],
            Instr::BoolNot => [OperandType::None, OperandType::None],
            Instr::BoolNewArray => [OperandType::None, OperandType::None],
            Instr::BoolPop => [OperandType::None, OperandType::None],
            Instr::BoolNotEq => [OperandType::None, OperandType::None],
            Instr::BoolEq => [OperandType::None, OperandType::None],
            Instr::IntStoreExternal => [OperandType::ImportIndex(None), OperandType::None],
            Instr::FloatStoreExternal => [OperandType::ImportIndex(None), OperandType::None],
            Instr::BoolStoreExternal => [OperandType::ImportIndex(None), OperandType::None],
            Instr::ArrayStoreExternal => [OperandType::ImportIndex(None), OperandType::None],
            Instr::BitMul => [OperandType::None, OperandType::None],
            Instr::BoolReturn => [OperandType::None, OperandType::None],
            Instr::BoolLoadLocal0 => [OperandType::None, OperandType::None],
            Instr::BoolLoadLocal1 => [OperandType::None, OperandType::None],
            Instr::BoolLoadLocal2 => [OperandType::None, OperandType::None],
            Instr::BoolLoadLocal3 => [OperandType::None, OperandType::None],
            Instr::BoolLoadTrue => [OperandType::None, OperandType::None],
            Instr::BoolLoadFalse => [OperandType::None, OperandType::None],
            Instr::BoolLoadConstant => [OperandType::ConstantIndex(None), OperandType::None],
            Instr::BoolLoadLocal => [OperandType::Local(None), OperandType::None],
            Instr::BoolLoadHigherLocal => [OperandType::LinkCount(None), OperandType::Local(None)],
            Instr::BoolLoadGlobal => [OperandType::GlobalIndex(None), OperandType::None],
            Instr::BoolLoadFromArray => [OperandType::None, OperandType::None],
            Instr::BoolStoreLocal => [OperandType::Local(None), OperandType::None],
            Instr::BoolStoreHigherLocal => [OperandType::LinkCount(None), OperandType::Local(None)],
            Instr::BoolStoreGlobal => [OperandType::GlobalIndex(None), OperandType::None],
            Instr::BoolStoreToArray => [OperandType::None, OperandType::None],
            Instr::IntIncr => [OperandType::Local(None), OperandType::ConstantIndex(None)],
            Instr::IntIncrBy1 => [OperandType::Local(None), OperandType::None],
            Instr::IntDecr => [OperandType::Local(None), OperandType::ConstantIndex(None)],
            Instr::IntDecrBy1 => [OperandType::Local(None), OperandType::None],
            Instr::Jump => [OperandType::Offset(None), OperandType::None],
            Instr::BranchTrue => [OperandType::Offset(None), OperandType::None],
            Instr::BranchFalse => [OperandType::Offset(None), OperandType::None],
            Instr::InitiateLocalSubRoutine => [OperandType::None, OperandType::None],
            Instr::InitiateHigherLocalSubRoutine => [OperandType::LinkCount(None), OperandType::None],
            Instr::IntiateBelowLocalSubRoutine => [OperandType::None, OperandType::None],
            Instr::InitiateGlobalSubRoutine => [OperandType::None, OperandType::None],
            Instr::EnterSubRoutine => [OperandType::Local(None), OperandType::None],
            Instr::JumpToSubRoutine => [OperandType::ArgumentCount(None), OperandType::Offset(None)],
            Instr::JumpToExternalSubRoutine => [OperandType::ImportIndex(None), OperandType::None],
            Instr::Return => [OperandType::None, OperandType::None],
            Instr::ArrayLoadLocal0 => [OperandType::None, OperandType::None],
            Instr::ArrayLoadLocal1 => [OperandType::None, OperandType::None],
            Instr::ArrayLoadLocal2 => [OperandType::None, OperandType::None],
            Instr::ArrayLoadLocal3 => [OperandType::None, OperandType::None],
            Instr::ArrayLoadLocal => [OperandType::Local(None), OperandType::None],
            Instr::ArrayLoadHigherLocal => [OperandType::LinkCount(None), OperandType::Local(None)],
            Instr::ArrayLoadGlobal => [OperandType::GlobalIndex(None), OperandType::None],
            Instr::ArrayStoreLocal => [OperandType::Local(None), OperandType::None],
            Instr::ArrayStoreGlobal => [OperandType::GlobalIndex(None), OperandType::None],
        }
    }
}

impl From<Instr> for u8 {
    fn from(value: Instr) -> Self {
        match value {
            Instr::IntAdd => 0x00,
            Instr::IntSub => 0x01,
            Instr::IntMul => 0x02,
            Instr::IntDiv => 0x03,
            Instr::IntRem => 0x04,
            Instr::IntNeg => 0x05,
            Instr::IntNewArray => 0x06,
            Instr::IntPop => 0x07,
            Instr::IntNotEq => 0x08,
            Instr::IntEq => 0x09,
            Instr::IntLess => 0x0A,
            Instr::IntLessEqual => 0x0B,
            Instr::IntGreater => 0x0C,
            Instr::IntGreaterEqual => 0x0D,
            Instr::IntToFloat => 0x0E,
            Instr::IntReturn => 0x0F,

            Instr::IntLoadLocal0 => 0x10,
            Instr::IntLoadLocal1 => 0x11,
            Instr::IntLoadLocal2 => 0x12,
            Instr::IntLoadLocal3 => 0x13,
            Instr::IntLoad0 => 0x14,
            Instr::IntLoad1 => 0x15,
            Instr::IntLoadM1 => 0x16,
            Instr::IntLoadConstant => 0x17,
            Instr::IntLoadLocal => 0x18,
            Instr::IntLoadHigherLocal => 0x19,
            Instr::IntLoadGlobal => 0x1A,
            Instr::IntLoadFromArray => 0x1B,
            Instr::IntStoreLocal => 0x1C,
            Instr::IntStoreHigherLocal => 0x1D,
            Instr::IntStoreGlobal => 0x1E,
            Instr::IntStoreToArray => 0x1F,

            Instr::FloatAdd => 0x20,
            Instr::FloatSub => 0x21,
            Instr::FloatMul => 0x22,
            Instr::FloatDiv => 0x23,
            //::None => 0x24,
            Instr::FloatNeg => 0x25,
            Instr::FloatNewArray => 0x26,
            Instr::FloatPop => 0x27,
            Instr::FloatNotEq => 0x28,
            Instr::FloatEq => 0x29,
            Instr::FloatLess => 0x2A,
            Instr::FloatLessEqual => 0x2B,
            Instr::FloatGreater => 0x2C,
            Instr::FloatGreaterEqual => 0x2D,
            Instr::FloatToInt => 0x2E,
            Instr::FloatReturn => 0x2F,

            Instr::FloatLoadLocal0 => 0x30,
            Instr::FloatLoadLocal1 => 0x31,
            Instr::FloatLoadLocal2 => 0x32,
            Instr::FloatLoadLocal3 => 0x33,
            Instr::FloatLoad0 => 0x34,
            Instr::FloatLoad1 => 0x35,
            //::None => 0x36,
            Instr::FloatLoadConstant => 0x37,
            Instr::FloatLoadLocal => 0x38,
            Instr::FloatLoadHigherLocal => 0x39,
            Instr::FloatLoadGlobal => 0x3A,
            Instr::FloatLoadFromArray => 0x3B,
            Instr::FloatStoreLocal => 0x3C,
            Instr::FloatStoreHigherLocal => 0x3D,
            Instr::FloatStoreGlobal => 0x3E,
            Instr::FloatStoreToArray => 0x3F,

            Instr::IntLoadExternal => 0x40,
            Instr::FloatLoadExternal => 0x41,
            Instr::BoolLoadExternal => 0x42,
            Instr::ArrayLoadExternal => 0x43,
            Instr::BoolAdd => 0x44,
            Instr::BoolNot => 0x45,
            Instr::BoolNewArray => 0x46,
            Instr::BoolPop => 0x47,
            Instr::BoolNotEq => 0x48,
            Instr::BoolEq => 0x49,
            Instr::IntStoreExternal => 0x4A,
            Instr::FloatStoreExternal => 0x4B,
            Instr::BoolStoreExternal => 0x4C,
            Instr::ArrayStoreExternal => 0x4D,
            Instr::BitMul => 0x4E,
            Instr::BoolReturn => 0x4F,

            Instr::BoolLoadLocal0 => 0x50,
            Instr::BoolLoadLocal1 => 0x51,
            Instr::BoolLoadLocal2 => 0x52,
            Instr::BoolLoadLocal3 => 0x53,
            Instr::BoolLoadTrue => 0x54,
            Instr::BoolLoadFalse => 0x55,
            //::None => 0x56,
            Instr::BoolLoadConstant => 0x57,
            Instr::BoolLoadLocal => 0x58,
            Instr::BoolLoadHigherLocal => 0x59,
            Instr::BoolLoadGlobal => 0x5A,
            Instr::BoolLoadFromArray => 0x5B,
            Instr::BoolStoreLocal => 0x5C,
            Instr::BoolStoreHigherLocal => 0x5D,
            Instr::BoolStoreGlobal => 0x5E,
            Instr::BoolStoreToArray => 0x5F,

            Instr::IntIncr => 0x60,
            Instr::IntIncrBy1 => 0x61,
            Instr::IntDecr => 0x62,
            Instr::IntDecrBy1 => 0x63,
            Instr::Jump => 0x64,
            Instr::BranchTrue => 0x65,
            Instr::BranchFalse => 0x66,
            //::None => 0x67,
            Instr::InitiateLocalSubRoutine => 0x68,
            Instr::InitiateHigherLocalSubRoutine => 0x69,
            Instr::IntiateBelowLocalSubRoutine => 0x6A,
            Instr::InitiateGlobalSubRoutine => 0x6B,
            Instr::EnterSubRoutine => 0x6C,
            Instr::JumpToSubRoutine => 0x6D,
            Instr::JumpToExternalSubRoutine => 0x6E,
            Instr::Return => 0x6F,

            Instr::ArrayLoadLocal0 => 0x70,
            Instr::ArrayLoadLocal1 => 0x71,
            Instr::ArrayLoadLocal2 => 0x72,
            Instr::ArrayLoadLocal3 => 0x73,
            //Instr::None => 0x74,
            //Instr::None => 0x75,
            //Instr::None => 0x76,
            //Instr::ArraySize => 0x77,
            Instr::ArrayLoadLocal => 0x78,
            Instr::ArrayLoadHigherLocal => 0x79,
            Instr::ArrayLoadGlobal => 0x7A,
            //Instr::None => 0x7B,
            Instr::ArrayStoreLocal => 0x7C,
            //Instr::None => 0x7D,
            Instr::ArrayStoreGlobal => 0x7E,
            //Instr::None => 0x7F,
        }
    }
}

impl From<&Instr> for u8 {
    #[inline]
    fn from(value: &Instr) -> Self {
        (*value).into()
    }
}

impl TryFrom<u8> for Instr {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, String> {
        match value {
            0x00 => Ok(Instr::IntAdd),
            0x01 => Ok(Instr::IntSub),
            0x02 => Ok(Instr::IntMul),
            0x03 => Ok(Instr::IntDiv),
            0x04 => Ok(Instr::IntRem),
            0x05 => Ok(Instr::IntNeg),
            0x06 => Ok(Instr::IntNewArray),
            0x07 => Ok(Instr::IntPop),
            0x08 => Ok(Instr::IntNotEq),
            0x09 => Ok(Instr::IntEq),
            0x0A => Ok(Instr::IntLess),
            0x0B => Ok(Instr::IntLessEqual),
            0x0C => Ok(Instr::IntGreater),
            0x0D => Ok(Instr::IntGreaterEqual),
            0x0E => Ok(Instr::IntToFloat),
            0x0F => Ok(Instr::IntReturn),

            0x10 => Ok(Instr::IntLoadLocal0),
            0x11 => Ok(Instr::IntLoadLocal1),
            0x12 => Ok(Instr::IntLoadLocal2),
            0x13 => Ok(Instr::IntLoadLocal3),
            0x14 => Ok(Instr::IntLoad0),
            0x15 => Ok(Instr::IntLoad1),
            0x16 => Ok(Instr::IntLoadM1),
            0x17 => Ok(Instr::IntLoadConstant),
            0x18 => Ok(Instr::IntLoadLocal),
            0x19 => Ok(Instr::IntLoadHigherLocal),
            0x1A => Ok(Instr::IntLoadGlobal),
            0x1B => Ok(Instr::IntLoadFromArray),
            0x1C => Ok(Instr::IntStoreLocal),
            0x1D => Ok(Instr::IntStoreHigherLocal),
            0x1E => Ok(Instr::IntStoreGlobal),
            0x1F => Ok(Instr::IntStoreToArray),

            0x20 => Ok(Instr::FloatAdd),
            0x21 => Ok(Instr::FloatSub),
            0x22 => Ok(Instr::FloatMul),
            0x23 => Ok(Instr::FloatDiv),
            //	0x24 => Ok(Instr::None),
            0x25 => Ok(Instr::FloatNeg),
            0x26 => Ok(Instr::FloatNewArray),
            0x27 => Ok(Instr::FloatPop),
            0x28 => Ok(Instr::FloatNotEq),
            0x29 => Ok(Instr::FloatEq),
            0x2A => Ok(Instr::FloatLess),
            0x2B => Ok(Instr::FloatLessEqual),
            0x2C => Ok(Instr::FloatGreater),
            0x2D => Ok(Instr::FloatGreaterEqual),
            0x2E => Ok(Instr::FloatToInt),
            0x2F => Ok(Instr::FloatReturn),

            0x30 => Ok(Instr::FloatLoadLocal0),
            0x31 => Ok(Instr::FloatLoadLocal1),
            0x32 => Ok(Instr::FloatLoadLocal2),
            0x33 => Ok(Instr::FloatLoadLocal3),
            0x34 => Ok(Instr::FloatLoad0),
            0x35 => Ok(Instr::FloatLoad1),
            //	0x36 => Ok(Instr::None),
            0x37 => Ok(Instr::FloatLoadConstant),
            0x38 => Ok(Instr::FloatLoadLocal),
            0x39 => Ok(Instr::FloatLoadHigherLocal),
            0x3A => Ok(Instr::FloatLoadGlobal),
            0x3B => Ok(Instr::FloatLoadFromArray),
            0x3C => Ok(Instr::FloatStoreLocal),
            0x3D => Ok(Instr::FloatStoreHigherLocal),
            0x3E => Ok(Instr::FloatStoreGlobal),
            0x3F => Ok(Instr::FloatStoreToArray),

            0x40 => Ok(Instr::IntLoadExternal),
            0x41 => Ok(Instr::FloatLoadExternal),
            0x42 => Ok(Instr::BoolLoadExternal),
            0x43 => Ok(Instr::ArrayLoadExternal),
            0x44 => Ok(Instr::BoolAdd),
            0x45 => Ok(Instr::BoolNot),
            0x46 => Ok(Instr::BoolNewArray),
            0x47 => Ok(Instr::BoolPop),
            0x48 => Ok(Instr::BoolNotEq),
            0x49 => Ok(Instr::BoolEq),
            0x4A => Ok(Instr::IntStoreExternal),
            0x4B => Ok(Instr::FloatStoreExternal),
            0x4C => Ok(Instr::BoolStoreExternal),
            0x4D => Ok(Instr::ArrayStoreExternal),
            0x4E => Ok(Instr::BitMul),
            0x4F => Ok(Instr::BoolReturn),

            0x50 => Ok(Instr::BoolLoadLocal0),
            0x51 => Ok(Instr::BoolLoadLocal1),
            0x52 => Ok(Instr::BoolLoadLocal2),
            0x53 => Ok(Instr::BoolLoadLocal3),
            0x54 => Ok(Instr::BoolLoadTrue),
            0x55 => Ok(Instr::BoolLoadFalse),
            //	0x56 => Ok(Instr::None),
            0x57 => Ok(Instr::BoolLoadConstant),
            0x58 => Ok(Instr::BoolLoadLocal),
            0x59 => Ok(Instr::BoolLoadHigherLocal),
            0x5A => Ok(Instr::BoolLoadGlobal),
            0x5B => Ok(Instr::BoolLoadFromArray),
            0x5C => Ok(Instr::BoolStoreLocal),
            0x5D => Ok(Instr::BoolStoreHigherLocal),
            0x5E => Ok(Instr::BoolStoreGlobal),
            0x5F => Ok(Instr::BoolStoreToArray),

            0x60 => Ok(Instr::IntIncr),
            0x61 => Ok(Instr::IntIncrBy1),
            0x62 => Ok(Instr::IntDecr),
            0x63 => Ok(Instr::IntDecrBy1),
            0x64 => Ok(Instr::Jump),
            0x65 => Ok(Instr::BranchTrue),
            0x66 => Ok(Instr::BranchFalse),
            //	0x67 => Ok(Instr::None),
            0x68 => Ok(Instr::InitiateLocalSubRoutine),
            0x69 => Ok(Instr::InitiateHigherLocalSubRoutine),
            0x6A => Ok(Instr::IntiateBelowLocalSubRoutine),
            0x6B => Ok(Instr::InitiateGlobalSubRoutine),
            0x6C => Ok(Instr::EnterSubRoutine),
            0x6D => Ok(Instr::JumpToSubRoutine),
            0x6E => Ok(Instr::JumpToExternalSubRoutine),
            0x6F => Ok(Instr::Return),

            0x70 => Ok(Instr::ArrayLoadLocal0),
            0x71 => Ok(Instr::ArrayLoadLocal1),
            0x72 => Ok(Instr::ArrayLoadLocal2),
            0x73 => Ok(Instr::ArrayLoadLocal3),
            //	0x74 => Ok(Instr::None),
            //	0x75 => Ok(Instr::None),
            //	0x76 => Ok(Instr::None),
            //	0x77 => Ok(Instr::ArraySize),
            0x78 => Ok(Instr::ArrayLoadLocal),
            0x79 => Ok(Instr::ArrayLoadHigherLocal),
            0x7A => Ok(Instr::ArrayLoadGlobal),
            //	0x7B => Ok(Instr::None),
            0x7C => Ok(Instr::ArrayStoreLocal),
            //	0x7D => Ok(Instr::None),
            0x7E => Ok(Instr::ArrayStoreGlobal),
            //	0x7F => Ok(Instr::None),
            _ => Err(format!("Unknown : {}", value)),
        }
    }
}

impl ToString for Instr {
    fn to_string(&self) -> String {
        match self {
            Instr::IntAdd => "iadd",
            Instr::IntSub => "isub",
            Instr::IntMul => "imul",
            Instr::IntDiv => "idiv",
            Instr::IntRem => "irem",
            Instr::IntNeg => "ineg",
            Instr::IntNewArray => "inewa",
            Instr::IntPop => "ipop",
            Instr::IntNotEq => "ine",
            Instr::IntEq => "ieq",
            Instr::IntLess => "ilt",
            Instr::IntLessEqual => "ile",
            Instr::IntGreater => "igt",
            Instr::IntGreaterEqual => "ige",
            Instr::IntToFloat => "i2f",
            Instr::IntReturn => "ireturn",

            Instr::IntLoadLocal0 => "iload_0",
            Instr::IntLoadLocal1 => "iload_1",
            Instr::IntLoadLocal2 => "iload_2",
            Instr::IntLoadLocal3 => "iload_3",
            Instr::IntLoad0 => "iloadc_0",
            Instr::IntLoad1 => "iloadc_1",
            Instr::IntLoadM1 => "iloadc_m1",
            Instr::IntLoadConstant => "iloadc",
            Instr::IntLoadLocal => "iload",
            Instr::IntLoadHigherLocal => "iloadn",
            Instr::IntLoadGlobal => "iloadg",
            Instr::IntLoadFromArray => "iloada",
            Instr::IntStoreLocal => "istore",
            Instr::IntStoreHigherLocal => "istoren",
            Instr::IntStoreGlobal => "istoreg",
            Instr::IntStoreToArray => "istorea",

            Instr::FloatAdd => "fadd",
            Instr::FloatSub => "fsub",
            Instr::FloatMul => "fmul",
            Instr::FloatDiv => "fdiv",
            Instr::FloatNeg => "fneg",
            Instr::FloatNewArray => "fnewa",
            Instr::FloatPop => "fpop",
            Instr::FloatNotEq => "fne",
            Instr::FloatEq => "feq",
            Instr::FloatLess => "flt",
            Instr::FloatLessEqual => "fle",
            Instr::FloatGreater => "fgt",
            Instr::FloatGreaterEqual => "fge",
            Instr::FloatToInt => "f2i",
            Instr::FloatReturn => "freturn",

            Instr::FloatLoadLocal0 => "fload_0",
            Instr::FloatLoadLocal1 => "fload_1",
            Instr::FloatLoadLocal2 => "fload_2",
            Instr::FloatLoadLocal3 => "fload_3",
            Instr::FloatLoad0 => "floadc_0",
            Instr::FloatLoad1 => "floadc_1",
            Instr::FloatLoadConstant => "floadc",
            Instr::FloatLoadLocal => "fload",
            Instr::FloatLoadHigherLocal => "floadn",
            Instr::FloatLoadGlobal => "floadg",
            Instr::FloatLoadFromArray => "floada",
            Instr::FloatStoreLocal => "fstore",
            Instr::FloatStoreHigherLocal => "fstoren",
            Instr::FloatStoreGlobal => "fstoreg",
            Instr::FloatStoreToArray => "fstorea",

            Instr::IntLoadExternal => "iloade",
            Instr::FloatLoadExternal => "floade",
            Instr::BoolLoadExternal => "bloade",
            Instr::ArrayLoadExternal => "aloade",
            Instr::BoolAdd => "badd",
            Instr::BoolNot => "bnot",
            Instr::BoolNewArray => "bnewa",
            Instr::BoolPop => "bpop",
            Instr::BoolNotEq => "bne",
            Instr::BoolEq => "beq",
            Instr::IntStoreExternal => "istoree",
            Instr::FloatStoreExternal => "fstoree",
            Instr::BoolStoreExternal => "bstoree",
            Instr::ArrayStoreExternal => "bmul",
            Instr::BitMul => "astoree",
            Instr::BoolReturn => "breturn",

            Instr::BoolLoadLocal0 => "bload_0",
            Instr::BoolLoadLocal1 => "bload_1",
            Instr::BoolLoadLocal2 => "bload_2",
            Instr::BoolLoadLocal3 => "bload_3",
            Instr::BoolLoadTrue => "bloadc_t",
            Instr::BoolLoadFalse => "bloadc_f",
            Instr::BoolLoadConstant => "bloadc",
            Instr::BoolLoadLocal => "bload",
            Instr::BoolLoadHigherLocal => "bloadn",
            Instr::BoolLoadGlobal => "bloadg",
            Instr::BoolLoadFromArray => "bloada",
            Instr::BoolStoreLocal => "bstore",
            Instr::BoolStoreHigherLocal => "bstoren",
            Instr::BoolStoreGlobal => "bstoreg",
            Instr::BoolStoreToArray => "bstorea",

            Instr::IntIncr => "iinc",
            Instr::IntIncrBy1 => "iinc_1",
            Instr::IntDecr => "idec",
            Instr::IntDecrBy1 => "idec_1",
            Instr::Jump => "jump",
            Instr::BranchTrue => "branch_t",
            Instr::BranchFalse => "branch_f",
            Instr::InitiateLocalSubRoutine => "isr",
            Instr::InitiateHigherLocalSubRoutine => "isrn",
            Instr::IntiateBelowLocalSubRoutine => "isrl",
            Instr::InitiateGlobalSubRoutine => "isrg",
            Instr::EnterSubRoutine => "esr",
            Instr::JumpToSubRoutine => "jsr",
            Instr::JumpToExternalSubRoutine => "jsre",
            Instr::Return => "return",

            Instr::ArrayLoadLocal0 => "aload_0",
            Instr::ArrayLoadLocal1 => "aload_1",
            Instr::ArrayLoadLocal2 => "aload_2",
            Instr::ArrayLoadLocal3 => "aload_3",
            Instr::ArrayLoadLocal => "aload",
            Instr::ArrayLoadHigherLocal => "aloadn",
            Instr::ArrayLoadGlobal => "aloadg",
            Instr::ArrayStoreLocal => "astore",
            Instr::ArrayStoreGlobal => "astoreg",
        }.to_string()
    }
}

impl TryFrom<&str> for Instr {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "iadd" => Ok(Instr::IntAdd),
            "isub" => Ok(Instr::IntSub),
            "imul" => Ok(Instr::IntMul),
            "idiv" => Ok(Instr::IntDiv),
            "irem" => Ok(Instr::IntRem),
            "ineg" => Ok(Instr::IntNeg),
            "inewa" => Ok(Instr::IntNewArray),
            "ipop" => Ok(Instr::IntPop),
            "ine" => Ok(Instr::IntNotEq),
            "ieq" => Ok(Instr::IntEq),
            "ilt" => Ok(Instr::IntLess),
            "ile" => Ok(Instr::IntLessEqual),
            "igt" => Ok(Instr::IntGreater),
            "ige" => Ok(Instr::IntGreaterEqual),
            "i2f" => Ok(Instr::IntToFloat),
            "ireturn" => Ok(Instr::IntReturn),

            "iload_0" => Ok(Instr::IntLoadLocal0),
            "iload_1" => Ok(Instr::IntLoadLocal1),
            "iload_2" => Ok(Instr::IntLoadLocal2),
            "iload_3" => Ok(Instr::IntLoadLocal3),
            "iloadc_0" => Ok(Instr::IntLoad0),
            "iloadc_1" => Ok(Instr::IntLoad1),
            "iloadc_m1" => Ok(Instr::IntLoadM1),
            "iloadc" => Ok(Instr::IntLoadConstant),
            "iload" => Ok(Instr::IntLoadLocal),
            "iloadn" => Ok(Instr::IntLoadHigherLocal),
            "iloadg" => Ok(Instr::IntLoadGlobal),
            "iloada" => Ok(Instr::IntLoadFromArray),
            "istore" => Ok(Instr::IntStoreLocal),
            "istoren" => Ok(Instr::IntStoreHigherLocal),
            "istoreg" => Ok(Instr::IntStoreGlobal),
            "istorea" => Ok(Instr::IntStoreToArray),

            "fadd" => Ok(Instr::FloatAdd),
            "fsub" => Ok(Instr::FloatSub),
            "fmul" => Ok(Instr::FloatMul),
            "fdiv" => Ok(Instr::FloatDiv),
            "fneg" => Ok(Instr::FloatNeg),
            "fnewa" => Ok(Instr::FloatNewArray),
            "fpop" => Ok(Instr::FloatPop),
            "fne" => Ok(Instr::FloatNotEq),
            "feq" => Ok(Instr::FloatEq),
            "flt" => Ok(Instr::FloatLess),
            "fle" => Ok(Instr::FloatLessEqual),
            "fgt" => Ok(Instr::FloatGreater),
            "fge" => Ok(Instr::FloatGreaterEqual),
            "f2i" => Ok(Instr::FloatToInt),
            "freturn" => Ok(Instr::FloatReturn),

            "fload_0" => Ok(Instr::FloatLoadLocal0),
            "fload_1" => Ok(Instr::FloatLoadLocal1),
            "fload_2" => Ok(Instr::FloatLoadLocal2),
            "fload_3" => Ok(Instr::FloatLoadLocal3),
            "floadc_0" => Ok(Instr::FloatLoad0),
            "floadc_1" => Ok(Instr::FloatLoad1),
            "floadc" => Ok(Instr::FloatLoadConstant),
            "fload" => Ok(Instr::FloatLoadLocal),
            "floadn" => Ok(Instr::FloatLoadHigherLocal),
            "floadg" => Ok(Instr::FloatLoadGlobal),
            "floada" => Ok(Instr::FloatLoadFromArray),
            "fstore" => Ok(Instr::FloatStoreLocal),
            "fstoren" => Ok(Instr::FloatStoreHigherLocal),
            "fstoreg" => Ok(Instr::FloatStoreGlobal),
            "fstorea" => Ok(Instr::FloatStoreToArray),

            "iloade" => Ok(Instr::IntLoadExternal),
            "floade" => Ok(Instr::FloatLoadExternal),
            "bloade" => Ok(Instr::BoolLoadExternal),
            "aloade" => Ok(Instr::ArrayLoadExternal),
            "badd" => Ok(Instr::BoolAdd),
            "bnot" => Ok(Instr::BoolNot),
            "bnewa" => Ok(Instr::BoolNewArray),
            "bpop" => Ok(Instr::BoolPop),
            "bne" => Ok(Instr::BoolNotEq),
            "beq" => Ok(Instr::BoolEq),
            "istoree" => Ok(Instr::IntStoreExternal),
            "fstoree" => Ok(Instr::FloatStoreExternal),
            "bstoree" => Ok(Instr::BoolStoreExternal),
            "bmul" => Ok(Instr::ArrayStoreExternal),
            "astoree" => Ok(Instr::BitMul),
            "breturn" => Ok(Instr::BoolReturn),

            "bload_0" => Ok(Instr::BoolLoadLocal0),
            "bload_1" => Ok(Instr::BoolLoadLocal1),
            "bload_2" => Ok(Instr::BoolLoadLocal2),
            "bload_3" => Ok(Instr::BoolLoadLocal3),
            "bloadc_t" => Ok(Instr::BoolLoadTrue),
            "bloadc_f" => Ok(Instr::BoolLoadFalse),
            "bloadc" => Ok(Instr::BoolLoadConstant),
            "bload" => Ok(Instr::BoolLoadLocal),
            "bloadn" => Ok(Instr::BoolLoadHigherLocal),
            "bloadg" => Ok(Instr::BoolLoadGlobal),
            "bloada" => Ok(Instr::BoolLoadFromArray),
            "bstore" => Ok(Instr::BoolStoreLocal),
            "bstoren" => Ok(Instr::BoolStoreHigherLocal),
            "bstoreg" => Ok(Instr::BoolStoreGlobal),
            "bstorea" => Ok(Instr::BoolStoreToArray),

            "iinc" => Ok(Instr::IntIncr),
            "iinc_1" => Ok(Instr::IntIncrBy1),
            "idec" => Ok(Instr::IntDecr),
            "idec_1" => Ok(Instr::IntDecrBy1),
            "jump" => Ok(Instr::Jump),
            "branch_t" => Ok(Instr::BranchTrue),
            "branch_f" => Ok(Instr::BranchFalse),
            "isr" => Ok(Instr::InitiateLocalSubRoutine),
            "isrn" => Ok(Instr::InitiateHigherLocalSubRoutine),
            "isrl" => Ok(Instr::IntiateBelowLocalSubRoutine),
            "isrg" => Ok(Instr::InitiateGlobalSubRoutine),
            "esr" => Ok(Instr::EnterSubRoutine),
            "jsr" => Ok(Instr::JumpToSubRoutine),
            "jsre" => Ok(Instr::JumpToExternalSubRoutine),
            "return" => Ok(Instr::Return),

            "aload_0" => Ok(Instr::ArrayLoadLocal0),
            "aload_1" => Ok(Instr::ArrayLoadLocal1),
            "aload_2" => Ok(Instr::ArrayLoadLocal2),
            "aload_3" => Ok(Instr::ArrayLoadLocal3),
            "aload" => Ok(Instr::ArrayLoadLocal),
            "aloadn" => Ok(Instr::ArrayLoadHigherLocal),
            "aloadg" => Ok(Instr::ArrayLoadGlobal),
            "astore" => Ok(Instr::ArrayStoreLocal),
            "astoreg" => Ok(Instr::ArrayStoreGlobal),

            _ => Err(format!("Unknown opcode: {}", value))
        }
    }
}

/// Type of an operand.
///
/// Specification of length and interpretation
#[derive(Clone, Copy, PartialEq)]
pub enum OperandType<T: ToString> {
    None,
    /// 'A' type
    ArgumentCount(Option<u8>),
    /// 'L' type
    Local(Option<u8>),
    /// 'N' type
    LinkCount(Option<u8>),
    /// 'C' type
    ConstantIndex(Option<u16>),
    /// 'G' type
    GlobalIndex(Option<u16>),
    /// 'I' type
    ImportIndex(Option<u16>),
    /// 'O' type
    Offset(Option<T>),
}

impl<T: ToString> ToString for OperandType<T> {
    fn to_string(&self) -> String {
        match self {
            OperandType::None => String::new(),
            OperandType::ArgumentCount(Some(x)) => x.to_string(),
            OperandType::Local(Some(x)) => x.to_string(),
            OperandType::LinkCount(Some(x)) => x.to_string(),
            OperandType::ConstantIndex(Some(x)) => x.to_string(),
            OperandType::GlobalIndex(Some(x)) => x.to_string(),
            OperandType::ImportIndex(Some(x)) => x.to_string(),
            OperandType::Offset(Some(x)) => x.to_string(),
            _ => String::new()
        }
    }
}


/// A VM operation
#[derive(Copy, Clone, PartialEq)]
pub struct Operation<T: ToString> {
    /// The instruction to execute
    instr: Instr,

    /// The operands to use for this instruction
    operands: [OperandType<T>; 2],
}

impl<T: ToString> Operation<T> {
    /// Get a mutalbe reference to the operands
    pub fn operands_mut(&mut self) -> &mut [OperandType<T>; 2] {
        &mut self.operands
    }

    /// Get the instruction
    pub fn instruction(&self) -> Instr {
        self.instr
    }

    /// Get the operands
    pub fn operands(&self) -> &[OperandType<T>; 2] {
        &self.operands
    }
}

impl<T: ToString> From<Instr> for Operation<T> {
    fn from(value: Instr) -> Self {
        Self { instr: value, operands: value.operands() }
    }
}

impl<T: ToString> ToString for Operation<T> {
    fn to_string(&self) -> String {
        format!(
            "{} {} {}",
            self.instr.to_string(),
            self.operands[0].to_string(),
            self.operands[1].to_string()
        )
    }
}