use tickflow_parse::*;

pub fn main() {
    let a = [
        old::parsing::read_statement(r#"name = "\"value\n\\" "#),
        old::parsing::read_statement(r#"name = u"\"value\n\\" "#),
        old::parsing::read_statement(r#"name = h"\"value\n\\" "#),
        old::parsing::read_statement(r#"name = 60"#),
        old::parsing::read_statement(r#"name = 0x1f"#),
        old::parsing::read_statement(r#"name = 0b01"#),
        old::parsing::read_statement(r#"name = 2 + 2"#),
        old::parsing::read_statement(r#"name = 2 - 0x10"#),
        old::parsing::read_statement(r#"name = 2 * some_constant"#),
        old::parsing::read_statement(r#"name = "a" / some_constant"#),
        old::parsing::read_statement(r#"name = 2 << some_constant >> 4 & 1 | 2 ^ 3"#),
        old::parsing::read_statement("name = value"),
        old::parsing::read_statement("label:"),
        old::parsing::read_statement("#index 0x69"),
        old::parsing::read_statement("#alias yourmom 0"),
        old::parsing::read_statement("#include thisShouldBeAString.tickflow"),
    ];
    for b in a {
        match b {
            Ok(c) => {
                println!("{c:?}");
            }
            Err(e) => {
                println!("{e}");
            }
        }
    }
}
