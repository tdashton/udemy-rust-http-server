use std::fmt::{Display, Formatter, Result as FmtResult};

// types that live on the stack can be copied easily
// types that live on the heap are usually only referenced on the stack
// 	e.g. a string on the stack is a reference to the actual contents of the heap

// Copy is for Types whose values can be duplicated simply by copying bits.
// Clone makes a deep copy including the heap data.

#[derive(Copy, Clone, Debug)]
pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
}

impl StatusCode {
    pub fn reason_phrase(&self) -> &str {
        match self {
            Self::Ok => "Ok",
            Self::BadRequest => "Bad Request",
            Self::NotFound => "Not Found",
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", *self as u16)
    }
}
