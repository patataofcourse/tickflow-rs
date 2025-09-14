// TODO: replace with actual modular definitions of some kind, + different function systems

use std::sync::LazyLock;

use bytestream::ByteOrder;
use tickflow_binaries::data::{
    btks::BtksType, ArgsTickflowOpDef, OperationSet, TickflowOpDef, ValueType,
};

use crate::data::TickflowCmdDef;

pub struct MegamixFromYaml;

pub struct MegamixFromYamlData {
    pub call_operations: Vec<ArgsTickflowOpDef>,
    pub return_operations: Vec<TickflowOpDef>,
}

macro_rules! define_macro_vars {{$($name:ident = $val:expr;)+} => {$(macro_rules! $name {{} => {$val}});+}}

define_macro_vars! {
    MEGAMIX_CMDS_FNAME = "../../cmds/megamix_intl.yml";
}

static MEGAMIX_FROM_YAML_DATA: LazyLock<MegamixFromYamlData> =
    LazyLock::new(initialize_megamix_from_yaml);

pub fn initialize_megamix_from_yaml() -> MegamixFromYamlData {
    let yaml: &Vec<TickflowCmdDef> =
        &serde_yaml::from_str(include_str!(MEGAMIX_CMDS_FNAME!())).unwrap();
    for cmd in yaml {
        cmd.sanity_check(MEGAMIX_CMDS_FNAME!()).unwrap()
    }

    let mut call_operations = vec![];
    let mut return_operations = vec![];
    for cmd in yaml {
        // call operations
        let matching_args: Vec<_> = cmd
            .args
            .iter()
            .enumerate()
            .filter_map(|(pos, arg)| {
                if let ValueType::TfPointer | ValueType::TfPointerSync = arg.tf_type {
                    Some((pos as i8, false))
                } else {
                    None
                }
            })
            .collect();

        if !matching_args.is_empty() {
            call_operations.push(ArgsTickflowOpDef {
                op: cmd.cmd,
                arg0: cmd.arg0,
                args: matching_args,
                scene: -1, // TODO
            })
        }

        // return operations
        if cmd.is_return {
            return_operations.push(TickflowOpDef {
                op: cmd.cmd,
                arg0: cmd.arg0,
                scene: -1, // TODO
            })
        }
    }

    MegamixFromYamlData {
        call_operations,
        return_operations,
    }
}

impl OperationSet for MegamixFromYaml {
    const BTKS_TICKFLOW_TYPE: BtksType = BtksType::MegamixIntl;
    const ENDIAN: ByteOrder = ByteOrder::LittleEndian;

    fn get_call_operations() -> Vec<ArgsTickflowOpDef> {
        MEGAMIX_FROM_YAML_DATA.call_operations.clone()
    }

    fn get_return_operations() -> Vec<TickflowOpDef> {
        MEGAMIX_FROM_YAML_DATA.return_operations.clone()
    }

    // TODO

    fn get_string_operations() -> Vec<ArgsTickflowOpDef> {
        vec![]
    }

    fn get_depth_operations() -> Vec<TickflowOpDef> {
        vec![]
    }

    fn get_undepth_operations() -> Vec<TickflowOpDef> {
        vec![]
    }

    fn get_array_operations() -> Vec<ArgsTickflowOpDef> {
        vec![]
    }

    fn get_scene_operation() -> ArgsTickflowOpDef {
        // this will never match anything
        ArgsTickflowOpDef { op: 0x4000, arg0: None, args: vec![], scene: -1 }
    }
}
