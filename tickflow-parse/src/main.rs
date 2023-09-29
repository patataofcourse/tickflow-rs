use std::fs::File;

use tickflow_parse::*;

const TEST_TICKFLOW: &str = include_str!("old/test.tickflow");

pub fn main() {
    let a = vec![
        old::parsing::read_statement(r#"name = "\"value\n\\""#, 1),
        old::parsing::read_statement(r#"name = u"\"value\n\\""#, 2),
        old::parsing::read_statement(r#"name = h"\"value\n\\""#, 3),
        old::parsing::read_statement("name = 60", 4),
        old::parsing::read_statement("name = 0x1f", 5),
        old::parsing::read_statement("name = 0b01", 6),
        old::parsing::read_statement("name = 2 + 3 * some_constant >> 4 & 1 | 2 ^ 3 << 1", 7),
        old::parsing::read_statement("name = 2 - 2", 8),
        old::parsing::read_statement("name = 2 / some_constant", 9),
        old::parsing::read_statement("name = value", 10),
        old::parsing::read_statement("label:", 11),
        old::parsing::read_statement("#index 0x69", 12),
        old::parsing::read_statement("#alias yourmom 0", 13),
        old::parsing::read_statement("#include thisShouldBeAString.tickflow", 14),
        old::parsing::read_statement("0 1, (((2))), -3, \"4\", five", 15),
        old::parsing::read_statement("cmdname<2> fa", 16),
    ];
    for b in &a {
        match b {
            Ok(c) => {
                //println!("{c:?}");
                println!("{c}");
            }
            Err(e) => {
                println!("//{e}");
            }
        }
    }
    println!("------");
    let b = old::parse_from_text(&mut TEST_TICKFLOW.bytes().collect::<Vec<u8>>().as_slice());
    match b {
        Ok(c) => {
            for d in &c {
                println!("{}", d.1)
            }
            println!("--");
            match old::Context::parse_file(c, File::open) {
                Ok(d) => {
                    println!(
                        "{{ index = {}, start = {}, assets = {} }}",
                        d.index, d.start[0], d.start[1]
                    );
                    for statement in d.parsed_cmds {
                        println!("{:?}", statement);
                    }
                }
                Err(e) => println!("{e}"),
            }
        }
        Err(e) => println!("{e}"),
    }
}
