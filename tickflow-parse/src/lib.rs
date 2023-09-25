pub mod old;

pub mod error;
pub use error::{Error, Result};

pub(crate) mod better_nom_regex;

use nom::IResult;

fn is_bin_digit(chr: impl nom::AsChar) -> bool {
    let chr = chr.as_char();
    chr == '0' || chr == '1'
}

fn bin_digit1<T, E: nom::error::ParseError<T>>(input: T) -> IResult<T, T, E>
where
    T: nom::InputTakeAtPosition,
    <T as nom::InputTakeAtPosition>::Item: nom::AsChar,
{
    input.split_at_position1_complete(|item| !is_bin_digit(item), nom::error::ErrorKind::Digit)
}

pub fn process_escapes(string: &str) -> String {
    string
        .replace(r"\\", r"\")
        .replace(r"\n", "\n")
        .replace("\\\"", "\"")
}

pub fn create_escapes(string: &str) -> String {
    string
        .replace('\\', r"\\")
        .replace('\n', r"\n")
        .replace('"', "\\\"")
}

pub fn read_anysign_int(src: &str, radix: u32) -> std::result::Result<i32, std::num::IntErrorKind> {
    let number = i64::from_str_radix(src, radix).map_err(|e| e.kind().clone())?;
    if number >= 0 {
        if number > (u32::MAX as i64) {
            Err(std::num::IntErrorKind::PosOverflow)
        } else {
            Ok(number as i32)
        }
    } else if number < (i32::MIN as i64) {
        Err(std::num::IntErrorKind::NegOverflow)
    } else {
        Ok(number as i32)
    }
}
