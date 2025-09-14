use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

pub mod macros;

/// Data representation for Rhythm Heaven Fever (Wii)
pub mod fever;
/// Data representation for Rhythm Heaven (NDS)
pub mod gold;
/// Data representation for Rhythm Heaven Megamix (3DS)
pub mod megamix;
/// Temporary module while I remove all my hardcoded fuckery
pub mod from_yaml;

mod serde_fns;

// TODO: remove this (?)
pub use tickflow_binaries::data::*;

// TODO: per-scene definitions
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TickflowCmdDef {
    #[serde(alias = "tkf_name")]
    pub name: String,
    pub tks_name: Option<String>,
    pub cmd: u16,
    pub arg0: Option<u32>,
    #[serde(default)]
    pub args: Vec<TickflowArgDef>,
    #[serde(default)]
    pub is_return: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TickflowArgDef {
    pub name: Option<String>, // maybe?
    #[serde(
        alias = "type",
        serialize_with = "serde_fns::serialize_type_yaml",
        deserialize_with = "serde_fns::deserialize_type_yaml"
    )]
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

impl TickflowCmdDef {
    pub fn sanity_check(&self, fname: &str) -> Result<()> {
        let mut taken_args = vec![];
        let mut was_arg0_used = false;

        for arg in &self.args {
            // TODO: maybe get the argument names into these errors lol
            // TODO: check command names
            // TODO: error on skipped argument
            match arg.into {
                ArgInto::Arg0 => {
                    if self.arg0.is_some() {
                        Err(Error::yaml_sanity(
                            fname.to_owned(),
                            format!(
                                "command {} - arg0 assigned as both argument and variant selector",
                                self.name
                            ),
                        ))?
                    } else if was_arg0_used {
                        Err(Error::yaml_sanity(
                            fname.to_owned(),
                            format!(
                                "command {} - multiple arguments assigned to arg0",
                                self.name
                            ),
                        ))?
                    } else {
                        was_arg0_used = true;
                    }
                }
                ArgInto::ArgPos(pos) => {
                    if taken_args.contains(&pos) {
                        Err(Error::yaml_sanity(
                            fname.to_owned(),
                            format!(
                                "command {} - multiple arguments assigned to argument #{}",
                                self.name, pos
                            ),
                        ))?
                    } else {
                        taken_args.push(pos)
                    }
                }
            }
            arg.sanity_check(fname, &self.name)?
        }

        Ok(())
    }
}

impl TickflowArgDef {
    pub(super) fn sanity_check(&self, fname: &str, cmd_name: &str) -> Result<()> {
        // TODO: check argument names
        if self.into == ArgInto::Arg0 && self.tf_type != ValueType::Unsigned {
            Err(Error::yaml_sanity(
                fname.to_owned(),
                format!(
                    "command {}, argument {} - arg0 argument can only be of type uint",
                    cmd_name,
                    self.name.as_deref().unwrap_or("<unnamed>")
                ),
            ))?;
        }

        Ok(())
    }
}
