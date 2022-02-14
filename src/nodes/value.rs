use crate::{Span, parsers::Error};

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Num(f64),
    // Str(String),
    // List(Vec<Value>),
    Func(String),
}

impl Value {
    pub fn num(self, span: Span) -> Result<f64, Error> {
        if let Value::Num(x) = self {
            Ok(x)
        } else {
            Err(Error {
                span,
                msg: format!("'{}' is not a number", self),
            })
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Null => write!(f, "null"),
            Self::Bool(x) => write!(f, "{}", x),
            Self::Num(x) => write!(f, "{}", x),
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