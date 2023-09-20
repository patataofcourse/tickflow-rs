use nom::{
    bytes::complete::tag,
    character::complete::{space0, space1},
    combinator::eof,
    sequence::tuple,
    IResult,
};
use nom_regex::str::re_find;

use crate::{error::OldTfError, Result};

use super::{Identifier, Statement, Value, IDENTIFIER_REGEX};

pub fn read_statement(input: &str) -> Result<Statement> {
    if let Ok((remaining, (_, name, _))) =
        tuple::<_, _, (), _>((tag::<_, _, ()>("#"), ident, space1))(input)
    {
        match name.as_str() {
            "index" | "start" | "assets" => todo!(),
            "alias" => todo!(),
            "include" => todo!(),
            _ => Err(OldTfError::InvalidDirective(name))?,
        }
    } else if let Ok((_, (name, _, _, _))) =
        tuple::<_, _, (), _>((ident, tag(":"), space0, eof))(input)
    {
        return Ok(Statement::Label(name));
    } else if let Ok((_, (name, _, _, _, value, _, _))) =
        tuple::<_, _, (), _>((ident, space0, tag("="), space0, value, space0, eof))(input)
    {
        return Ok(Statement::Constant { name, value });
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

pub fn value<'a, E: nom::error::ParseError<&'a str>>(input: &str) -> IResult<&str, Value, E> {
    todo!();
}
