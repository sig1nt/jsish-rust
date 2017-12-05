use types::{JsishResult, JsishError, FStream};

use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

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
    TkInUse,
    TkNum(i64),
    TkId(String),
    TkString(String),
    TkEof
}

use tokenizer::Token::*;

static keywordTokens: [(&str, Token); 15] =
   [
      ("else", 		TkElse),
      ("false",		TkFalse),
      ("function", 	TkFunction),
      ("if", 		TkIf),
      ("new", 		TkNew),
      ("print", 	TkPrint),
      ("return", 	TkReturn),
      ("this", 		TkThis),
      ("true", 		TkTrue),
      ("typeof", 	TkTypeof),
      ("undefined", TkUndefined),
      ("var", 		TkVar),
      ("while", 	TkWhile),
      ("gc", 		TkGc),
      ("inUse", 	TkInUse)
   ]
;

static symbolTokens: [(&str, Token); 26] =
   [
      ("{", 	TkLbrace),
      ("}", 	TkRbrace),
      ("(", 	TkLparen),
      (")", 	TkRparen),
      ("[", 	TkLbracket),
      ("]", 	TkRbracket),
      (",", 	TkComma),
      (";", 	TkSemi),
      ("?", 	TkQuestion),
      (":", 	TkColon),
      (".", 	TkDot),
      ("+", 	TkPlus),
      ("-", 	TkMinus),
      ("*", 	TkTimes),
      ("/", 	TkDivide),
      ("%", 	TkMod),
      ("&&", 	TkAnd),
      ("||", 	TkOr),
      ("=", 	TkAssign),
      ("==", 	TkEq),
      ("<", 	TkLt),
      ("<=", 	TkLe),
      (">", 	TkGt),
      (">=", 	TkGe),
      ("!", 	TkNot),
      ("!=", 	TkNe)
   ]
;

fn lookahead (itr: &mut FStream) -> JsishResult<char> {
    match itr.peek() {
        None => Err(JsishError::from("Unexpected EOF")),
        Some(&res) => match res {
            Ok(c) => Ok(c as char),
            Err(err) => Err(JsishError::from(err))
        }
    }
}

fn tokenizeIdentifier(itr: &mut FStream) -> JsishResult<Token> {
    let mut token_vec = Vec::new();
    loop {
        let next_char = lookahead(itr)?;
        if next_char.is_alphanumeric() {
            token_vec.push(itr.next().expect("Itr Failure")? as u8);
        }
        else {
            break;
        }
    }

    let tk_str = String::from_utf8(token_vec)?;

	match kwmap.get(&tk_str) {
		Some(tk) => Ok(tk),
		None => Ok(TkId(tk_str))
	}
}

fn diversifyTokens(itr: &mut FStream) -> JsishResult<Token> {
    let next_char = lookahead(itr)?;

    if next_char.is_alphabetic() {
        tokenizeIdentifier(itr)
    }
    else {
        Err(JsishError::from("Unknown token type"))
    }

}

fn recognize_first_token(itr: &mut FStream) -> JsishResult<Token> {
    match itr.peek() {
        None => Ok(TkEof),
        Some(&res) => match res {
            Ok(c) => diversifyTokens(itr),
            Err(err) => Err(JsishError::from(err))
        }
    }
}

fn clear_whitespace(itr: &mut FStream) -> () {
    itr.skip_while(
        |x| match *x { Ok(c) => (c as char).is_whitespace(), Err(_) => false});
}

pub fn nextToken(itr: &mut FStream) -> JsishResult<Token> {
    clear_whitespace(itr);
    recognize_first_token(itr)
}

pub fn createFileStream(filename: String) -> JsishResult<FStream> {
    match File::open(filename) {
        Ok(f) => Ok(f.bytes().peekable()),
        Err(e) => Err(JsishError::from(e))
    }
}
