use std::io::{Error, ErrorKind, Read, Result, Seek, SeekFrom};

use bytestream::{ByteOrder, StreamReader};

#[derive(Debug, Clone)]
pub struct DolFile<F: Read + Seek> {
    pub text: [DolSection; 7],
    pub data: [DolSection; 11],
    pub bss: [u32; 2],
    pub entry: u32,
    inner: F,
    addr: u32,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DolSection {
    pub address: u32,
    pub offset: u64,
    pub size: u32,
}

impl DolSection {
    pub fn end(&self) -> u32 {
        self.address + self.size
    }
}

impl<F: Read + Seek> DolFile<F> {
    pub fn new(mut file: F, endian: ByteOrder) -> Result<Self> {
        let mut text: [DolSection; 7] = [DolSection::default(); 7];
        let mut data: [DolSection; 11] = [DolSection::default(); 11];

        for section in &mut text {
            section.offset = u32::read_from(&mut file, endian)? as u64;
        }
        for section in &mut data {
            section.offset = u32::read_from(&mut file, endian)? as u64;
        }
        for section in &mut text {
            section.address = u32::read_from(&mut file, endian)?;
        }
        for section in &mut data {
            section.address = u32::read_from(&mut file, endian)?;
        }
        for section in &mut text {
            section.size = u32::read_from(&mut file, endian)?;
        }
        for section in &mut data {
            section.size = u32::read_from(&mut file, endian)?;
        }

        let bss = [
            u32::read_from(&mut file, endian)?,
            u32::read_from(&mut file, endian)?,
        ];

        let entry = u32::read_from(&mut file, endian)?;

        let mut out = Self {
            text, data, bss, entry, inner: file, addr: 0
        };

        out.seek(SeekFrom::Start(entry as u64))?;
        Ok(out)

    }

    pub fn new_with_addr(file: F, endian: ByteOrder, addr: u32) -> Result<Self> {
        let mut out = Self::new(file, endian)?;
        out.addr = addr;
        Ok(out)
    }

    pub fn get_section_of(&self, addr: u32) -> Option<DolSection> {
        for section in &self.text {
            if addr >= section.address && addr < section.end() {
                return Some(*section);
            }
        }

        for section in &self.data {
            if addr >= section.address && addr < section.end() {
                return Some(*section);
            }
        }

        None
    }
}

impl<F: Read + Seek> Read for DolFile<F> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let Some(section) = self.get_section_of(self.addr) else {
            return Ok(0);
        };

        let read_end = buf
            .len()
            .try_into()
            .map(|c| self.addr.checked_add(c))
            .ok()
            .flatten()
            .ok_or(Error::new(
                ErrorKind::AddrNotAvailable,
                "DOL read address overflows over the 32-bit limit",
            ))?;

        self.addr = read_end;
        if read_end >= section.end() {
            let bytes_avail = (section.end() - self.addr) as usize;
            self.inner.read_exact(&mut buf[..bytes_avail])?;

            // try to see if a section starts right after the end of this one
            if self.get_section_of(section.end()).is_some() {
                // seek to new position, read rest of the bytes, add bytes read before to result
                self.seek(SeekFrom::Start(section.end() as u64))?;
                self.read(&mut buf[bytes_avail..]).map(|c| c + bytes_avail)
            } else {
                // can't do anything more, abort read early
                Ok(bytes_avail)
            }
        } else {
            self.inner.read_exact(buf)?;
            Ok(buf.len())
        }
    }
}

impl<F: Read + Seek> Seek for DolFile<F> {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        let pos = match pos {
            SeekFrom::Start(c) => c,
            SeekFrom::End(_) => unimplemented!("DOL seeking from end"),
            SeekFrom::Current(c) => (self.addr as u64)
                .checked_add_signed(c)
                .ok_or(Error::other("64-bit overflow on DOL seeking (??)"))?,
        };
        let addr: u32 = pos.try_into().map_err(|_| {
            Error::new(
                ErrorKind::AddrNotAvailable,
                "DOL seek position must be 32-bit",
            )
        })?;

        let section = self.get_section_of(addr).ok_or(Error::new(
            ErrorKind::AddrNotAvailable,
            "DOL seeking: address not mapped",
        ))?;

        self.inner.seek(SeekFrom::Start(
            section.offset + (addr - section.address) as u64,
        ))?;
        self.addr = addr;
        Ok(addr as u64)
    }
}
