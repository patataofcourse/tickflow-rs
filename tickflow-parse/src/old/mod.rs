//! Tickflow (.tickflow / .tkm) language support
//!
//! How to use:
//! 1. Run [`parse_from_text`] on your text file/string value
//! 2. Run the output `Vec<(usize, Statement)>`, which is a Rust representation of the raw contents of the file, through [Context::parse_file]
//! 3. You can use the outputted `Vec<ParsedStatement>` with the main `tickflow` library

//TODO: split the entire module in more files

use std::{collections::HashMap, io::Read, ops::Deref};

use lazy_static::lazy_static;
use regex::Regex;

use crate::{error::OldTfError, Result};

/// [nom] parsers for Tickflow syntax
pub mod parsing;

pub use parsing::parse_from_text;

/// This module doesn't include any new items, just Display definitions for Value, Statement, etc.
mod printing;

#[derive(Debug, Clone)]
pub struct Context {
    pub index: i32,
    pub start: [Option<i32>; 2],

    pub parsed_cmds: Vec<ParsedStatement>,
}

lazy_static! {
    static ref IDENTIFIER_REGEX: Regex = Regex::new("^[A-Za-z_$][A-Za-z0-9_$]*").unwrap();
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier(String);

impl Deref for Identifier {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Identifier {
    pub fn new(contents: impl ToString, fname: &str, line_num: usize) -> Result<Self> {
        let contents = contents.to_string();
        if !IDENTIFIER_REGEX.is_match(&contents) {
            Err(OldTfError::InvalidIdentifier(contents).with_ctx(fname, line_num))?
        } else {
            Ok(Self(contents))
        }
    }

    pub(crate) fn new_unchecked(contents: impl ToString) -> Self {
        Self(contents.to_string())
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Operation {
        op: Operation,
        values: [Box<Value>; 2],
    },
    Negated(Box<Value>),
    Constant(Identifier),
    Integer(i32),
    String {
        value: String,
        is_unicode: bool,
    },
}

impl Value {
    pub fn unwrap_constant(&self) -> &Identifier {
        match self {
            Self::Constant(c) => c,
            _ => panic!("unwrap_constant called on a Value that is not Constant"),
        }
    }

    pub fn unwrap_int(&self) -> &i32 {
        match self {
            Self::Integer(c) => c,
            _ => panic!("unwrap_int called on a Value that is not Integer"),
        }
    }

    pub fn unwrap_string(&self) -> (&String, bool) {
        match self {
            Self::String { value, is_unicode } => (value, *is_unicode),
            _ => panic!("unwrap_string called on a Value that is not String"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ParsedValue {
    Integer(i32),
    String { value: String, is_unicode: bool },
    Label(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
    Shl,
    Shr,
    And,
    Or,
    Xor,
}

impl Operation {
    pub fn apply(self, val1: i32, val2: i32) -> i32 {
        match self {
            Operation::Add => val1 + val2,
            Operation::Sub => val1 - val2,
            Operation::Mul => val1 * val2,
            Operation::Div => val1 / val2,
            Operation::Shl => val1 << val2,
            Operation::Shr => val1 >> val2,
            Operation::And => val1 & val2,
            Operation::Or => val1 | val2,
            Operation::Xor => val1 ^ val2,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Statement {
    Directive {
        name: Identifier,
        args: Vec<Value>,
    },
    Constant {
        name: Identifier,
        value: Value,
    },
    Label(Identifier),
    Command {
        cmd: CommandName,
        arg0: Option<Value>,
        args: Vec<Value>,
    },
}

#[derive(Debug, Clone)]
pub enum ParsedStatement {
    Label(String),
    Command {
        cmd: CommandName,
        arg0: Option<u32>,
        args: Vec<ParsedValue>,
    },
}

#[derive(Debug, Clone)]
pub enum CommandName {
    Raw(i32),
    Named(Identifier),
}

impl Context {
    pub fn parse_file<T: Read>(
        statements: Vec<(usize, Statement)>,
        include_fn: impl Fn(String) -> std::io::Result<T>,
        fname: &str,
    ) -> Result<Self> {
        let mut constants = HashMap::new();

        let DirectiveResult {
            statements,
            index,
            start,
            assets,
            aliases,
        } = Self::preprocess_directives(statements, &include_fn, false, fname)?;

        let Some(index) = index else {
            Err(OldTfError::MissingRequiredDirective("index").with_ctx(fname, 1))?
        };

        let mut parsed_cmds = vec![];

        // read file
        for (l, st) in statements {
            match st {
                Statement::Constant { name, value } => {
                    constants.insert(name, Self::parse_value(&constants, value, fname, l)?);
                }
                Statement::Label(c) => parsed_cmds.push(ParsedStatement::Label(c.0)),
                Statement::Command { cmd, arg0, args } => {
                    let cmd = match &cmd {
                        CommandName::Raw(_) => cmd,
                        CommandName::Named(c) => {
                            if let Some(c) = aliases.get(c) {
                                CommandName::Raw(*c)
                            } else {
                                cmd
                            }
                        }
                    };
                    let arg0 = if let Some(c) = arg0 {
                        //TODO: arg0 must be Integer
                        match Self::parse_value(&constants, c, fname, l)? {
                            ParsedValue::Integer(c) if c == c & ((1 << 18) - 1) => Some(c as u32),
                            ParsedValue::Integer(c) => {
                                Err(OldTfError::OOBArg0(c).with_ctx(fname, l))?
                            }
                            _ => Err(OldTfError::InvalidArg0Type.with_ctx(fname, l))?,
                        }
                    } else {
                        None
                    };
                    let args = args
                        .into_iter()
                        .map(|c| Self::parse_value(&constants, c, fname, l))
                        .collect::<Result<_>>()?;
                    parsed_cmds.push(ParsedStatement::Command { cmd, arg0, args })
                }
                Statement::Directive { .. } => unreachable!(),
            }
        }

        let find_label = |lname| {
            move |c: &ParsedStatement| {
                if let ParsedStatement::Label(name) = c {
                    if name == lname {
                        Some(())
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        };

        let start = if let Some(c) = start {
            Some(c)
        } else if let Some(Some(_)) = parsed_cmds
            .iter()
            .map(find_label("start"))
            .find(Option::is_some)
        {
            None
        } else {
            Err(OldTfError::MissingRequiredDirective("start").with_ctx(fname, 1))?
        };

        let assets = if let Some(c) = assets {
            Some(c)
        } else if let Some(Some(_)) = parsed_cmds
            .iter()
            .map(find_label("assets"))
            .find(Option::is_some)
        {
            None
        } else {
            Some(0)
            //Err(OldTfError::MissingRequiredDirective("assets").with_ctx(fname, 1))?
        };

        Ok(Self {
            index,
            start: [start, assets],
            parsed_cmds,
        })
    }

    fn parse_value(
        constants: &HashMap<Identifier, ParsedValue>,
        value: Value,
        fname: &str,
        line_num: usize,
    ) -> Result<ParsedValue> {
        // difference from what Tickompiler does - labels, instead of being treated as integers with
        // special metadata, are their own type, so operations don't apply to them
        match value {
            Value::Operation { op, values } => {
                let [val1, val2] =
                    values.map(|c| Self::parse_value(constants, *c, fname, line_num));

                if let (ParsedValue::Integer(val1), ParsedValue::Integer(val2)) = (val1?, val2?) {
                    Ok(ParsedValue::Integer(op.apply(val1, val2)))
                } else {
                    Err(OldTfError::InvalidOpType.with_ctx(fname, line_num))?
                }
            }
            Value::Negated(c) => match Self::parse_value(constants, *c, fname, line_num)? {
                ParsedValue::Integer(c) => Ok(ParsedValue::Integer(-c)),
                _ => Err(OldTfError::InvalidOpType.with_ctx(fname, line_num))?,
            },
            //TODO: ensure all labels used exist
            Value::Constant(c) => Ok(constants
                .get(&c)
                .map(Clone::clone)
                .unwrap_or(ParsedValue::Label(c.0))),
            Value::Integer(c) => Ok(ParsedValue::Integer(c)),
            Value::String { value, is_unicode } => Ok(ParsedValue::String { value, is_unicode }),
        }
    }
}

struct DirectiveResult {
    statements: Vec<(usize, Statement)>,
    index: Option<i32>,
    start: Option<i32>,
    assets: Option<i32>,
    aliases: HashMap<Identifier, i32>,
}

impl Context {
    fn preprocess_directives<T: Read>(
        statements: Vec<(usize, Statement)>,
        include_fn: &impl Fn(String) -> std::io::Result<T>,
        is_included_file: bool,
        fname: &str,
    ) -> Result<DirectiveResult> {
        let (mut index, mut start, mut assets) = (None, None, None);
        let mut aliases = HashMap::new();

        let mut out_statements = vec![];

        for (l, st) in statements {
            if let Statement::Directive { name, args } = st {
                // slight difference from what tickompiler does for #start/#assets vs start:/assets: but it's such an
                // edge case no reasonable person should've ever encountered it (and if they have it's a very easy fix)
                match name.as_ref() {
                    "index" => index = Some(*args[0].unwrap_int()),
                    "start" => start = Some(*args[0].unwrap_int()),
                    "assets" => assets = Some(*args[0].unwrap_int()),
                    "alias" => {
                        aliases.insert(args[0].unwrap_constant().clone(), *args[1].unwrap_int());
                    }
                    "include" if !is_included_file => {
                        let fname = args[0].unwrap_string().0;
                        let mut included_file = include_fn(fname.clone())?;
                        let included_file = parse_from_text(fname, &mut included_file)?;
                        let DirectiveResult {
                            statements: included_file,
                            index,
                            start,
                            assets,
                            aliases: included_aliases,
                        } = Self::preprocess_directives(included_file, include_fn, true, fname)?;

                        let (None, None, None) = (index, start, assets) else {
                            Err(OldTfError::IncludedDirective.with_ctx(fname, 1))?
                        };
                        out_statements.extend(included_file);
                        aliases.extend(included_aliases);
                    }
                    "include" if is_included_file => {
                        Err(OldTfError::IncludedDirective.with_ctx(fname, 1))?
                    }
                    _ => unreachable!(),
                }
            } else {
                out_statements.push((l, st));
            }
        }
        Ok(DirectiveResult {
            statements: out_statements,
            index,
            start,
            assets,
            aliases,
        })
    }
}

// Not sure if these will ever actually be needed

impl From<ParsedStatement> for Statement {
    fn from(value: ParsedStatement) -> Self {
        match value {
            ParsedStatement::Label(name) => Statement::Label(Identifier(name)),
            ParsedStatement::Command { cmd, arg0, args } => Statement::Command {
                cmd,
                arg0: arg0.map(|a0| Value::Integer(a0 as i32)),
                args: args.iter().map(|c| c.clone().into()).collect(),
            },
        }
    }
}

impl From<ParsedValue> for Value {
    fn from(value: ParsedValue) -> Self {
        match value {
            ParsedValue::Integer(c) => Value::Integer(c),
            ParsedValue::String { value, is_unicode } => Value::String { value, is_unicode },
            ParsedValue::Label(c) => Value::Constant(Identifier(c)),
        }
    }
}
