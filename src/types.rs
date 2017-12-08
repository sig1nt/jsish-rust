use std::error;
use std::fmt;
use std::io;
use std::str;
use std::string;
use std::num;

use std::iter::Peekable;
use std::io::Bytes;
use std::fs::File;

#[derive(Debug)]
pub enum JsishError {
    Message(&'static str),
    IoError(io::Error),
}

pub type JsishResult<T> = Result<T, JsishError>;
pub type FStream = Peekable<Bytes<File>>;

impl error::Error for JsishError {
    fn description(&self) -> &str {
        match *self {
            JsishError::Message(msg) => msg,
            JsishError::IoError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            JsishError::IoError(ref err) => Some(err as &error::Error),
            _ => None,
        }
    }
}

impl fmt::Display for JsishError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            JsishError::Message(msg) => msg.fmt(f),
            JsishError::IoError(ref err) => err.fmt(f),
        }
    }
}

impl From<&'static str> for JsishError {
    fn from(err: &'static str) -> JsishError {
        JsishError::Message(err)
    }
}

impl From<io::Error> for JsishError {
    fn from(err: io::Error) -> JsishError {
        JsishError::IoError(err)
    }
}

impl From<string::FromUtf8Error> for JsishError {
    fn from(_: string::FromUtf8Error) -> JsishError {
        JsishError::Message("Invalid UTF-8 data")
    }
}

impl From<num::ParseIntError> for JsishError {
    fn from(_: num::ParseIntError) -> JsishError {
        JsishError::Message("Invalid integer")
    }
}
