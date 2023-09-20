use std::{collections::HashMap, ops::Deref};

use lazy_static::lazy_static;
use regex::Regex;

use crate::{error::OldTfError, Result};

pub mod parsing;

#[derive(Debug, Clone)]
pub struct Context {
    pub index: Option<i32>,
    pub start: [Option<usize>; 2],
    pub constants: HashMap<Identifier, ParsedValue>,
    pub aliases: HashMap<Identifier, i32>,

    //TODO: labels should be processed before parsed_cmds is fully done, for lookahead
    pub parsed_cmds: Vec<ParsedCommand>,
    pub labels: HashMap<Identifier, usize>,
}

lazy_static! {
    static ref IDENTIFIER_REGEX: Regex = Regex::new("[A-Za-z_$][A-Za-z0-9_$]*").unwrap();
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
    pub fn new(contents: impl ToString) -> Result<Self> {
        let contents = contents.to_string();
        if !IDENTIFIER_REGEX.is_match(&contents) {
            Err(OldTfError::InvalidIdentifier(contents))?
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
    Directive { name: Identifier, args: Vec<Value> },
    Constant { name: Identifier, value: Value },
    Label(Identifier),
    Command(Command),
}

#[derive(Debug, Clone)]
pub struct Command {
    pub cmd: CommandName,
    pub arg0: Option<Value>,
    pub args: Vec<Value>,
}

#[derive(Debug, Clone)]
pub struct ParsedCommand {
    pub cmd: CommandName,
    pub arg0: Option<ParsedValue>,
    pub args: Vec<ParsedValue>,
}

#[derive(Debug, Clone)]
pub enum CommandName {
    Raw(i32),
    Named(Identifier),
}
