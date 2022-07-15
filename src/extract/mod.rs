use crate::data::OperationSet;
use std::io::Read;

pub mod megamix;

pub fn extract<R: Read, T: OperationSet>(
    file: &mut R,
    base_offset: u32,
    start_queue: Vec<u32>,
) -> Vec<T> {
    let mut queue = vec![];
    for pos in start_queue {
        queue.push((pos, -1));
    }
    let mut bincmds = vec![];
    let mut bindata = vec![];
    let mut data_pointers = vec![];
    let mut pos = 0;
    while pos < queue.len() {}
    todo!();
}
