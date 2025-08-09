use bytestream::ByteOrder;
use tickflow_binaries::data::{
    btks::BtksType, ArgsTickflowOpDef, OperationSet, TickflowOpDef,
};

use crate::{args_tf_op, args_tf_op_vec, tf_op_vec};

pub struct FeverUsOp;

impl OperationSet for FeverUsOp {
    const BTKS_TICKFLOW_TYPE: BtksType = BtksType::FeverUs;

    const ENDIAN: ByteOrder = ByteOrder::BigEndian;

    fn get_call_operations() -> Vec<ArgsTickflowOpDef> {
        args_tf_op_vec![
            0, [(0)];
            1, [(0)];
            0x100<0>, [(1)];
        ]
    }

    fn get_string_operations() -> Vec<ArgsTickflowOpDef> {
        args_tf_op_vec![
            0x105, [(0)];
            0x106, [(0)];
            0x107<0>, [(0)];
            0x107<1>, [(0)];
            0x107<2>, [(0)];
            0x108, [(0)];
            0x124, [(0)];
        ]
    }

    fn get_array_operations() -> Vec<ArgsTickflowOpDef> {
        todo!()
    }

    fn get_depth_operations() -> Vec<TickflowOpDef> {
        tf_op_vec![
            0x10<0>,
            0x10<1>,
            0x10<2>,
            0x10<3>,
            0x10<4>,
            0x10<5>,
            0x13
        ]
    }

    fn get_undepth_operations() -> Vec<TickflowOpDef> {
        tf_op_vec![0x12, 0x17]
    }

    fn get_scene_operation() -> ArgsTickflowOpDef {
        args_tf_op!(0x100<0>, [(0)])
    }

    fn get_return_operations() -> Vec<TickflowOpDef> {
        tf_op_vec![2, 3]
    }
}
