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
}

#[derive(Debug, Clone)]
pub struct TempoVal {
    pub beats: f32,
    pub time: u32, // in 32000ths of a second
    pub loop_val: u32,
}
