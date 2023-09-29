use tickflow_parse::*;

const TEST_TICKFLOW: &str = include_str!("old/test.tickflow");

pub fn main() {
    let a = [
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
    for b in a {
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
    match old::parse_from_text(&mut TEST_TICKFLOW.bytes().collect::<Vec<u8>>().as_slice()) {
        Ok(c) => {
            for b in c {
                println!("{}", b.1)
            }
        }
        Err(e) => println!("{e}"),
    }
}
