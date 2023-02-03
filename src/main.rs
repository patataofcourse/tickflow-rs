use std::{fs::File, io::Result};
use tickflow_rs::{
    data::{gold::GoldOp, megamix::MegamixOp},
    extract::{self, gold::TICKOVY_OFFSET_US, megamix::CODE_OFFSET},
};

fn main() -> Result<()> {
    let mut f = File::open("test_files/code.bin")?;
    extract::extract::<MegamixOp>(&mut f, CODE_OFFSET, vec![0x39a974])?;

    let mut f = File::open("test_files/ovy9_90.bin")?;
    extract::extract::<GoldOp>(&mut f, TICKOVY_OFFSET_US, vec![todo!()])?;

    Ok(())
}
