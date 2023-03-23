use crate::data::{
    btks::{self, BTKS},
    OperationSet, RawTickflowOp,
};
use bytestream::{ByteOrder, StreamReader, StreamWriter};
use std::{
    collections::HashMap,
    io::{Read, Seek, SeekFrom},
};

pub mod gold;
pub mod megamix;

type Result<T> = std::io::Result<T>; //TODO: make my own error type

#[derive(Debug, Clone)]
pub struct Pointer {
    at: usize,
    points_to: u32,
    ptype: PointerType,
}

impl Pointer {
    pub fn as_ptro(&self) -> [u8; 5] {
        let mut out = [0; 5];
        out[..4].copy_from_slice(&self.at.to_le_bytes());
        out[4] = self.ptype as u8;
        out
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PointerType {
    Tickflow,
    Data,
}

pub fn extract<T: OperationSet>(
    file: &mut (impl Read + Seek),
    base_offset: u32,
    start_queue: &[u32],
) -> Result<BTKS> {
    //TODO: proper error instead of panic if start_queue is empty
    let start_offset = start_queue[0];

    let mut functions = HashMap::new();
    let mut queue = vec![];
    for pos in start_queue {
        queue.push((*pos, -1));
    }
    let mut bincmds = vec![];
    let mut bindata = vec![];
    let mut pointers = vec![];
    let mut pos = 0;
    while pos < queue.len() {
        //TODO: hashmap? btreemap?
        functions.insert(queue[pos].0 - base_offset, bindata.len() as u32);
        pointers.extend(extract_tickflow_at::<T>(
            base_offset,
            file,
            &mut queue,
            pos,
            &mut bincmds,
            &mut bindata,
            T::ENDIAN,
        )?);
        pos += 1
    }

    for pointer in &pointers {
        bincmds.splice(
            pointer.at..pointer.at + 4,
            if pointer.ptype == PointerType::Tickflow {
                functions[&pointer.points_to]
            } else {
                pointer.points_to
            }
            .to_le_bytes(),
        );
    }

    // TODO: tempos
    // TODO: handle related subs?
    Ok(BTKS {
        btks_type: T::BTKS_TICKFLOW_TYPE,
        flow: btks::FlowSection {
            start_offset,
            data: bincmds,
        },
        ptro: if pointers.is_empty() {
            None
        } else {
            Some(pointers)
        },
        strd: if bindata.is_empty() {
            None
        } else {
            Some(bindata)
        },
        tmpo: None,
    })
}

/// Equivalent to Tickompiler's firstPass
fn extract_tickflow_at<T: OperationSet>(
    base_offset: u32,
    file: &mut (impl Read + Seek),
    queue: &mut Vec<(u32, i32)>,
    pos: usize,
    bincmds: &mut Vec<u8>,
    bindata: &mut Vec<u8>,
    endian: ByteOrder,
) -> Result<Vec<Pointer>> {
    let mut scene = queue[pos].1;
    file.seek(SeekFrom::Start(queue[pos].0 as u64 - base_offset as u64))?;
    let mut done = false;
    let mut pointers = vec![];
    let mut depth = 0;
    while !done {
        let op_int = u32::read_from(file, T::ENDIAN)?;
        let op = (op_int & 0x3FF) as u16;
        let arg0 = op_int >> 14;
        let arg_count = ((op_int & 0x3C00) >> 10) as u8;
        let mut args = vec![];
        for _ in 0..arg_count {
            args.push(u32::read_from(file, T::ENDIAN)?);
        }
        let tf_op = RawTickflowOp {
            op,
            arg0,
            args: args.clone(),
            scene,
        };

        //TODO: what if some operations are multiple? make sure that never happens,
        //or offer an actual alternative
        if let Some(c) = T::is_scene_operation(&tf_op) {
            scene = if c == -1 { arg0 } else { args[c as usize] } as i32;
        } else if let Some(c) = T::is_call_operation(&tf_op, scene) {
            let pointer_pos = args[c.args[0].0 as usize];
            let mut is_in_queue = false;
            'found: for (position, _) in &*queue {
                if *position == pointer_pos {
                    is_in_queue = true;
                    break 'found;
                }
            }
            if !is_in_queue {
                queue.push((pointer_pos, scene));
            }
            args[c.args[0].0 as usize] = pointer_pos - base_offset;

            pointers.push(Pointer {
                at: bincmds.len() + (4 * (c.args[0].0 + 1)) as usize,
                points_to: pointer_pos - base_offset,
                ptype: PointerType::Tickflow,
            });
        } else if let Some(c) = T::is_string_operation(&tf_op, scene) {
            for arg in &c.args {
                pointers.push(Pointer {
                    at: bincmds.len() + (4 * (arg.0 + 1)) as usize,
                    points_to: bindata.len() as u32,
                    ptype: PointerType::Data,
                });

                bindata.extend(read_string(
                    base_offset,
                    file,
                    args[arg.0 as usize].into(),
                    arg.1,
                    endian,
                )?);
            }
        //TODO: check if array_op
        } else if T::is_depth_operation(&tf_op, scene).is_some() {
            depth += 1;
        } else if T::is_undepth_operation(&tf_op, scene).is_some() {
            if depth > 0 {
                depth -= 1;
            }
        } else if T::is_return_operation(&tf_op, scene).is_some() && depth <= 0 {
            done = true;
        }
        op_int.write_to(bincmds, T::ENDIAN)?;
        for arg in args {
            arg.write_to(bincmds, T::ENDIAN)?;
        }
    }
    Ok(pointers)
}

fn read_string<F: Read + Seek>(
    base_offset: u32,
    file: &mut F,
    pos: u64,
    is_unicode: bool,
    endian: ByteOrder,
) -> Result<Vec<u8>> {
    let og_pos = file.stream_position()?;
    if pos < base_offset as u64 {
        return Ok(vec![0, 0]);
    }
    file.seek(SeekFrom::Start(pos - base_offset as u64))?;
    let mut string_data = vec![];

    if is_unicode {
        loop {
            let chr = u16::read_from(file, endian)?;
            string_data.extend(chr.to_le_bytes());
            if chr == 0 {
                break;
            }
        }
    } else {
        loop {
            let chr = u8::read_from(file, endian)?;
            string_data.push(chr);
            if chr == 0 {
                break;
            }
        }
    }

    //padding
    string_data.extend(vec![0; 4 - string_data.len() % 4]); //TODO: is this needed anymore?

    file.seek(SeekFrom::Start(og_pos))?;
    Ok(string_data)
}
