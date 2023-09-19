use lazy_static::lazy_static;
use regex::Regex;

use crate::{error::Error, Result};

lazy_static! {
    static ref IDENTIFIER_REGEX: Regex = Regex::new("[A-Za-z_$][A-Za-z0-9_$]*").unwrap();
}

pub struct Identifier(String);

impl Identifier {
    pub fn new(contents: impl ToString) -> Result<Self> {
        let contents = contents.to_string();
        if !IDENTIFIER_REGEX.is_match(&contents) {
            return Err(Error::InvalidIdentifier(contents));
        }
        Ok(Self(contents))
    }
}

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

pub enum Statement {
    Directive { name: Identifier, args: Vec<String> },
    Constant {},
}
