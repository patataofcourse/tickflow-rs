use std::io::{self, Cursor, Seek, SeekFrom, Write};

use crate::{data::{TickflowOp}, extract::{self, Pointer}};

use bytestream::{ByteOrder, StreamWriter};

type Result<T> = std::io::Result<T>; //TODO: make my own error type

#[derive(Debug, Clone)]
pub struct BTKS {
    pub btks_type: BtksType,
    pub flow: FlowSection,
    pub ptro: Option<Vec<Pointer>>,
    pub tmpo: Option<Vec<Tempo>>,
    pub strd: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct FlowSection {
    pub start_offset: u32,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct Tempo {
    pub id: u32,
    pub data: Vec<TempoVal>,
    pub sample_rate: u32,
}

#[derive(Debug, Clone)]
pub struct TempoVal {
    pub beats: f32,
    pub time: u32, // in samples
    pub loop_val: u32,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum BtksType {
    MegamixIntl = 0,
    MegamixJp = 1,
    FeverJp = 2,
    FeverUs = 3,
    FeverEu = 4,
    FeverKr = 5,
    Gold = 6,
    Unspecified = -1,
}

impl BTKS {
    const REVISION: u32 = 2;
    const HEADER_SIZE: u32 = 0x18;
    const FLOW_HEADER: u32 = 0xC;
    const PTRO_HEADER: u32 = 0xC;
    const TMPO_HEADER: u32 = 0xC;
    const STRD_HEADER: u32 = 0x8;
}

impl BTKS {
    pub fn to_btks_file<F: Write + Seek>(&self, f: &mut F, endian: ByteOrder) -> io::Result<()> {
        // ------------
        //    Header
        // ------------
        f.write_all(b"BTKS")?; //magic
        let mut size = Self::HEADER_SIZE;
        let mut num_sections = 2;
        let size_pos = f.stream_position()?;
        0u32.write_to(f, endian)?;
        Self::REVISION.write_to(f, endian)?;
        Self::HEADER_SIZE.write_to(f, endian)?;
        let num_sections_pos = f.stream_position()?;
        0u32.write_to(f, endian)?;
        (self.btks_type as i32).write_to(f, endian)?; // US-EU-KR Megamix

        // ----------
        //    FLOW
        // ----------
        f.write_all(b"FLOW")?; //magic
        let flow_size = Self::FLOW_HEADER + self.flow.data.len() as u32;
        size += flow_size;
        flow_size.write_to(f, endian)?;
        self.flow.start_offset.write_to(f, endian)?;
        f.write_all(&self.flow.data)?;

        // ----------
        //    PTRO
        // ----------
        if let Some(c) = &self.ptro {
            num_sections += 1;
            f.write_all(b"PTRO")?; //magic
            let ptro_size: u32 = Self::PTRO_HEADER + c.len() as u32 * 5;
            size += ptro_size;
            ptro_size.write_to(f, endian)?;
            (c.len() as u32).write_to(f, endian)?;
            for pointer in c {
                f.write_all(&pointer.as_ptro(endian))?;
            }
        }

        // ----------
        //    TMPO
        // ----------
        if let Some(c) = &self.tmpo {
            num_sections += 1;
            f.write_all(b"TMPO")?; //magic
            let mut tmpo_size: u32 = Self::TMPO_HEADER + c.len() as u32 * 8;
            for tempo in c {
                tmpo_size += tempo.data.len() as u32 * 0x10;
            }
            tmpo_size.write_to(f, endian)?;
            (c.len() as u32).write_to(f, endian)?;
            for tempo in c {
                tempo.write_to(f, endian)?;
            }
        }

        // ----------
        //    STRD
        // ----------
        f.write_all(b"STRD")?; //magic
        let strd_size: u32 = Self::STRD_HEADER + self.strd.len() as u32;
        size += strd_size;
        strd_size.write_to(f, endian)?;
        f.write_all(&self.strd)?;

        // Write filesize and number of sections
        f.seek(SeekFrom::Start(size_pos))?;
        size.write_to(f, endian)?;
        f.seek(SeekFrom::Start(num_sections_pos))?;
        num_sections.write_to(f, endian)?;

        Ok(())
    }

    // for debugging reasons
    pub fn to_raw_tickflow_ops(&self, endian: ByteOrder) -> Result<Vec<TickflowOp>> {
        let mut data = Cursor::new(&self.flow.data);
        let mut ops = vec![];
        while data.position() != data.get_ref().len() as u64 {
            let raw_op = extract::binary_to_raw_tf_op(&mut data, -1, endian)?.1;
            ops.push(raw_op.into())
        }
        Ok(ops)
    }
}

impl Tempo {
    pub fn is_streamed(&self) -> bool {
        //TODO: currently always makes custom tempo IDs AAC since they can't be called for BCGRPs
        !(self.id >= 0x01000101 && self.id <= 0x01000281)
    }
}

impl StreamWriter for Tempo {
    fn write_to<W: Write>(&self, buffer: &mut W, order: ByteOrder) -> io::Result<()> {
        self.id.write_to(buffer, order)?;
        (self.data.len() as u32).write_to(buffer, order)?;
        (if self.is_streamed() { 1u32 } else { 0u32 }).write_to(buffer, order)?;
        for value in &self.data {
            buffer.write_all(&match order {
                ByteOrder::BigEndian => value.beats.to_be_bytes(),
                ByteOrder::LittleEndian => value.beats.to_le_bytes(),
            })?;
            value.time.write_to(buffer, order)?;
            value.loop_val.write_to(buffer, order)?;
        }
        Ok(())
    }
}
