use bytestream::ByteOrder;

use crate::{
    data::{
        btks::BtksType, ArgsTickflowOpDef, OperationSet, Pointer, RawTickflowOp, TickflowOpDef,
    },
    tf_op, tf_op_args,
};

pub enum MegamixOp {
    CallSub {
        sub: u32,
        time: Option<u32>,
        cat: Option<u32>,
    },
    CallFunc {
        func: u32,
        time: Option<u32>,
    },
    SetFunc {
        func: u32,
        pos: Pointer,
    },
    Call {
        loc: Pointer,
        time: Option<u32>,
    },
    KillAll,
    KillCat(u32),
    KillLoc(Pointer),
    KillSub(u32),
    RunSub(u32),
    RunFunc(u32),
    Run(Pointer),
    Return,
    Stop,
    Cat(u32),
    SetCondvar(i32),
    AddCondvar(i32),
    PushCondvar,
    PopCondvar,
    Rest(u32),
    SetRest {
        slot: u32,
        amount: u32,
    },
    GetRest(u32),
    Delay(u32),
    RestReset,
    Unrest(u32),
    Label(u32),
    Goto(u32),
    IfEq(i32),
    IfNe(i32),
    IfLt(i32),
    IfLe(i32),
    IfGt(i32),
    IfGe(i32),
    Else,
    EndIf,
    Switch,
    Case(i32),
    BreakCase,
    DefaultCase,
    EndSwitch,
    SetCountdown(i32),
    SetCountdownCondvar,
    GetCountdownInit,
    GetCountdownProgress,
    GetCountdown,
    DecCountdown,
    Tempo(u32),
    TempoRel {
        factor: u32,
        lower: u32,
        upper: u32,
    },
    TempoID(u32),
    Speed(u32),
    SpeedRel {
        factor: u32,
        lower: u32,
        upper: u32,
    },
    Scene(u32),
    SceneDone,
    LoadStoredScene,
    SetStoredScene,
    BottomScreenBg(bool),
    SetSceneInitCounter,
    IncSceneInitCounter(i32),
    UnrestSceneInitCounter,

    Other(RawTickflowOp),
}

impl OperationSet for MegamixOp {
    const BTKS_TICKFLOW_TYPE: BtksType = BtksType::MegamixIntl;
    const ENDIAN: ByteOrder = ByteOrder::LittleEndian;

    //TODO: MissingRequiredArgument errors
    //TODO: finish this
    fn get_operation(op: RawTickflowOp) -> Self {
        match op.as_definition() {
            tf_op!(0) => Self::CallSub {
                sub: *op.args.first().expect("Missing required argument"),
                time: op.args.get(1).copied(),
                cat: op.args.get(2).copied(),
            },
            tf_op!(1<0>) => Self::CallFunc {
                func: *op.args.first().expect("Missing required argument"),
                time: op.args.get(1).copied(),
            },
            tf_op!(1<1>) => Self::SetFunc {
                func: *op.args.first().expect("Missing required argument"),
                pos: (*op.args.get(1).expect("Missing required argument")).into(),
            },
            _ => Self::Other(op),
        }
    }

    fn get_call_operations() -> Vec<ArgsTickflowOpDef> {
        vec![
            tf_op_args!(0x1<1>, [(1)]),
            tf_op_args!(0x2, [(0)]),
            tf_op_args!(0x3<2>, [(0)]),
            tf_op_args!(0x6, [(0)]),
        ]
    }
    fn get_string_operations() -> Vec<ArgsTickflowOpDef> {
        vec![
            tf_op_args!(0x31, [(1, true)]),
            tf_op_args!(0x35<0>, [(1, true)]),
            tf_op_args!(0x36, [(1, true)]),
            tf_op_args!(0x39<0>, [(1, true)]),
            tf_op_args!(0x3A, [(1, true)]),
            tf_op_args!(0x3B, [(2)]),
            tf_op_args!(0x3E<0>, [(1, true)]),
            tf_op_args!(0x5D<0>, [(1, true)]),
            tf_op_args!(0x5D<2>, [(0, true)]),
            tf_op_args!(0x61<2>, [(0, true)]),
            tf_op_args!(0x65<1>, [(1)]),
            tf_op_args!(0x66, [(1)]),
            tf_op_args!(0x67<1>, [(1)]),
            tf_op_args!(0x68<1>, [(1)]),
            tf_op_args!(0x93, [(2), (3)]),
            tf_op_args!(0x94, [(1), (2), (3)]),
            tf_op_args!(0x95, [(1)]),
            tf_op_args!(0xAF<2>, [(2),]),
            tf_op_args!(0xB0<4>, [(1),]),
            tf_op_args!(0xB0<5>, [(1),]),
            tf_op_args!(0xB0<6>, [(1),]),
            tf_op_args!(0xB5, [(0),]),
            tf_op_args!(0x105, [(0)], 1),
            tf_op_args!(0x107<0>, [(0)], 0xC),
            tf_op_args!(0x107<1>, [(0)], 0xC),
            tf_op_args!(0x106, [(0)], 0x18),
            tf_op_args!(0x106, [(0)], 0x2A),
            tf_op_args!(0x10B, [(0)], 0x2C),
            tf_op_args!(0x107<0>, [(0)], 0x39),
            tf_op_args!(0x107<1>, [(0),], 0x39),
            tf_op_args!(0x108, [(0)], 0x39),
            tf_op_args!(0x109, [(0), (1)], 0x39),
            tf_op_args!(0x10A, [(0)], 0x39),
        ]
    }
    fn get_array_operations() -> Vec<ArgsTickflowOpDef> {
        todo!();
    }
    fn get_depth_operations() -> Vec<TickflowOpDef> {
        vec![
            tf_op!(0x16),
            tf_op!(0x16<1>),
            tf_op!(0x16<2>),
            tf_op!(0x16<3>),
            tf_op!(0x16<4>),
            tf_op!(0x16<5>),
            tf_op!(0x19),
        ]
    }
    fn get_undepth_operations() -> Vec<TickflowOpDef> {
        vec![tf_op!(0x18), tf_op!(0x1D)]
    }
    fn get_scene_operation() -> ArgsTickflowOpDef {
        tf_op_args!(0x28, [(0)])
    }
    fn get_return_operations() -> Vec<TickflowOpDef> {
        vec![tf_op!(0x7), tf_op!(0x8)]
    }
}
