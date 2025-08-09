use std::{
    fs::File,
    io::{Result, Write},
};
use tickflow::{
    data::{fever::FeverUsOp, gold::GoldOp, megamix::MegamixOp, OperationSet},
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

    //let mut f = File::open("test_files/ovy9_90.bin")?;
    //extract::extract::<GoldOp>(&mut f, TICKOVY_OFFSET_US, &[])?;

    let mut f = DolFile::new(File::open("test_files/main.dol")?, FeverUsOp::ENDIAN)?;
    let mut fw = File::create("test_files/characterIntro.btk")?;
    let btks = extract::extract::<FeverUsOp>(&mut f, OFFSET_RHF, &[0x802B5D40])?;

    let mut fw2 = File::create("test_files/characterIntro.btk.out")?;

    writeln!(fw2, "{:#08x?}", btks.ptro)?;

    for op in btks.to_raw_tickflow_ops(FeverUsOp::ENDIAN)? {
        let op = tickflow_parse::old::Statement::Command {
            cmd: tickflow_parse::old::CommandName::Raw(op.op as i32),
            arg0: if op.arg0 == 0 {
                None
            } else {
                Some(tickflow_parse::old::Value::Integer(op.arg0 as i32))
            },
            args: op
                .args
                .iter()
                .map(|c| tickflow_parse::old::Value::Integer(*c as i32))
                .collect(),
        };
        writeln!(fw2, "{op}")?;
    }

    btks.to_btks_file(&mut fw, FeverUsOp::ENDIAN)?;

    println!("{:#?}", serde_yaml::from_str::<Vec<tickflow::data::TickflowCmdDef>>(include_str!("../test_files/cmds.yml")).unwrap());

    Ok(())
}
