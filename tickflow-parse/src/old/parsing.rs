use std::io::Read;

use lazy_static::lazy_static;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, hex_digit1, space0, space1},
    combinator::{eof, opt},
    error::ErrorKind as NomErrorKind,
    error::ParseError,
    multi::{many1, separated_list0},
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

use super::{CommandName, Identifier, Operation, Statement, Value, IDENTIFIER_REGEX};

pub fn parse_from_text(f: &mut impl Read) -> Result<Vec<(usize, Statement)>> {
    let mut text = String::new();
    f.read_to_string(&mut text)?;

    let mut statements = vec![];
    for (i, line) in text.lines().enumerate() {
        if line.trim_start().starts_with("//") || line.is_empty() {
            continue;
        }
        statements.push((
            i,
            read_statement(line.split_once("//").map(|c| c.0).unwrap_or(line).trim(), i)?,
        ));
    }
    Ok(statements)
}

pub fn read_statement(input: &str, line_num: usize) -> Result<Statement> {
    if let Ok((remaining, (_, name, _))) =
        tuple::<_, _, (), _>((tag::<_, _, ()>("#"), ident, space1))(input)
    {
        match name.as_str() {
            "index" | "start" | "assets" => {
                if let Ok((_, (val, _))) = tuple::<_, _, (), _>((integer(line_num), eof))(remaining)
                {
                    Ok(Statement::Directive {
                        name,
                        args: vec![Value::Integer(val?)],
                    })
                } else {
                    Err(OldTfError::SyntaxError.with_ctx(line_num))
                }
            }
            "alias" => {
                match tuple::<_, _, (), _>((ident, space0, integer(line_num), eof))(remaining) {
                    Ok((_, (aname, _, val, _))) => Ok(Statement::Directive {
                        name,
                        args: vec![Value::Constant(aname), Value::Integer(val?)],
                    }),
                    Err(_) => Err(OldTfError::SyntaxError.with_ctx(line_num)),
                }
            }
            "include" => Ok(Statement::Directive {
                name,
                args: vec![Value::String {
                    value: remaining.trim().to_string(),
                    is_unicode: false,
                }],
            }),
            _ => Err(OldTfError::InvalidDirective(name).with_ctx(line_num))?,
        }
    } else if let Ok((_, (name, _, _))) = tuple::<_, _, (), _>((ident, tag(":"), eof))(input) {
        return Ok(Statement::Label(name));
    } else if let Ok((_, (name, _, _, _, value, _))) =
        tuple::<_, _, (), _>((ident, space0, tag("="), space0, value(line_num), eof))(input)
    {
        return Ok(Statement::Constant {
            name,
            value: value?,
        });
    } else {
        let (_, (cmd, arg0, args, _)) = tuple::<_, _, (), _>((
            cmd_name(line_num),
            opt(delimited(pair(space0, tag("<")), value(line_num), tag(">"))),
            opt(pair(
                space1,
                separated_list0(tuple((space0, tag(","), space0)), value(line_num)),
            )),
            eof,
        ))(input)
        .map_err(|_| OldTfError::SyntaxError.with_ctx(line_num))?;
        let args = args.map(|c| c.1).unwrap_or(vec![]);
        return Ok(Statement::Command {
            cmd: cmd?,
            arg0: match arg0 {
                Some(Err(e)) => Err(e)?,
                Some(Ok(c)) => Some(c),
                None => None,
            },
            args: args.into_iter().collect::<Result<Vec<_>>>()?,
        });
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
    line_num: usize,
) -> (&'a str, Result<i32>) {
    (
        remaining,
        crate::read_anysign_int(val, radix)
            .map_err(|c| <_ as Into<OldTfError>>::into(c).with_ctx(line_num)),
    )
}

pub fn integer<'a, E: nom::error::ParseError<&'a str>>(
    line_num: usize,
) -> impl Fn(&'a str) -> IResult<&str, Result<i32>, E> {
    move |input| {
        let (remaining, val) = digit1::<_, E>(input)?;
        Ok(
            if let Ok((remaining, (_, val))) = tuple((tag("0x"), hex_digit1::<_, E>))(input) {
                int_ok::<E>(val, 16, remaining, line_num)
            } else if let Ok((remaining, (_, val))) = tuple((tag("0b"), bin_digit1::<_, E>))(input)
            {
                int_ok::<E>(val, 2, remaining, line_num)
            } else {
                int_ok::<E>(val, 10, remaining, line_num)
            },
        )
    }
}

pub fn cmd_name<'a, E: nom::error::ParseError<&'a str>>(
    line_num: usize,
) -> impl Fn(&'a str) -> IResult<&str, Result<CommandName>, E> {
    move |input| {
        if let Ok((remaining, val)) = integer::<E>(line_num)(input) {
            Ok((remaining, val.map(CommandName::Raw)))
        } else {
            let (remaining, val) = ident(input)?;
            Ok((remaining, Ok(CommandName::Named(val))))
        }
    }
}

pub fn value<'a, E: nom::error::ParseError<&'a str>>(
    line_num: usize,
) -> impl Fn(&'a str) -> IResult<&str, Result<Value>, E> {
    move |input| {
        if let Ok((remaining, (val1, ops))) = tuple::<_, _, E, _>((
            value_no_ops(line_num),
            many1(tuple((
                space0,
                re_find(OP_REGEX.clone()),
                space0,
                value_no_ops(line_num),
            ))),
        ))(input)
        {
            let val1 = match val1 {
                Ok(c) => c,
                Err(e) => return Ok((input, Err(e))),
            };
            panic!("{:?} {:?}", val1, ops);
        } else {
            value_no_ops(line_num)(input)
        }
    }
}

pub fn value_no_ops<'a, E: nom::error::ParseError<&'a str>>(
    line_num: usize,
) -> impl Fn(&'a str) -> IResult<&str, Result<Value>, E> {
    move |input| {
        let out: Value;
        let rem: &str;
        //TODO: order of operations + brackets

        if let Ok((remaining, (_, val, _, _))) = with_matching_brackets(
            '(',
            ')',
            tuple::<_, _, E, _>((space0, value(line_num), space0, eof)),
        )(input)
        {
            let val = match val {
                Ok(c) => c,
                Err(e) => return Ok((input, Err(e))),
            };
            rem = remaining;
            out = val;
        } else if let Ok((remaining, (_, _, val))) =
            tuple::<_, _, E, _>((tag("-"), space0, value(line_num)))(input)
        {
            let val = match val {
                Ok(c) => c,
                Err(e) => return Ok((input, Err(e))),
            };
            rem = remaining;
            out = Value::Negated(Box::new(val));
        } else if let Ok((remaining, value)) = re_capture::<E>(STRING_REGEX.clone())(input) {
            let mut is_unicode = false;

            match value[1] {
                "u" => is_unicode = true,
                "" => {}
                c => {
                    return OldTfError::InvalidStrPrefix(c.to_string())
                        .with_ctx(line_num)
                        .wrap_nom(remaining);
                }
            }
            out = Value::String {
                value: crate::process_escapes(value[2]),
                is_unicode,
            };
            rem = remaining;
        } else if let Ok((remaining, val)) = integer::<E>(line_num)(input) {
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
        /*
        if let Ok((remaining, (_, op, _, val2))) =
            tuple::<_, _, E, _>((space0, re_find(OP_REGEX.clone()), space0, value(line_num)))(rem)
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
        */
        nom_ok(out, rem)
    }
}

pub fn with_matching_brackets<'a, O, E: ParseError<&'a str>>(
    opening: char,
    closing: char,
    mut inner: impl FnMut(&'a str) -> IResult<&'a str, O, E>,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E> {
    move |input| {
        // step 1: ensure equal bracketing
        let mut chars = input.chars();
        let mut depth = 1;
        let mut captured = 0;
        if let Some(c) = chars.next() {
            if opening != c {
                return Err(nom::Err::Error(E::from_error_kind(
                    input,
                    NomErrorKind::Char,
                )));
            }
        } else {
            return Err(nom::Err::Error(E::from_error_kind(
                input,
                NomErrorKind::Eof,
            )));
        }
        while depth > 0 {
            let c = chars.next().ok_or(nom::Err::Error(E::from_error_kind(
                input,
                NomErrorKind::Eof,
            )))?;
            match &c {
                c if c == &opening => depth += 1,
                c if c == &closing => depth -= 1,
                _ => (),
            }
            if depth > 0 {
                captured += 1
            }
        }
        println!("{:?}", &input[1..1 + captured]);

        // step 2: match the inner
        let remaining = chars.as_str();
        inner(&input[1..1 + captured]).map(|(_, val)| (remaining, val))
    }
}
