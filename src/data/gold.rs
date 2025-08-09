use bytestream::ByteOrder;

use crate::args_tf_op;

use tickflow_binaries::data::{
    btks::BtksType, ArgsTickflowOpDef, OperationSet, TickflowOpDef,
};

pub struct GoldOp;

impl OperationSet for GoldOp {
    const BTKS_TICKFLOW_TYPE: BtksType = BtksType::Gold;
    const ENDIAN: ByteOrder = ByteOrder::LittleEndian;

    fn get_scene_operation() -> ArgsTickflowOpDef {
        args_tf_op!(0x100, [(-1)])
    }

    fn get_call_operations() -> Vec<ArgsTickflowOpDef> {
        todo!();
    }

    fn get_string_operations() -> Vec<ArgsTickflowOpDef> {
        todo!();
    }

    fn get_array_operations() -> Vec<ArgsTickflowOpDef> {
        todo!();
    }

    fn get_return_operations() -> Vec<TickflowOpDef> {
        todo!();
    }

    fn get_depth_operations() -> Vec<TickflowOpDef> {
        todo!();
    }

    fn get_undepth_operations() -> Vec<TickflowOpDef> {
        todo!();
    }
}
