//! Tickflow (.tickflow / .tkm) language support
//!
//! How to use:
//! 1. Run [`parse_from_text`] on your text file/string value
//! 2. Run the output `Vec<Statement>`, which is a Rust representation of the raw contents of the file, through [Context::parse_file]
//! 3. Use some other library (such as `tickflow-binaries`) to convert to Tickompiler binary or BTKS

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
    pub start: [i32; 2],

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
    pub fn new(contents: impl ToString, line_num: usize) -> Result<Self> {
        let contents = contents.to_string();
        if !IDENTIFIER_REGEX.is_match(&contents) {
            Err(OldTfError::InvalidIdentifier(contents).with_ctx(line_num))?
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

#[derive(Debug, Clone)]
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
    Label(String, usize),
    Command {
        cmd: CommandName,
        arg0: Option<ParsedValue>,
        args: Vec<ParsedValue>,
    },
}

#[derive(Debug, Clone)]
pub enum CommandName {
    Raw(i32),
    Named(Identifier),
}

impl Context {
    pub fn parse_file(
        statements: Vec<Statement>,
        include_fn: impl Fn(String) -> Box<dyn Read>,
    ) -> Result<Self> {
        let mut constants = HashMap::new();

        let DirectiveResult {
            statements,
            index,
            start,
            assets,
            aliases,
        } = Self::preprocess_directives(statements, include_fn)?;

        let Some(index) = index else {
            Err(OldTfError::MissingRequiredDirective("index").with_ctx(1))?
        };
        let Some(start) = start else {
            Err(OldTfError::MissingRequiredDirective("start").with_ctx(1))?
        };
        let Some(assets) = assets else {
            Err(OldTfError::MissingRequiredDirective("assets").with_ctx(1))?
        };

        let mut out = Self {
            index,
            start: [start, assets],
            parsed_cmds: vec![],
        };

        // read file
        for st in statements {
            match st {
                Statement::Constant { name, value } => {
                    constants.insert(name, value);
                }
                Statement::Label(c) => {
                    let position = {
                        let mut size = 0;
                        for a in &out.parsed_cmds {
                            if let ParsedStatement::Command { args, .. } = a {
                                size += 4 * (1 + args.len())
                            }
                        }
                        size
                    };
                    out.parsed_cmds
                        .push(ParsedStatement::Label((*c).clone(), position))
                }
                Statement::Command { cmd, arg0, args } => todo!(),
                Statement::Directive { .. } => unreachable!(),
            }
        }
        todo!()
    }
}

struct DirectiveResult {
    statements: Vec<Statement>,
    index: Option<i32>,
    start: Option<i32>,
    assets: Option<i32>,
    aliases: HashMap<Identifier, i32>,
}

impl Context {
    //TODO: make the return type a struct or smth
    fn preprocess_directives(
        statements: Vec<Statement>,
        include_fn: impl Fn(String) -> Box<dyn Read>,
    ) -> Result<DirectiveResult> {
        let (mut index, mut start, mut assets) = (None, None, None);
        let mut aliases = HashMap::new();

        let mut out_statements = vec![];

        for st in statements {
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
                    "include" => {
                        let mut included_file = include_fn(args[0].unwrap_string().0.clone());
                        let included_file = parse_from_text(&mut included_file)?;
                        let DirectiveResult {
                            statements: included_file,
                            index,
                            start,
                            assets,
                            aliases: included_aliases,
                        } = Self::preprocess_directives(included_file, &include_fn)?;

                        let (None, None, None) = (index, start, assets) else {
                            Err(OldTfError::IncludedDirective.with_ctx(1))?
                        };
                        out_statements.extend(included_file);
                        aliases.extend(included_aliases);
                    }
                    _ => unreachable!(),
                }
            } else {
                out_statements.push(st);
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
