use std::{fs::File, io::{SeekFrom, Write}};
use tickflow::{
    data::{fever::FeverUsOp, from_yaml::MegamixFromYaml, gold::GoldOp, megamix::MegamixOp, OperationSet},
    error::Result,
    extract::{
        self, dol::DolFile, fever::CODE_OFFSET as OFFSET_RHF, gold::TICKOVY_OFFSET_US,
        megamix::CODE_OFFSET as OFFSET_RHM,
    },
};

const MEGAMIX_GAME: usize = 0;

const MEGAMIX_POS: u32 = extract::megamix::LOCATIONS_US.games[MEGAMIX_GAME].1;
const MEGAMIX_NAME: &str = extract::megamix::LOCATIONS_US.games[MEGAMIX_GAME].0;

fn main() -> Result<()> {
    let mut f = File::open("test_files/code.bin")?;
    let btks = extract::extract::<MegamixOp>(&mut f, OFFSET_RHM, &[MEGAMIX_POS])?;
    let mut fw = File::create(format!("test_files/{MEGAMIX_NAME}.btk",))?;

    btks.to_btks_file(&mut fw, MegamixOp::ENDIAN)?;

    let btks_yaml = extract::extract::<MegamixFromYaml>(&mut f, OFFSET_RHM,  &[MEGAMIX_POS])?;
    writeln!(File::create("test_files/megamixtest.txt")?, "{btks_yaml:#?}")?;
    writeln!(File::create("test_files/megamixtest2.txt")?, "{btks:#?}")?;


    //let mut f = File::open("test_files/ovy9_90.bin")?;
    //extract::extract::<GoldOp>(&mut f, TICKOVY_OFFSET_US, &[])?;

    let mut f = DolFile::new(File::open("test_files/main.dol")?, FeverUsOp::ENDIAN)?;
    let mut fw = File::create("test_files/characterIntro.btk")?;
    let btks = extract::extract::<FeverUsOp>(&mut f, OFFSET_RHF, &[0x802B5D40])?;
    btks.to_btks_file(&mut fw, FeverUsOp::ENDIAN)?;

    Ok(())
}
