use super::{Pointer, TickflowOp};

pub enum MegamixOp {
    CallSub(u32),
    CallFunc(u32),
    Call(Pointer),
    SetFunc { func: u32, pos: Pointer },
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
    SetRest { slot: u32, amount: u32 },
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
    TempoRel { factor: u32, lower: u32, upper: u32 },
    TempoID(u32),
    Speed(u32),
    SpeedRel { factor: u32, lower: u32, upper: u32 },
    Scene(u32),
    SceneDone,
    LoadStoredScene,
    SetStoredScene,
    BottomScreenBg(bool),
    SetSceneInitCounter,
    IncSceneInitCounter(i32),
    UnrestSceneInitCounter,

    Other(TickflowOp),
}
