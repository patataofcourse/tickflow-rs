use std::{fs::File, io::Result};
use tickflow::{
    data::{gold::GoldOp, megamix::MegamixOp, OperationSet},
    extract::{self, gold::TICKOVY_OFFSET_US, megamix::CODE_OFFSET},
};

const MEGAMIX_GAME: usize = 0;

const MEGAMIX_POS: u32 = extract::megamix::LOCATIONS_US.games[MEGAMIX_GAME].1;
const MEGAMIX_NAME: &str = extract::megamix::LOCATIONS_US.games[MEGAMIX_GAME].0;

fn main() -> Result<()> {
    let mut f = File::open("test_files/code.bin")?;
    let btks = extract::extract::<MegamixOp>(&mut f, CODE_OFFSET, &[MEGAMIX_POS])?;
    let mut fw = File::create(format!("test_files/{}.btk", MEGAMIX_NAME))?;
    btks.to_btks_file(&mut fw, MegamixOp::ENDIAN)?;

    //let mut f = File::open("test_files/ovy9_90.bin")?;
    //extract::extract::<GoldOp>(&mut f, TICKOVY_OFFSET_US, &[])?;

    Ok(())
}
