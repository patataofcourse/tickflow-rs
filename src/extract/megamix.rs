pub const CODE_OFFSET: u32 = 0x100000;

pub enum Region {
    JP,
    US,
    EU,
    KR,
}

// TODO: check for differences in JP
#[repr(u8)]
pub enum Scene {
    // Global subs
    None = 0xFF,

    // Tengoku
    AgbBatter = 0,
    AgbClap = 1,
    AgbGhost = 2,
    AgbHair = 3,
    AgbHopping = 4,
    AgbMarcher = 5,
    AgbNightWalk = 6,
    AgbQuiz = 7,
    AgbRabbit = 8,
    AgbRat = 9,
    AgbShuji = 0xA,
    AgbSpaceDance = 0xB,
    AgbTap = 0xC,
    AgbTono = 0xD,

    // DS
    NtrAirBoard = 0xE,
    NtrBackbeat = 0xF,
    NtrBlueBirds = 0x10,
    NtrBoxShow = 0x11,
    NtrCameraMan = 0x12,
    NtrChorus = 0x13,
    NtrFrog = 0x14,
    NtrIdol = 0x15,
    NtrNinja = 0x16,
    NtrPingPong = 0x17,
    NtrRobot = 0x18,
    NtrShooting = 0x19,
    NtrShortLive = 0x1A,
    NtrShugyo = 0x1B,

    // Fever
    RvlAssemble = 0x1C,
    /*
    0x1D - Air Rally
    0x1E - Exhibition Match
    0x1F - Flock Step
    0x20 - Cheer Readers
    0x21 - Double Date
    0x22 - Catch of the Day
    0x23 - Micro-Row
    0x24 - Fork Lifter
    0x25 - Hole in One
    0x26 - Flipper-Flop
    0x27 - Ringside
    0x28 - Karate Man
    0x29 - Working Dough
    0x2A - Figure Fighter
    0x2B - Love Rap
    0x2C - Bossa Nova
    0x2D - Screwbot Factory
    0x2E - Launch Party
    0x2F - Board Meeting
    0x30 - Samurai Slice
    0x31 - See-Saw
    0x32 - Packing Pests
    0x33 - Monkey Watch

    // Megamix
    0x34 - Blue Bear
    0x35 - Animal Acrobat
    0x36 - Tongue Lashing
    0x37 - Super Samurai Slice
    0x38 - Fruit Basket
    0x39 - Second Contact
    0x3A - Pajama Party
    0x3B - Catchy Tune
    0x3C - Sumo Brothers
    0x3D - Tangotronic 3000
    0x3E - Kitties!
    0x3F - LumBEARjack

    // Endless
    0x40 - Sick Beats
    0x41 - Coin Toss
    0x42 - Clap Trap
    0x43 - Charging Chicken

    // Misc
    0x44 - ???
    0x45 - Corrupt Save Handler
    0x46 - Title Screen
    0x47 - Game Select (Land)
    0x48 - Game Select (Tower)
    0x49 - Prologue
    0x4A - Epilogue
    0x4B - Opening
    0x4C - Farewell/To Heaven World
    0x4D - Gameplay Counts
    0x4E - Letter After MED
    0x4F - Introduction
    0x50 - Goat Stable
    0x51 - Café
    0x52 - Shop
    0x53 - Museum
    0x54 - Badges
    0x55 - Memories
    0x56 - Mascots
    0x57 - Epilogue Slideshow
    0x58 - Streetpass Terrace/Figure Fighter VS
    0x59 - Challenge Land
    0x5A - Challenge Train
    0x5B - Challenge Train Results (Score Goal)
    0x5C - Challenge Train Results (Lofe Goal)
    0x5D - Challenge Train Results (Monster Goal)
    0x5E - Challenge Land (Course Select)
    */
    Unk5F = 0x5F,
    Dlp = 0x60,
    DemoUnk61 = 0x61,
    DemoThankYou = 0x62,
}

impl Scene {
    #[allow(non_upper_case_globals)]
    pub const Global: Self = Self::None;
}

type NamedLocations = &'static [(&'static str, u32)];

struct MegamixLocations {
    games: NamedLocations,
    gates: NamedLocations,
    gate_practices: NamedLocations,
    subs: NamedLocations, //HashMap? maybe with lazy_static?
    misc: NamedLocations,
}

const LOCATIONS_US: MegamixLocations = MegamixLocations {
    #[rustfmt::skip]
    games: &[
        // Tengoku - prequels
        ("agbClap_short", 0x39d6dc),
        ("agbGhost_short", 0x39f2d4),
        ("agbHair_short", 0x3a36e8),

        // DS - prequels
        ("ntrChorus_short", 0x418274),
        ("ntrPingPong_short", 0x436b04),
        ("ntrRobot_short", 0x43963c),
        ("ntrShooting_short", 0x43cf18),

        // Fever - prequels
        ("rvlBadminton_short", 0x4719cc),
        ("rvlFlea_short", 0x481260),
        ("rvlGoma_short", 0x48b308),
        ("rvlMuscle_short", 0x4ac0ec),

        // Megamix - prequels
        ("ctrFruitbasket_short", 0x3ee7d8),
        ("ctrInterpreter_short", 0x3f2380),
        ("ctrStep_short", 0x3f7b60),
        ("ctrWoodcat_short", 0x403c94),

        // Tengoku - standard games
        ("agbBatter_long", 0x39a43c),
        ("agbClap_long", 0x39c824),
        ("agbGhost_long", 0x39ea80),
        ("agbHair_long", 0x3a0e24),
        ("agbHopping_long", 0x3a5b44),
        ("agbMarcher_long", 0x3a6cd8),
        ("agbNightWalk_long", 0x3a87c4),
        ("agbQuiz_long", 0x3ab094),
        ("agbRabbit_long", 0x3add80),
        ("agbRat_long", 0x3afa0c),
        ("agbShuji_long", 0x3b1350),
        ("agbSpaceDance_long", 0x3b6cec),
        ("agbTap_long", 0x3ba4ec),
        ("agbTono_long", 0x3bc5a4),

        // DS - standard games
        ("ntrAirBoard_long", 0x40aff0),
        ("ntrBackbeat_long", 0x40d5f0),
        ("ntrBlueBirds_long", 0x410070),
        ("ntrBoxShow_long", 0x411b24),
        ("ntrCameraMan_long", 0x413e10),
        ("ntrChorus_long", 0x416554),
        ("ntrFrog_long", 0x41eba4),
        ("ntrIdol_long", 0x42f9a4),
        ("ntrNinja_long", 0x432420),
        ("ntrPingPong_long", 0x435388),
        ("ntrRobot_long", 0x438404),
        ("ntrShooting_long", 0x43b36c),
        ("ntrShortLive_long", 0x43f288),
        ("ntrShugyo_long", 0x441b20),

        // Fever - standard games
        ("rvlAssemble_long", 0x467c24),
        ("rvlBadminton_long", 0x4702b0),
        ("rvlBatting_long", 0x473488),
        ("rvlBird_long", 0x474670),
        ("rvlBook_long", 0x4776b8),
        ("rvlDate_long", 0x47b1b8),
        ("rvlFishing_long", 0x47dba4),
        ("rvlFlea_long", 0x47fe54),
        ("rvlFork_long", 0x4831b8),
        ("rvlGolf_long", 0x486874),
        ("rvlGoma_long", 0x488604),
        ("rvlInterview_long", 0x490ef0),
        ("rvlManju_long", 0x4a5ccc),
        ("rvlMuscle_long", 0x4a9774),
        ("rvlRap_long", 0x4ade78),
        ("rvlRecieve_long", 0x4b2a94),
        ("rvlRobot_long", 0x4b4bdc),
        ("rvlRocket_long", 0x4b6608),
        ("rvlRotation_long", 0x4b8948),
        ("rvlSamurai_long", 0x4ba75c),
        ("rvlSeesaw_long", 0x4bc698),
        ("rvlSort_long", 0x4bdac8),
        ("rvlWatch_long", 0x4c1d24),

        // Megamix - standard games
        ("ctrBear_long", 0x3c2ad0),
        ("ctrBlanco_long", 0x3c53e8),
        ("ctrChameleon_long", 0x3c68dc),
        ("ctrDotSamurai_long", 0x3e9dfc),
        ("ctrFruitbasket_long", 0x3ecca0),
        ("ctrInterpreter_long", 0x3f04a0),
        ("ctrPillow_long", 0x3f47ac),
        ("ctrStep_long", 0x3f60f8),
        ("ctrSumou_long", 0x3f9fb4),
        ("ctrTango_long", 0x3fcae8),
        ("ctrTeppan_long", 0x3ff460),
        ("ctrWoodCat_long", 0x401360),

        // Tengoku - sequels
        ("agbClap_arrange", 0x39b938),
        ("agbSpaceDance_arrange", 0x3b4f88),
        ("agbTap_arrange", 0x3b980c),

        // DS - sequels
        ("ntrFrog_arrange", 0x41c894),
        ("ntrIdol_arrange", 0x42bd50),
        ("ntrPingPong_arrange", 0x433ce4),

        // Fever - sequels
        ("rvlGolf_arrange", 0x485164),
        ("rvlManju_arrange", 0x4a56fc),
        ("rvlMuscle_arrange", 0x4a6de8),

        // Megamix - sequels
        ("ctrBlanco_arrange", 0x3c42a4),
        ("ctrDotSamurai_arrange", 0x3e88ec),

        // Karate Man
        ("rvlKarate0", 0x493220),   // Prequel
        ("rvlKarate1", 0x495c28),   // Returns
        ("rvlKarate2", 0x4983fc),   // Kicks
        ("rvlKarate3", 0x49c174),   // Combos
        ("rvlKarate4", 0x4a1d30),   // Senior

        // Remixes
        ("remixLED", 0x45d3e4),     // Lush (Low EnDing)
        ("remixTED", 0x4600bc),     // Final (Top EnDing)
        ("remix00", 0x443f94),      // Honeybee
        ("remix01", 0x44734c),      // Machine
        ("remix02", 0x44b458),      // Citrus
        ("remix03", 0x44e498),      // Donut
        ("remix04", 0x450b58),      // Barbershop
        ("remix05", 0x45350c),      // Songbird
        ("remix06", 0x456210),      // Left-Hand
        ("remix07", 0x459b20),      // Right-Hand
    ],
    gates: &[
        ("agbVirus00", 0x3bdd64),
        ("agbVirus01", 0x3bfdb0),
        ("agbVirus02", 0x3c0888),
        ("agbVirusEndless", 0x3c1448),
        ("ntrCoinToss00", 0x419b40),
        ("ntrCoinToss01", 0x41b688),
        ("ntrCoinToss02", 0x41bd5c),
        ("ntrCoinTossEndless", 0x41c430),
        ("rvlSword00", 0x4bf1c8),
        ("rvlSword01", 0x4c09cc),
        ("rvlSword02", 0x4c0f08),
        ("rvlSwordEndless", 0x4c1304),
        ("ctrChicken00", 0x3c96dc),
        ("ctrChicken01", 0x3cb3d4),
        ("ctrChicken02", 0x3cc278),
        ("ctrChickenEndless", 0x3cd11c),
    ],
    gate_practices: &[
        ("agbVirusPractice", 0x3bdee4),
        ("ctrCoinTossPractice", 0x419cc0),
        ("rvlSwordPractice", 0x4bf348),
        ("ctrChickenPractice", 0x3c985c),
    ],
    subs: &[],
    misc: &[],
};

pub fn extract_games_from_code(region: Region) {
    todo!();
}
