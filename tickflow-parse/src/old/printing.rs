use std::fmt::Display;

impl Display for super::Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Command { cmd, arg0, args } => write!(
                f,
                "{}{}{}",
                cmd,
                if let Some(c) = arg0 {
                    format!("<{c}>")
                } else {
                    String::new()
                },
                if args.is_empty() {
                    String::new()
                } else {
                    " ".to_string()
                        + &args
                            .iter()
                            .map(|c| c.to_string())
                            .collect::<Vec<_>>()
                            .join(", ")
                }
            ),
            Self::Label(l) => write!(f, "{}:", **l),
            Self::Directive { name, args } => {
                write!(
                    f,
                    "#{} {}",
                    **name,
                    // include doesn't take a string as a value which is absolute fuckery
                    if **name == "include" {
                        let super::Value::String { value, .. } = &args[0] else {
                            panic!("old tickflow: #include was given a non-string argument, which was then not validated")
                        };
                        value.clone()
                    } else {
                        args.iter()
                            .map(ToString::to_string)
                            .collect::<Vec<_>>()
                            .join(" ")
                    }
                )
            }
            Self::Constant { name, value } => {
                write!(f, "{} = {}", **name, value)
            }
        }
    }
}

impl Display for super::CommandName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Raw(c) => {
                if c.abs() > 9 {
                    write!(f, "0x{c:x}")
                } else {
                    write!(f, "{c}")
                }
            }
            Self::Named(c) => write!(f, "{}", **c),
        }
    }
}

impl Display for super::Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Operation { op, values } => write!(f, "{} {op} {}", values[0], values[1]),
            Self::String { value, is_unicode } => write!(
                f,
                "{}\"{}\"",
                if *is_unicode { "u" } else { "" },
                crate::create_escapes(value)
            ),
            Self::Negated(c) => write!(f, "-{}", c),
            //TODO: add ability to change hex-ness situationally
            Self::Integer(c) => {
                if c.abs() > 9 {
                    write!(f, "0x{c:x}")
                } else {
                    write!(f, "{c}")
                }
            }
            Self::Constant(c) => write!(f, "{}", **c),
        }
    }
}

impl Display for super::Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Add => "+",
                Self::Sub => "-",
                Self::Mul => "*",
                Self::Div => "/",
                Self::Shl => "<<",
                Self::Shr => ">>",
                Self::And => "&",
                Self::Or => "|",
                Self::Xor => "^",
            }
        )
    }
}
