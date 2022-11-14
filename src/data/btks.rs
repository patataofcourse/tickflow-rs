use crate::data::Pointer;

#[derive(Debug, Clone)]
pub struct BTKS {
    pub btks_type: u32,
    pub flow: FlowSection,
    pub ptro: Option<Vec<Pointer>>,
    pub tmpo: Option<Vec<Tempo>>,
    pub strd: Option<Vec<u8>>,
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
#[derive(Debug, Clone)]
pub enum BtksType {
    MegamixIntl = 0,
    MegamixJp = 1,
    Fever = 2,
    Gold = 3,
    Unspecified = -1,
}
