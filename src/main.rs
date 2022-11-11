fn main() -> std::io::Result<()> {
    let mut f = std::fs::File::open("test_files/code.bin")?;

    tickflow_rs::extract::extract::<std::fs::File, tickflow_rs::data::megamix::MegamixOp>(
        &mut f,
        0x00100000,
        vec![0x0039a974],
    )?;
    Ok(())
}
