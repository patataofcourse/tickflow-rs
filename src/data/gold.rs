use crate::{
    data::{btks::BtksType, ArgsTickflowOp, OperationSet, RawTickflowOp},
    tf_op_args,
};

pub enum GoldOp {
    Other(RawTickflowOp),
}

impl OperationSet for GoldOp {
    const BTKS_TICKFLOW_TYPE: BtksType = BtksType::Gold;

    fn get_operation(op: RawTickflowOp) -> Self
    where
        Self: Sized,
    {
        //TODO
        Self::Other(op)
    }

    fn get_scene_operation() -> ArgsTickflowOp {
        tf_op_args!(0x100, [(-1)])
    }

    fn get_call_operations() -> Vec<ArgsTickflowOp> {
        todo!();
    }

    fn get_string_operations() -> Vec<ArgsTickflowOp> {
        todo!();
    }

    fn get_array_operations() -> Vec<ArgsTickflowOp> {
        todo!();
    }

    fn get_return_operations() -> Vec<ArgsTickflowOp> {
        todo!();
    }

    fn get_depth_operations() -> Vec<ArgsTickflowOp> {
        todo!();
    }

    fn get_undepth_operations() -> Vec<ArgsTickflowOp> {
        todo!();
    }
}
