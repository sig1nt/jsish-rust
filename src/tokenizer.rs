use types::{JsishResult, JsishError};

use std::fs::File;
use std::io::prelude::*;
use std::iter::Peekable;
use std::io::Bytes;

pub type FStream = Peekable<Bytes<File>>;

pub enum Token {
    TkLbrace,
    TkRbrace,
    TkLparen,
    TkRparen,
    TkLbracket,
    TkRbracket,
    TkComma,
    TkSemi,
    TkQuestion,
    TkColon,
    TkDot,
    TkPlus,
    TkMinus,
    TkTimes,
    TkDivide,
    TkMod,
    TkAnd,
    TkOr,
    TkAssign,
    TkEq,
    TkLt,
    TkLe,
    TkGt,
    TkGe,
    TkNot,
    TkNe,
    TkElse,
    TkFalse,
    TkFunction,
    TkIf,
    TkNew,
    TkPrint,
    TkReturn,
    TkThis,
    TkTrue,
    TkTypeof,
    TkUndefined,
    TkVar,
    TkWhile,
    TkGc,
    TkInuse,
    TkNum(i64),
    TkId(String),
    TkString(String),
    TkEof
}

fn clear_whitespace(itr: &mut FStream) -> () {
    itr.skip_while(
        |x| match *x { Ok(c) => (c as char).is_whitespace(), Err(_) => false});
}

fn recognize_first_token(itr: &mut FStream) -> JsishResult<Token> {
    match itr.peek() {
        None => Ok(Token::TkEof),
        Some(&res) => match res {
            Ok(_) => Ok(Token::TkIf),
            Err(err) => Err(JsishError::from(err))
        }
    }
}

pub fn nextToken(itr: &mut FStream) -> JsishResult<Token> {
    clear_whitespace(itr);
    recognize_first_token(itr)
}
