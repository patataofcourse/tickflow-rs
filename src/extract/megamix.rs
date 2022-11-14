pub const CODE_OFFSET: u32 = 0x00100000;

pub enum Region {
    JP,
    US,
    EU,
    KR,
}

type NamedLocations = &'static [(&'static str, u32)];

struct MegamixLocations {
    games: NamedLocations,
    gates: NamedLocations,
    gate_practices: NamedLocations,
    subs: NamedLocations,
    misc: NamedLocations,
}

const LOCATIONS_US: MegamixLocations = MegamixLocations {
    games: &[],
    gates: &[],
    gate_practices: &[],
    subs: &[],
    misc: &[],
};

pub fn extract_games_from_code() {
    todo!();
}
