use std::{
    fs::File,
    io::{Result, Write},
};
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
    let mut fw2 = File::create(format!("test_files/{MEGAMIX_NAME}.btk.out"))?;

    writeln!(fw2, "{:#08x?}", btks.ptro)?;

    for op in btks.to_raw_tickflow_ops(MegamixOp::ENDIAN)? {
        let op = tickflow_parse::old::Statement::Command {
            cmd: tickflow_parse::old::CommandName::Raw(op.op as i32),
            arg0: {
                let temp = match op.arg0 {
                tickflow::data::Arg0::Signed(c) => c,
                tickflow::data::Arg0::Unsigned(c) => c as i32,
                tickflow::data::Arg0::Unknown(c) => c as i32,
            }; if temp == 0 {
                None
            } else {Some(tickflow_parse::old::Value::Integer(temp))}
            },
            args: op.args.iter().map(|c| tickflow_parse::old::Value::String{value:format!("{c:?}"), is_unicode: false}).collect(),
        };
        writeln!(fw2, "{}", op)?;
    }

    btks.to_btks_file(&mut fw, MegamixOp::ENDIAN)?;

    //let mut f = File::open("test_files/ovy9_90.bin")?;
    //extract::extract::<GoldOp>(&mut f, TICKOVY_OFFSET_US, &[])?;

    Ok(())
}
