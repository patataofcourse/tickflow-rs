pub mod megamix;

/// Tickflow operation as decompiled, before parsing
pub struct RawTickflowOp {
    pub op: u16,
    pub arg0: u32,
    pub args: Vec<u32>,
    pub scene: i32,
}

/// Tickflow operation ready for compilation
pub struct TickflowOp {
    pub op: u16,
    pub arg0: Arg0,
    pub args: Vec<Arg>,
    pub scene: i32,
}

pub enum Arg0 {
    Signed(i32),
    Unsigned(u32),
    Unknown(u32),
}

pub enum Arg {
    Signed(i32),
    Unsigned(u32),
    String(String),
    Array(Array),
    Pointer(Pointer),
    Struct(Vec<u8>),
    Unknown(u32),
}

pub enum Array {
    Word(Vec<u32>),
    SignedWord(Vec<i32>),
    Byte(Vec<u8>),
    SignedByte(Vec<i8>),
    Half(Vec<u16>),
    SignedHalf(Vec<i16>),
}

pub struct Pointer(pub u32);

pub trait OperationSet {
    fn get_operation(op: RawTickflowOp) -> Self
    where
        Self: Sized;
}

impl Arg {
    pub fn from_struct(s: impl Into<Vec<u8>>) -> Self {
        Self::Struct(s.into())
    }
}

impl From<RawTickflowOp> for TickflowOp {
    fn from(op: RawTickflowOp) -> Self {
        let mut args = vec![];
        for arg in op.args {
            args.push(Arg::Unknown(arg))
        }
        TickflowOp {
            op: op.op,
            arg0: Arg0::Unknown(op.arg0),
            args: args,
            scene: op.scene,
        }
    }
}

impl OperationSet for TickflowOp {
    fn get_operation(op: RawTickflowOp) -> Self {
        op.into()
    }
}

impl From<Vec<u32>> for Array {
    fn from(vec: Vec<u32>) -> Self {
        Self::Word(vec)
    }
}

impl From<Vec<i32>> for Array {
    fn from(vec: Vec<i32>) -> Self {
        Self::SignedWord(vec)
    }
}

impl From<Vec<u16>> for Array {
    fn from(vec: Vec<u16>) -> Self {
        Self::Half(vec)
    }
}

impl From<Vec<i16>> for Array {
    fn from(vec: Vec<i16>) -> Self {
        Self::SignedHalf(vec)
    }
}

impl From<Vec<u8>> for Array {
    fn from(vec: Vec<u8>) -> Self {
        Self::Byte(vec)
    }
}

impl From<Vec<i8>> for Array {
    fn from(vec: Vec<i8>) -> Self {
        Self::SignedByte(vec)
    }
}

impl From<Array> for Arg {
    fn from(array: Array) -> Self {
        Self::Array(array.into())
    }
}

impl From<String> for Arg {
    fn from(string: String) -> Self {
        Self::String(string)
    }
}

impl From<u32> for Arg {
    fn from(int: u32) -> Self {
        Self::Unsigned(int)
    }
}

impl From<u32> for Arg0 {
    fn from(int: u32) -> Self {
        Self::Unsigned(int)
    }
}

impl From<i32> for Arg {
    fn from(int: i32) -> Self {
        Self::Signed(int)
    }
}

impl From<i32> for Arg0 {
    fn from(int: i32) -> Self {
        Self::Signed(int)
    }
}

impl From<i32> for Pointer {
    fn from(int: i32) -> Self {
        Self(int as u32)
    }
}

impl From<u32> for Pointer {
    fn from(int: u32) -> Self {
        Self(int)
    }
}
