use bytestream::ByteOrder;
use tickflow_derive::OperationSet;

use crate as tickflow;

use tickflow_binaries::data::{
    btks::BtksType, Pointer, RawTickflowOp,
};

//TODO: derive macro that creates specifications automatically
#[derive(OperationSet)]
#[tickflow(btks_type=BtksType::MegamixIntl, endian=ByteOrder::LittleEndian)]
pub enum MegamixOp {
    #[tickflow_op(val = 0)]
    CallSub {
        //#[arg(0)]
        sub: u32,
        //#[arg(1, default=0)]
        time: Option<u32>,
        //#[arg(2, default=???)]
        cat: Option<u32>,
    },
    #[tickflow_op(val = 1<0>)]
    CallFunc {
        func: u32,
        time: Option<u32>,
    },
    #[tickflow_op(val = 1<1>)]
    SetFunc {
        func: u32,
        pos: Pointer,
    },
    #[tickflow_op(val = 2)]
    Call {
        loc: Pointer,
        time: Option<u32>,
    },
    #[tickflow_op(val = 3<0>)]
    KillAll,
    #[tickflow_op(val = 3<1>)]
    KillCat(u32),
    #[tickflow_op(val = 3<2>)]
    KillLoc(Pointer),
    #[tickflow_op(val = 3<3>)]
    KillSub(u32),
    #[tickflow_op(val = 4)]
    CallSubSync(u32),
    #[tickflow_op(val = 5)]
    CallFuncSync(u32),
    #[tickflow_op(val = 6)]
    CallSync(Pointer),
    #[tickflow_op(val = 7)]
    Return,
    #[tickflow_op(val = 8)]
    Stop,
    #[tickflow_op(val = 9)]
    Cat(u32),
    #[tickflow_op(val = 0xA)]
    SetCondvar(i32),
    #[tickflow_op(val = 0xB)]
    AddCondvar(i32),
    #[tickflow_op(val = 0xC)]
    PushCondvar,
    #[tickflow_op(val = 0xD)]
    PopCondvar,
    #[tickflow_op(val = 0xE)]
    Rest(u32),
    #[tickflow_op(val = 0xF<0>)]
    SetRest {
        slot: u32,
        amount: u32,
    },
    #[tickflow_op(val = 0xF<1>)]
    GetRest(u32),
    #[tickflow_op(val = 0x10)]
    Sleep(u32),
    #[tickflow_op(val = 0x11)]
    RestReset,
    #[tickflow_op(val = 0x12)]
    Unrest(u32),
    #[tickflow_op(val = 0x14)]
    Label(u32),
    #[tickflow_op(val = 0x15)]
    Goto(u32),
    #[tickflow_op(val = 0x16<0>)]
    IfEq(i32),
    #[tickflow_op(val = 0x16<1>)]
    IfNe(i32),
    #[tickflow_op(val = 0x16<2>)]
    IfLt(i32),
    #[tickflow_op(val = 0x16<3>)]
    IfLe(i32),
    #[tickflow_op(val = 0x16<4>)]
    IfGt(i32),
    #[tickflow_op(val = 0x16<5>)]
    IfGe(i32),
    #[tickflow_op(val = 0x17)]
    Else,
    #[tickflow_op(val = 0x18)]
    EndIf,
    #[tickflow_op(val = 0x19)]
    Switch,
    #[tickflow_op(val = 0x1A)]
    Case(i32),
    #[tickflow_op(val = 0x1B)]
    BreakCase,
    #[tickflow_op(val = 0x1C)]
    DefaultCase,
    #[tickflow_op(val = 0x1D)]
    EndSwitch,
    #[tickflow_op(val = 0x1E<0>)]
    SetCountdown(i32),
    #[tickflow_op(val = 0x1E<1>)]
    SetCountdownCondvar,
    #[tickflow_op(val = 0x1E<2>)]
    GetCountdownInit,
    #[tickflow_op(val = 0x1E<3>)]
    GetCountdownProgress,
    #[tickflow_op(val = 0x1E<4>)]
    GetCountdown,
    #[tickflow_op(val = 0x1E<5>)]
    DecCountdown,
    #[tickflow_op(val = 0x21)]
    Tempo(u32),
    #[tickflow_op(val = 0x22)]
    TempoRel {
        factor: u32,
        lower: u32,
        upper: u32,
    },
    #[tickflow_op(val = 0x23)]
    TempoID(u32),
    #[tickflow_op(val = 0x24)]
    Speed(u32),
    #[tickflow_op(val = 0x25)]
    SpeedRel {
        factor: u32,
        lower: u32,
        upper: u32,
    },
    #[tickflow_op(val = 0x27)]
    Speed120 {
        unk: u32,
        val: u32,
    },
    //TODO: make this an enum
    #[tickflow_op(val = 0x28<0>)]
    Scene(i32),
    #[tickflow_op(val = 0x28<1>)]
    SceneDone,
    #[tickflow_op(val = 0x28<2>)]
    LoadStoredScene,
    #[tickflow_op(val = 0x28<3>)]
    SetStoredScene(u32),
    #[tickflow_op(val = 0x28<4>)]
    BottomScreenBg(bool),
    #[tickflow_op(val = 0x29<0>)]
    SetSceneInitCounter,
    #[tickflow_op(val = 0x29<1>)]
    IncSceneInitCounter(i32),
    #[tickflow_op(val = 0x29<2>)]
    UnrestSceneInitCounter,
    #[tickflow_op(val = 0x2A<0>)]
    SceneModel {
        scene: i32,
        model_slot: u32,
    },
    #[tickflow_op(val = 0x2A<2>)]
    SceneCellanim {
        scene: i32,
        cellanim_slot: u32,
    },
    #[tickflow_op(val = 0x2A<3>)]
    SceneEffect {
        scene: i32,
        effect_slot: u32,
    },
    #[tickflow_op(val = 0x2A<4>)]
    SceneLayout {
        scene: i32,
        layout_slot: u32,
    },
    #[tickflow_op(val = 0x2B<0>)]
    SceneVersion {
        scene: i32,
        version: u32, //TODO: make this an enum
    },
    #[tickflow_op(val = 0x2B<1>)]
    SceneGetVersion(i32),
    //TODO: make an enum
    #[tickflow_op(val = 0x2B<2>)]
    CurSceneIsVersion(u32),
    #[tickflow_op(val = 0x2C<0>)]
    SceneUnload,
    #[tickflow_op(val = 0x2C<1>)]
    SceneIsUnloaded,
    #[tickflow_op(val = 0x2D)]
    Pause(bool),

    //#[tickflow_op(val = default)]
    //Other(RawTickflowOp),
}

/* impl OperationSet for MegamixOp {
    const BTKS_TICKFLOW_TYPE: BtksType = BtksType::MegamixIntl;
    const ENDIAN: ByteOrder = ByteOrder::LittleEndian;

    //TODO: MissingRequiredArgument errors
    //TODO: finish this
    fn get_operation(op: RawTickflowOp) -> Self {
        match op.as_definition() {
            tf_op!(~0) => Self::CallSub {
                sub: *op.args.first().expect("Missing required argument"),
                time: op.args.get(1).copied(),
                cat: op.args.get(2).copied(),
            },
            tf_op!(~1<0>) => Self::CallFunc {
                func: *op.args.first().expect("Missing required argument"),
                time: op.args.get(1).copied(),
            },
            tf_op!(~1<1>) => Self::SetFunc {
                func: *op.args.first().expect("Missing required argument"),
                pos: (*op.args.get(1).expect("Missing required argument")).into(),
            },
            tf_op!(~2) => Self::Call {
                loc: (*op.args.first().expect("Missing required argument")).into(),
                time: op.args.get(1).copied(),
            },

            tf_op!(~3<0>) => Self::KillAll,
            tf_op!(~3<1>) => Self::KillCat(*op.args.first().expect("Missing required argument")),
            tf_op!(~3<2>) => Self::KillSub(*op.args.first().expect("Missing required argument")),
            tf_op!(~3<3>) => {
                Self::KillLoc((*op.args.first().expect("Missing required argument")).into())
            }

            tf_op!(~4) => Self::CallSubSync(*op.args.first().expect("Missing required argument")),
            tf_op!(~5) => Self::CallFuncSync(*op.args.first().expect("Missing required argument")),
            tf_op!(~6) => Self::Call {
                loc: (*op.args.first().expect("Missing required argument")).into(),
                time: op.args.get(1).copied(),
            },

            tf_op!(~7) => Self::Return,
            tf_op!(~8) => Self::Stop,

            tf_op!(~9) => Self::Cat(*op.args.first().expect("Missing required argument")),

            tf_op!(~0xa) => {
                Self::SetCondvar(*op.args.first().expect("Missing required argument") as i32)
            }
            tf_op!(~0xb) => {
                Self::AddCondvar(*op.args.first().expect("Missing required argument") as i32)
            }
            tf_op!(~0xc) => Self::PushCondvar,
            tf_op!(~0xd) => Self::PopCondvar,

            tf_op!(~0xe<=arg0=>) => Self::Rest(arg0),
            tf_op!(~0xf<0>) => Self::SetRest {
                slot: *op.args.first().expect("Missing required argument"),
                amount: *op.args.get(1).expect("Missing required argument"),
            },
            tf_op!(~0xf<1>) => Self::GetRest(*op.args.first().expect("Missing required argument")),
            tf_op!(~0x10<=arg0=>) => Self::Sleep(arg0),
            tf_op!(~0x11) => Self::RestReset,
            //TODO: check if this one is truly arg0
            tf_op!(~0x12<=arg0=>) => Self::Unrest(arg0),

            _ => Self::Other(op),
        }
    }

    fn get_call_operations() -> Vec<ArgsTickflowOpDef> {
        args_tf_op_vec![
            0x1<1>, [(1)];
            0x2, [(0)];
            0x3<2>, [(0)];
            0x6, [(0)];
        ]
    }
    fn get_string_operations() -> Vec<ArgsTickflowOpDef> {
        args_tf_op_vec![
            0x31, [(1, true)];
            0x35<0>, [(1, true)];
            0x36, [(1, true)];
            0x39<0>, [(1, true)];
            0x3A, [(1, true)];
            0x3B, [(2)];
            0x3E<0>, [(1, true)];
            0x5D<0>, [(1, true)];
            0x5D<2>, [(0, true)];
            0x61<2>, [(0, true)];
            0x65<1>, [(1)];
            0x66, [(1)];
            0x67<1>, [(1)];
            0x68<1>, [(1)];
            0x93, [(2), (3)];
            0x94, [(1), (2), (3)];
            0x95, [(1)];
            0xAF<2>, [(2)];
            0xB0<4>, [(1)];
            0xB0<5>, [(1)];
            0xB0<6>, [(1)];
            0xB5, [(0)];
            0x105, [(0)], 1;
            0x107<0>, [(0)], 0xC;
            0x107<1>, [(0)], 0xC;
            0x106, [(0)], 0x18;
            0x106, [(0)], 0x2A;
            0x10B, [(0)], 0x2C;
            0x107<0>, [(0)], 0x39;
            0x107<1>, [(0)], 0x39;
            0x108, [(0)], 0x39;
            0x109, [(0), (1)], 0x39;
            0x10A, [(0)], 0x39;
        ]
    }
    fn get_array_operations() -> Vec<ArgsTickflowOpDef> {
        todo!();
    }
    fn get_depth_operations() -> Vec<TickflowOpDef> {
        tf_op_vec![
            0x16,
            0x16<1>,
            0x16<2>,
            0x16<3>,
            0x16<4>,
            0x16<5>,
            0x19,
        ]
    }
    fn get_undepth_operations() -> Vec<TickflowOpDef> {
        tf_op_vec![0x18, 0x1D]
    }
    fn get_scene_operation() -> ArgsTickflowOpDef {
        args_tf_op!(0x28, [(0)])
    }
    fn get_return_operations() -> Vec<TickflowOpDef> {
        tf_op_vec![0x7, 0x8]
    }
}
 */
