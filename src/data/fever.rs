use bytestream::ByteOrder;
use tickflow_binaries::data::{btks::BtksType, ArgsTickflowOpDef, OperationSet, RawTickflowOp, TickflowOpDef};

use crate::{tf_op, tf_op_args};

pub enum FeverOp {
    Other(RawTickflowOp),
}

impl OperationSet for FeverOp {
    const BTKS_TICKFLOW_TYPE: BtksType = BtksType::Fever;

    const ENDIAN: ByteOrder = ByteOrder::BigEndian;

    fn get_operation(op: RawTickflowOp) -> Self
    where
        Self: Sized,
    {
        Self::Other(op)
    }

    fn get_call_operations() -> Vec<ArgsTickflowOpDef> {
        vec![
            tf_op_args!(0, [(0)]),
            tf_op_args!(1, [(0)]),
            tf_op_args!(0x100<0>, [(1)]),
        ]
    }

    fn get_string_operations() -> Vec<ArgsTickflowOpDef> {
        vec![
            tf_op_args!(0x105, [(0)]),
            tf_op_args!(0x106, [(0)]),
            tf_op_args!(0x107, [(0)]),
            tf_op_args!(0x108, [(0)]),
            tf_op_args!(0x124, [(0)]),
        ]
    }

    fn get_array_operations() -> Vec<ArgsTickflowOpDef> {
        todo!()
    }

    fn get_depth_operations() -> Vec<TickflowOpDef> {
        vec![
            tf_op!(0x10<0>),
            tf_op!(0x10<1>),
            tf_op!(0x10<2>),
            tf_op!(0x10<3>),
            tf_op!(0x10<4>),
            tf_op!(0x10<5>),
            tf_op!(0x13),
        ]
    }

    fn get_undepth_operations() -> Vec<TickflowOpDef> {
        vec![
            tf_op!(0x12),
            tf_op!(0x17),
        ]
    }

    fn get_scene_operation() -> ArgsTickflowOpDef {
        tf_op_args!(0x100<0>, [(0)])
    }

    fn get_return_operations() -> Vec<TickflowOpDef> {
        vec![
            tf_op!(2),
            tf_op!(3),
        ]
    }
}
