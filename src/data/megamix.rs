use bytestream::ByteOrder;

use crate::{args_tf_op, args_tf_op_vec, tf_op_vec};

use tickflow_binaries::data::{
    btks::BtksType, ArgsTickflowOpDef, OperationSet, Pointer, RawTickflowOp, TickflowOpDef,
};

//TODO: derive macro that creates specifications automatically
pub enum MegamixOp {
    //#[tickflow_op(0)]
    CallSub {
        //#[arg(0)]
        sub: u32,
        //#[arg(1, default=0)]
        time: Option<u32>,
        //#[arg(2, default=???)]
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
    CallSubSync(u32),
    CallFuncSync(u32),
    CallSync(Pointer),
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
    Sleep(u32),
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
    SceneModel {
        scene: i32,
        model_slot: u32,
    },
    SceneCellanim {
        scene: i32,
        cellanim_slot: u32,
    },
    SceneEffect {
        scene: i32,
        effect_slot: u32,
    },
    SceneLayout {
        scene: i32,
        layout_slot: u32,
    },
    SceneVersion {
        scene: i32,
        version: u32, //TODO: make this an enum
    },
    SceneGetVersion(i32),
    //TODO: make this yet another enum
    CurSceneIsVersion(u32),
    SceneUnload,
    SceneIsUnloaded,
    Pause(bool),

    Other(RawTickflowOp),
}

impl OperationSet for MegamixOp {
    const BTKS_TICKFLOW_TYPE: BtksType = BtksType::MegamixIntl;
    const ENDIAN: ByteOrder = ByteOrder::LittleEndian;

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
