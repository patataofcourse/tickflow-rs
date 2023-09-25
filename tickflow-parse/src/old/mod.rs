//! Tickflow (.tickflow / .tkm) language support
//!
//! How to use:
//! 1. Run [`parse_from_text`] on your text file/string value
//! 2. Run the output `Vec<Statement>`, which is a Rust representation of the raw contents of the file, through [Context::parse_file]
//! 3. Use some other library (such as `tickflow-binaries`) to convert to Tickompiler binary or BTKS

use std::ops::Deref;

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
    pub index: Option<i32>,
    pub start: [Option<usize>; 2],

    pub parsed_cmds: Vec<ParsedStatement>,
}

lazy_static! {
    static ref IDENTIFIER_REGEX: Regex = Regex::new("^[A-Za-z_$][A-Za-z0-9_$]*").unwrap();
}

#[derive(Debug, Clone)]
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
    Label(String),
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
