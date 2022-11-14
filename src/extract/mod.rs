use crate::data::{btks::BTKS, OperationSet, RawTickflowOp};
use bytestream::*;
use std::io::{Read, Seek, SeekFrom};

pub mod gold;
pub mod megamix;

type Result<T> = std::io::Result<T>; //TODO: make my own error type

#[derive(Debug)]
enum Pointer {
    Tickflow { at: u32, points_to: u32 },
    String { at: u32, points_to: u32 },
    Array { at: u32, points_to: u32 },
}

pub fn extract<T: OperationSet>(
    file: &mut (impl Read + Seek),
    base_offset: u32,
    start_queue: Vec<u32>,
) -> Result<BTKS> {
    let mut func_order = vec![];
    let mut func_positions = vec![];
    let mut queue = vec![];
    for pos in start_queue {
        queue.push((pos, -1));
    }
    let mut bincmds = vec![];
    let mut bindata = vec![];
    let mut pointers = vec![];
    let mut pos = 0;
    while pos < queue.len() {
        func_order.push(queue[pos].0 - base_offset);
        func_positions.push(bindata.len());
        pointers.extend(extract_tickflow_at::<T>(
            base_offset,
            file,
            &mut queue,
            pos,
            &mut bincmds,
            &mut bindata,
        )?);
        pos += 1
    }
    dbg!(bincmds, bindata, pointers);
    todo!();
}

/// Equivalent to Tickompiler's firstPass
fn extract_tickflow_at<T: OperationSet>(
    base_offset: u32,
    file: &mut (impl Read + Seek),
    queue: &mut Vec<(u32, i32)>,
    pos: usize,
    bincmds: &mut Vec<u8>,
    bindata: &mut Vec<u8>,
) -> Result<Vec<Pointer>> {
    let mut scene = queue[pos].1;
    file.seek(SeekFrom::Start(queue[pos].0 as u64 - base_offset as u64))?;
    let mut done = false;
    let mut pointers = vec![];
    let mut depth = 0;
    while !done {
        let op_int = u32::read_from(file, ByteOrder::LittleEndian)?;
        let op = (op_int & 0x3FF) as u16;
        let arg0 = op_int >> 0x18;
        let arg_count = ((op_int & 0x3C00) >> 10) as u8;
        let mut args = vec![];
        for _ in 0..arg_count {
            args.push(u32::read_from(file, ByteOrder::LittleEndian)?);
        }
        let tf_op = RawTickflowOp {
            op,
            arg0,
            args: args.clone(),
            scene,
        };

        //TODO: what if some operations are both? make sure that never happens,
        //or offer an actual alternative
        if let Some(c) = T::is_scene_operation(&tf_op) {
            scene = if c == -1 { arg0 } else { args[c as usize] } as i32;
        } else if let Some(c) = T::is_call_operation(&tf_op, scene) {
            let pointer_pos = args[c.args[0].0 as usize];
            let mut is_in_queue = false;
            for (position, _) in &*queue {
                if *position == pointer_pos {
                    is_in_queue = true;
                    break;
                }
            }
            if !is_in_queue {
                queue.push((pointer_pos, scene));
            }
            args[c.args[0].0 as usize] = pointer_pos - base_offset;

            pointers.push(Pointer::Tickflow {
                at: bincmds.len() as u32 + (4 * (c.args[0].0 + 1)) as u32,
                points_to: pointer_pos - base_offset,
            });
        } else if let Some(c) = T::is_string_operation(&tf_op, scene) {
            for arg in &c.args {
                pointers.push(Pointer::String {
                    at: bindata.len() as u32 + (4 * (arg.0 + 1)) as u32,
                    points_to: bindata.len() as u32,
                });

                bindata.extend(read_string(
                    base_offset,
                    file,
                    args[arg.0 as usize].into(),
                    arg.1,
                )?);
            }
        //TODO: check if array_op
        } else if let Some(_) = T::is_depth_operation(&tf_op, scene) {
            depth += 1;
        } else if let Some(_) = T::is_undepth_operation(&tf_op, scene) {
            if depth > 0 {
                depth -= 1;
            }
        } else if let Some(_) = T::is_return_operation(&tf_op, scene) {
            if depth <= 0 {
                done = true;
            }
        }
        op_int.write_to(bincmds, ByteOrder::LittleEndian)?;
        for arg in args {
            arg.write_to(bincmds, ByteOrder::LittleEndian)?;
        }
    }
    Ok(pointers)
}

fn read_string<F: Read + Seek>(
    base_offset: u32,
    file: &mut F,
    pos: u64,
    is_unicode: bool,
) -> Result<Vec<u8>> {
    let og_pos = file.stream_position()?;
    if pos < base_offset as u64 {
        return Ok(vec![0, 0]);
    }
    file.seek(SeekFrom::Start(pos - base_offset as u64))?;
    let mut string_data = vec![];

    if is_unicode {
        loop {
            let chr = u16::read_from(file, ByteOrder::LittleEndian)?;
            string_data.extend(chr.to_le_bytes());
            if chr == 0 {
                break;
            }
        }
    } else {
        loop {
            let chr = u8::read_from(file, ByteOrder::LittleEndian)?;
            string_data.push(chr);
            if chr == 0 {
                break;
            }
        }
    }

    //padding
    for _ in 0..(4 - string_data.len() % 4) {
        string_data.push(0);
    }

    file.seek(SeekFrom::Start(og_pos))?;
    Ok(string_data)
}
