use std::{fs::File, io::Result};
use tickflow_rs::{
    data::{gold::GoldOp, megamix::MegamixOp, OperationSet},
    extract::{self, gold::TICKOVY_OFFSET_US, megamix::CODE_OFFSET},
};

fn main() -> Result<()> {
    let mut f = File::open("test_files/code.bin")?;
    let btks = extract::extract::<MegamixOp>(&mut f, CODE_OFFSET, &[0x39d6dc])?;
    let mut fw = File::create("test_files/agbClap_short.btk")?;
    btks.to_btks_file(&mut fw, MegamixOp::ENDIAN)?;

    let mut f = File::open("test_files/ovy9_90.bin")?;
    extract::extract::<GoldOp>(&mut f, TICKOVY_OFFSET_US, &[])?;

    Ok(())
}
