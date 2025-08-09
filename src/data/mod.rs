use serde::{Deserialize, Serialize};

pub mod macros;

/// Data representation for Rhythm Heaven Fever (Wii)
pub mod fever;
/// Data representation for Rhythm Heaven (NDS)
pub mod gold;
/// Data representation for Rhythm Heaven Megamix (3DS)
pub mod megamix;

// TODO: remove this (?)
pub use tickflow_binaries::data::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TickflowCmdDef {
    #[serde(alias = "tkf_name")]
    pub name: String,
    pub tks_name: Option<String>,
    pub cmd: u32,
    pub arg0: Option<u32>,
    pub args: Vec<TickflowArgDef>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TickflowArgDef {
    pub name: String, // maybe?
    #[serde(alias = "type")]
    pub tf_type: ValueType,
    #[serde(alias = "pass_to")]
    pub into: ArgInto,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum ArgInto {
    #[serde(rename = "arg0")]
    Arg0,
    #[serde(untagged)]
    ArgPos(u32),
}
