use lazy_static::lazy_static;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, hex_digit1, space0, space1},
    combinator::eof,
    sequence::tuple,
    IResult,
};
use nom_regex::str::{re_capture, re_find};
use regex::Regex;

use crate::{
    bin_digit1,
    error::{nom_ok, OldTfError},
    Result,
};

use super::{Identifier, Statement, Value, IDENTIFIER_REGEX};

pub fn read_statement(input: &str) -> Result<Statement> {
    if let Ok((remaining, (_, name, _))) =
        tuple::<_, _, (), _>((tag::<_, _, ()>("#"), ident, space1))(input)
    {
        match name.as_str() {
            "index" | "start" | "assets" => todo!(),
            "alias" => todo!(),
            "include" => todo!(),
            _ => Err(OldTfError::InvalidDirective(name).with_ctx())?,
        }
    } else if let Ok((_, (name, _, _, _))) =
        tuple::<_, _, (), _>((ident, tag(":"), space0, eof))(input)
    {
        return Ok(Statement::Label(name));
    } else if let Ok((_, (name, _, _, _, value, _, _))) =
        tuple::<_, _, (), _>((ident, space0, tag("="), space0, value, space0, eof))(input)
    {
        return Ok(Statement::Constant {
            name,
            value: value?,
        });
    } else {
        // commands
        todo!()
    }
}

pub fn ident<'a, E: nom::error::ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&str, Identifier, E> {
    let match_id = re_find(IDENTIFIER_REGEX.clone());
    let (remaining, ident) = match_id(input)?;
    Ok((remaining, Identifier::new_unchecked(ident)))
}

lazy_static! {
    static ref STRING_REGEX: Regex = Regex::new(r#"([a-z])?"(([^\\"]|\\.)*)""#).unwrap();
}

//TODO: operations
pub fn value<'a, E: nom::error::ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&str, Result<Value>, E> {
    if let Ok((remaining, value)) = re_capture::<E>(STRING_REGEX.clone())(input) {
        println!("{:?}", value);
        let mut is_unicode = false;

        //TODO: check for optional captures
        match value[1] {
            "u" => is_unicode = true,
            c => {
                return OldTfError::InvalidStrPrefix(c.to_string())
                    .with_ctx()
                    .wrap_nom(remaining);
            }
        }
        return nom_ok(
            Value::String {
                value: crate::process_escapes(value[2]),
                is_unicode,
            },
            remaining,
        );
    } else if let Ok((remaining, value)) = digit1::<_, E>(input) {
        // integer, check all types
        if let Ok((remaining, (_, val))) = tuple((tag("0x"), hex_digit1::<_, E>))(input) {
            return Ok((
                remaining,
                crate::read_anysign_int(val, 16)
                    .map(Value::Integer)
                    .map_err(Into::into),
            ));
        } else if let Ok((remaining, (_, val))) = tuple((tag("0b"), bin_digit1::<_, E>))(input) {
            return Ok((
                remaining,
                crate::read_anysign_int(val, 2)
                    .map(Value::Integer)
                    .map_err(Into::into),
            ));
        } else {
            return Ok((
                remaining,
                crate::read_anysign_int(value, 2)
                    .map(Value::Integer)
                    .map_err(Into::into),
            ));
        }
    }
    todo!()
}
