use lazy_static::lazy_static;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, hex_digit1, space0, space1},
    combinator::{eof, opt},
    error::ParseError,
    multi::separated_list0,
    sequence::{delimited, pair, tuple},
    IResult,
};
use regex::Regex;

use crate::better_nom_regex::{re_capture, re_find};
use crate::{
    bin_digit1,
    error::{nom_ok, OldTfError},
    Result,
};

use super::{Command, CommandName, Identifier, Operation, Statement, Value, IDENTIFIER_REGEX};

pub fn read_statement(input: &str) -> Result<Statement> {
    if let Ok((remaining, (_, name, _))) =
        tuple::<_, _, (), _>((tag::<_, _, ()>("#"), ident, space1))(input)
    {
        match name.as_str() {
            "index" | "start" | "assets" => {
                if let Ok((_, (val, _, _))) =
                    tuple::<_, _, (), _>((integer, space0, eof))(remaining)
                {
                    Ok(Statement::Directive {
                        name,
                        args: vec![Value::Integer(val?)],
                    })
                } else {
                    Err(OldTfError::SyntaxError.with_ctx())
                }
            }
            "alias" => match tuple::<_, _, (), _>((ident, space0, integer, space0, eof))(remaining)
            {
                Ok((_, (aname, _, val, _, _))) => Ok(Statement::Directive {
                    name,
                    args: vec![Value::Constant(aname), Value::Integer(val?)],
                }),
                Err(_) => Err(OldTfError::SyntaxError.with_ctx()),
            },
            "include" => Ok(Statement::Directive {
                name,
                args: vec![Value::String {
                    value: remaining.trim().to_string(),
                    is_unicode: false,
                }],
            }),
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
        let (_, (cmd, arg0, args, _, _)) = tuple::<_, _, (), _>((
            cmd_name,
            opt(delimited(pair(space0, tag("<")), value, tag(">"))),
            opt(pair(
                space1,
                separated_list0(tuple((space0, tag(","), space0)), value),
            )),
            space0,
            eof,
        ))(input)
        .map_err(|_| OldTfError::SyntaxError.with_ctx())?;
        let args = args.map(|c| c.1).unwrap_or(vec![]);
        return Ok(Statement::Command(Command {
            cmd: cmd?,
            arg0: match arg0 {
                Some(Err(e)) => Err(e)?,
                Some(Ok(c)) => Some(c),
                None => None,
            },
            args: args.into_iter().collect::<Result<Vec<_>>>()?,
        }));
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
    static ref STRING_REGEX: Regex = Regex::new(r#"^([a-z])?"(([^\\"]|\\.)*)""#).unwrap();
    static ref OP_REGEX: Regex = Regex::new(r"^[+\-*/&|^]|>>|<<").unwrap();
}

pub fn int_ok<'a, E: ParseError<&'a str>>(
    val: &str,
    radix: u32,
    remaining: &'a str,
) -> (&'a str, Result<i32>) {
    (
        remaining,
        crate::read_anysign_int(val, radix).map_err(Into::into),
    )
}

pub fn integer<'a, E: nom::error::ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&str, Result<i32>, E> {
    let (remaining, val) = digit1::<_, E>(input)?;
    Ok(
        if let Ok((remaining, (_, val))) = tuple((tag("0x"), hex_digit1::<_, E>))(input) {
            int_ok::<E>(val, 16, remaining)
        } else if let Ok((remaining, (_, val))) = tuple((tag("0b"), bin_digit1::<_, E>))(input) {
            int_ok::<E>(val, 2, remaining)
        } else {
            int_ok::<E>(val, 10, remaining)
        },
    )
}

pub fn cmd_name<'a, E: nom::error::ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&str, Result<CommandName>, E> {
    if let Ok((remaining, val)) = integer::<E>(input) {
        Ok((remaining, val.map(CommandName::Raw)))
    } else {
        let (remaining, val) = ident(input)?;
        Ok((remaining, Ok(CommandName::Named(val))))
    }
}

pub fn value<'a, E: nom::error::ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&str, Result<Value>, E> {
    let out: Value;
    let rem: &str;

    //TODO: negation

    if let Ok((remaining, value)) = re_capture::<E>(STRING_REGEX.clone())(input) {
        let mut is_unicode = false;

        match value[1] {
            "u" => is_unicode = true,
            "" => {}
            c => {
                return OldTfError::InvalidStrPrefix(c.to_string())
                    .with_ctx()
                    .wrap_nom(remaining);
            }
        }
        out = Value::String {
            value: crate::process_escapes(value[2]),
            is_unicode,
        };
        rem = remaining;
    } else if let Ok((remaining, val)) = integer::<E>(input) {
        if let Ok(c) = val {
            rem = remaining;
            out = Value::Integer(c);
        } else {
            return Ok((remaining, val.map(|_| unreachable!())));
        }
    } else if let Ok((remaining, val)) = ident::<E>(input) {
        out = Value::Constant(val);
        rem = remaining;
    } else {
        return Err(nom::Err::Error(nom::error::make_error(
            input,
            nom::error::ErrorKind::IsNot,
        )));
    }

    // check for operations
    if let Ok((remaining, (_, op, _, val2))) =
        tuple::<_, _, E, _>((space0, re_find(OP_REGEX.clone()), space0, value))(rem)
    {
        let val2 = match val2 {
            Ok(c) => c,
            Err(e) => return e.wrap_nom(remaining),
        };
        nom_ok(
            Value::Operation {
                op: match op {
                    "+" => Operation::Add,
                    "-" => Operation::Sub,
                    "*" => Operation::Mul,
                    "/" => Operation::Div,
                    "<<" => Operation::Shl,
                    ">>" => Operation::Shr,
                    "|" => Operation::Or,
                    "&" => Operation::And,
                    "^" => Operation::Xor,
                    _ => unreachable!(),
                },
                values: [Box::new(out), Box::new(val2)],
            },
            remaining,
        )
    } else {
        nom_ok(out, rem)
    }
}
