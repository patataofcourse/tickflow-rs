pub mod macros;

/// Data representation for the BTKS (Binary Tickflow Specification) file format
pub mod btks;
/// Data representation for Rhythm Heaven (NDS)
pub mod gold;
/// Data representation for Rhythm Heaven Megamix (3DS)
pub mod megamix;

use btks::BtksType;
use bytestream::ByteOrder;

/// Tickflow operation as decompiled, before parsing
#[derive(Debug, Clone)]
pub struct RawTickflowOp {
    pub op: u16,
    pub arg0: u32,
    pub args: Vec<u32>,
    pub scene: i32,
}

/// Tickflow operation ready for compilation
#[derive(Debug, Clone)]
pub struct TickflowOp {
    pub op: u16,
    pub arg0: Arg0,
    pub args: Vec<Arg>,
    pub scene: i32,
}

/// Tickflow operation definition
#[derive(Debug, Clone, Eq)]
pub struct TickflowOpDef {
    pub op: u16,
    pub arg0: Option<u32>,
    pub scene: i32,
}

/// Tickflow operation definition with specific args indicated
#[derive(Debug, Clone)]
pub struct ArgsTickflowOpDef {
    pub op: u16,
    pub arg0: Option<u32>,
    pub args: Vec<(i8, bool)>,
    pub scene: i32,
}

impl RawTickflowOp {
    pub fn as_definition(&self) -> TickflowOpDef {
        TickflowOpDef {
            op: self.op,
            arg0: Some(self.arg0),
            scene: self.scene,
        }
    }
}

impl PartialEq for TickflowOpDef {
    fn eq(&self, other: &Self) -> bool {
        self.op == other.op
            && (if let Some(c) = &self.arg0 {
                if let Some(d) = &other.arg0 {
                    c == d
                } else {
                    true
                }
            } else {
                true
            })
            && (if self.scene != -1 && other.scene != -1 {
                self.scene == other.scene
            } else {
                true
            })
    }
}

#[derive(Debug, Clone)]
pub enum Arg0 {
    Signed(i32),
    Unsigned(u32),
    Unknown(u32),
}

impl From<Arg0> for u32 {
    fn from(value: Arg0) -> Self {
        match value {
            Arg0::Signed(c) => c as u32,
            Arg0::Unsigned(c) => c,
            Arg0::Unknown(c) => c,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Arg {
    Signed(i32),
    Unsigned(u32),
    String(String),
    Array(Array),
    Pointer(Pointer),
    Struct(Vec<u8>),
    Unknown(u32),
}

#[derive(Debug, Clone)]
pub enum Array {
    Word(Vec<u32>),
    SignedWord(Vec<i32>),
    Byte(Vec<u8>),
    SignedByte(Vec<i8>),
    Half(Vec<u16>),
    SignedHalf(Vec<i16>),
}

#[derive(Debug, Clone)]
pub enum Pointer {
    Raw(u32),
    Label(String),
}

/// Trait for every type of Tickflow operation.
/// You can see implementations for Megamix (international), Fever, and DS in this library.
pub trait OperationSet {
    const BTKS_TICKFLOW_TYPE: BtksType;
    const ENDIAN: ByteOrder;

    //TODO: tempo operations, sub operations

    fn get_operation(op: RawTickflowOp) -> Self
    where
        Self: Sized;
    fn get_call_operations() -> Vec<ArgsTickflowOpDef>;
    fn is_call_operation(op: &RawTickflowOp, scene: i32) -> Option<ArgsTickflowOpDef> {
        for call_op in Self::get_call_operations() {
            if op.op == call_op.op && (call_op.scene == -1 || call_op.scene == scene) {
                match &call_op.arg0 {
                    None => return Some(call_op),
                    Some(c) => {
                        if op.arg0 == *c {
                            return Some(call_op);
                        }
                    }
                }
            }
        }
        None
    }
    fn get_string_operations() -> Vec<ArgsTickflowOpDef>;
    fn is_string_operation(op: &RawTickflowOp, scene: i32) -> Option<ArgsTickflowOpDef> {
        for return_op in Self::get_string_operations() {
            if op.op == return_op.op && (return_op.scene == -1 || return_op.scene == scene) {
                match &return_op.arg0 {
                    None => return Some(return_op),
                    Some(c) => {
                        if op.arg0 == *c {
                            return Some(return_op);
                        }
                    }
                }
            }
        }
        None
    }
    fn get_array_operations() -> Vec<ArgsTickflowOpDef>;
    fn is_array_operation(op: &RawTickflowOp, scene: i32) -> Option<ArgsTickflowOpDef> {
        for array_op in Self::get_array_operations() {
            if op.op == array_op.op && (array_op.scene == -1 || array_op.scene == scene) {
                match &array_op.arg0 {
                    None => return Some(array_op),
                    Some(c) => {
                        if op.arg0 == *c {
                            return Some(array_op);
                        }
                    }
                }
            }
        }
        None
    }
    fn get_depth_operations() -> Vec<TickflowOpDef>;
    fn is_depth_operation(op: &RawTickflowOp, scene: i32) -> Option<TickflowOpDef> {
        for depth_op in Self::get_depth_operations() {
            if op.op == depth_op.op && (depth_op.scene == -1 || depth_op.scene == scene) {
                match &depth_op.arg0 {
                    None => return Some(depth_op),
                    Some(c) => {
                        if op.arg0 == *c {
                            return Some(depth_op);
                        }
                    }
                }
            }
        }
        None
    }
    fn get_undepth_operations() -> Vec<TickflowOpDef>;
    fn is_undepth_operation(op: &RawTickflowOp, scene: i32) -> Option<TickflowOpDef> {
        for undepth_op in Self::get_undepth_operations() {
            if op.op == undepth_op.op && (undepth_op.scene == -1 || undepth_op.scene == scene) {
                match &undepth_op.arg0 {
                    None => return Some(undepth_op),
                    Some(c) => {
                        if op.arg0 == *c {
                            return Some(undepth_op);
                        }
                    }
                }
            }
        }
        None
    }
    fn get_scene_operation() -> ArgsTickflowOpDef;
    fn is_scene_operation(op: &RawTickflowOp) -> Option<i8> {
        let scene_op = Self::get_scene_operation();
        if op.op == scene_op.op {
            match &scene_op.arg0 {
                None => return Some(scene_op.args[0].0),
                Some(c) => {
                    if op.arg0 == *c {
                        return Some(scene_op.args[0].0);
                    }
                }
            }
        }
        None
    }
    fn get_return_operations() -> Vec<TickflowOpDef>;
    fn is_return_operation(op: &RawTickflowOp, scene: i32) -> Option<TickflowOpDef> {
        for return_op in Self::get_return_operations() {
            if op.op == return_op.op && (return_op.scene == -1 || return_op.scene == scene) {
                match &return_op.arg0 {
                    None => return Some(return_op),
                    Some(c) => {
                        if op.arg0 == *c {
                            return Some(return_op);
                        }
                    }
                }
            }
        }
        None
    }
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
            args,
            scene: op.scene,
        }
    }
}

impl OperationSet for TickflowOp {
    const BTKS_TICKFLOW_TYPE: BtksType = BtksType::Unspecified;
    const ENDIAN: ByteOrder = ByteOrder::LittleEndian;

    fn get_operation(op: RawTickflowOp) -> Self {
        op.into()
    }
    fn get_call_operations() -> Vec<ArgsTickflowOpDef> {
        unimplemented!("Operation types for generic TickflowOp")
    }
    fn get_string_operations() -> Vec<ArgsTickflowOpDef> {
        unimplemented!("Operation types for generic TickflowOp")
    }
    fn get_array_operations() -> Vec<ArgsTickflowOpDef> {
        unimplemented!("Operation types for generic TickflowOp")
    }
    fn get_depth_operations() -> Vec<TickflowOpDef> {
        unimplemented!("Operation types for generic TickflowOp")
    }
    fn get_undepth_operations() -> Vec<TickflowOpDef> {
        unimplemented!("Operation types for generic TickflowOp")
    }
    fn get_scene_operation() -> ArgsTickflowOpDef {
        unimplemented!("Operation types for generic TickflowOp")
    }
    fn get_return_operations() -> Vec<TickflowOpDef> {
        unimplemented!("Operation types for generic TickflowOp")
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
        Self::Array(array)
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
        Self::Raw(int as u32)
    }
}

impl From<u32> for Pointer {
    fn from(int: u32) -> Self {
        Self::Raw(int)
    }
}
