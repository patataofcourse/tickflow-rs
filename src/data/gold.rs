use bytestream::ByteOrder;

use crate::{
    data::{btks::BtksType, ArgsTickflowOpDef, OperationSet, RawTickflowOp, TickflowOpDef},
    tf_op_args,
};

pub enum GoldOp {
    Other(RawTickflowOp),
}

impl OperationSet for GoldOp {
    const BTKS_TICKFLOW_TYPE: BtksType = BtksType::Gold;
    const ENDIAN: ByteOrder = ByteOrder::LittleEndian;

    fn get_operation(op: RawTickflowOp) -> Self
    where
        Self: Sized,
    {
        //TODO
        Self::Other(op)
    }

    fn get_scene_operation() -> ArgsTickflowOpDef {
        tf_op_args!(0x100, [(-1)])
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
