use crate::{
    data::{ArgsTickflowOp, OperationSet, Pointer, RawTickflowOp},
    tf_op_args,
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

//TODO: MissingRequiredArgument errors
//TODO: finish this
impl OperationSet for MegamixOp {
    fn get_operation(op: RawTickflowOp) -> Self {
        match (&op.op, &op.arg0, &op.scene) {
            (0, 0, _) => Self::CallSub {
                sub: *op.args.get(0).expect("Missing required argument"),
                time: match op.args.get(1) {
                    Some(&c) => Some(c),
                    None => None,
                },
                cat: match op.args.get(2) {
                    Some(&c) => Some(c),
                    None => None,
                },
            },
            (1, 0, _) => Self::CallFunc {
                func: *op.args.get(0).expect("Missing required argument"),
                time: match op.args.get(1) {
                    Some(&c) => Some(c),
                    None => None,
                },
            },
            (1, 1, _) => Self::SetFunc {
                func: *op.args.get(0).expect("Missing required argument"),
                pos: (*op.args.get(1).expect("Missing required argument")).into(),
            },
            (_, _, _) => Self::Other(op),
        }
    }
    fn get_call_operations() -> Vec<ArgsTickflowOp> {
        vec![
            tf_op_args!(0x1<1>, [(1),]),
            tf_op_args!(0x2, [(0),]),
            tf_op_args!(0x3<2>, [(0),]),
            tf_op_args!(0x6, [(0),]),
        ]
    }
    fn get_string_operations() -> Vec<ArgsTickflowOp> {
        todo!();
    }
    fn get_array_operations() -> Vec<ArgsTickflowOp> {
        todo!();
    }
    fn get_depth_operations() -> Vec<ArgsTickflowOp> {
        todo!();
    }
    fn get_undepth_operations() -> Vec<ArgsTickflowOp> {
        todo!();
    }
    fn get_scene_operation() -> ArgsTickflowOp {
        todo!();
    }
    fn get_return_operations() -> Vec<ArgsTickflowOp> {
        todo!();
    }
}
