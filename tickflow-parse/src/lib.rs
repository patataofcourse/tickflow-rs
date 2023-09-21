/// "Old tickflow", .tickflow / .tkm / Tickompiler tickflow
pub mod old;

pub mod error;
pub use error::{Error, Result};

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, hex_digit1, oct_digit1},
    combinator::eof,
    sequence::tuple,
    IResult,
};

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

pub fn parse_integer(input: &str) -> IResult<&str, i64> {
    type ErrorType<'a> = nom::error::Error<&'a str>;
    if let Ok((remaining, (val, _spacing))) = tuple((digit1::<&str, ErrorType>, eof))(input) {
        Ok((remaining, val.parse().unwrap()))
    } else if let Ok((remaining, (_, val, _))) =
        tuple((tag("0x"), hex_digit1::<&str, ErrorType>, eof))(input)
    {
        Ok((remaining, i64::from_str_radix(val, 16).unwrap()))
    } else if let Ok((remaining, (_, val, _))) =
        tuple((tag("0o"), oct_digit1::<&str, ErrorType>, eof))(input)
    {
        Ok((remaining, i64::from_str_radix(val, 8).unwrap()))
    } else if let Ok((remaining, (_, val, _))) =
        tuple((tag("0b"), bin_digit1::<&str, ErrorType>, eof))(input)
    {
        Ok((remaining, i64::from_str_radix(val, 2).unwrap()))
    } else {
        Err(nom::Err::Failure(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Digit,
        )))
    }
}

pub fn process_escapes(string: &str) -> String {
    string
        .replace(r"\\", r"\")
        .replace(r"\n", "\n")
        .replace("\\\"", "\"")
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
