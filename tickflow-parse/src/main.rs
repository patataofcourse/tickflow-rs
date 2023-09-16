use tickflow_parse::*;

pub fn main() {
    let a = [
        parse_integer("0"),
        parse_integer("1435"),
        parse_integer("0x1000"),
        parse_integer("0o244"),
        parse_integer("0b1010101011"),
        parse_integer("gajdklgadj"),
    ];
    for b in a {
        match b {
            Ok(c) => {dbg!(c.1);},
            Err(e) => println!("{}", e),
        }
    }
}
