use crate::{parsers::Error, Span};

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    I32(i32),
    I64(i64),
    // Str(String),
    // List(Vec<Value>),
    Func(String),
}

impl Value {
    pub fn num_i32(self, span: Span) -> Result<i32, Error> {
        if let Value::I32(x) = self {
            Ok(x)
        } else {
            Err(Error {
                span,
                msg: format!("'{}' is not a number", self),
            })
        }
    }

    pub fn num_i64(self, span: Span) -> Result<i64, Error> {
        if let Value::I64(x) = self {
            Ok(x)
        } else {
            Err(Error {
                span,
                msg: format!("'{}' is not a number", self),
            })
        }
    }

    pub fn bool(self, span: Span) -> Result<bool, Error> {
        if let Value::Bool(b) = self {
            Ok(b)
        } else {
            Err(Error {
                span,
                msg: format!("'{}' is not a bool", self),
            })
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Null => write!(f, "null"),
            Self::Bool(x) => write!(f, "{}", x),
            Self::I32(x) => write!(f, "{}", x),
            Self::I64(x) => write!(f, "{}", x),
            // Self::Str(x) => write!(f, "{}", x),
            // Self::List(xs) => write!(
            //     f,
            //     "[{}]",
            //     xs.iter()
            //         .map(|x| x.to_string())
            //         .collect::<Vec<_>>()
            //         .join(", ")
            // ),
            Self::Func(name) => write!(f, "<function: {}>", name),
        }
    }
}
