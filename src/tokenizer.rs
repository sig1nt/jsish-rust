use types::{JsishResult, JsishError, FStream};

use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
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

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match *self {
            TkLbrace => "{",
            TkRbrace => "}",
            TkLparen => "(",
            TkRparen => ")",
            TkLbracket => "[",
            TkRbracket => "]",
            TkComma => ",",
            TkSemi => ";",
            TkQuestion => "?",
            TkColon => ":",
            TkDot => ".",
            TkPlus => "+",
            TkMinus => "-",
            TkTimes => "*",
            TkDivide => "/",
            TkMod => "%",
            TkAnd => "&&",
            TkOr => "||",
            TkAssign => "=",
            TkEq => "==",
            TkLt => "<",
            TkLe => "<=",
            TkGt => ">",
            TkGe => ">=",
            TkNot => "!",
            TkNe => "!=",
            TkElse => "else",
            TkFalse => "false",
            TkFunction => "function",
            TkIf => "if",
            TkNew => "new",
            TkPrint => "print",
            TkReturn => "return",
            TkThis => "this",
            TkTrue => "true",
            TkTypeof => "typeof",
            TkUndefined => "undefined",
            TkVar => "var",
            TkWhile => "while",
            TkGc => "gc",
            TkInUse => "InUse",
            TkEof => "eof",
            TkNum(n) => return write!(f, "{}", n),
            TkId(ref s) => return write!(f, "{}", s),
            TkString(ref s) => return write!(f, "{}", s)
        };

        write!(f, "{}", c)
    }
}

use tokenizer::Token::*;

fn lookahead (itr: &mut FStream) -> JsishResult<char> {
    // Try and just read the file
    if let Some(&Ok(c)) = itr.peek() {
        return Ok(c as char);
    }

    // Something went wrong, so we have to figure out what
    match itr.next() {
        None => Err(JsishError::from("Unexpected EOF")),
        Some(Err(err)) => Err(JsishError::from(err)),
        _ => panic!("Peek and Next have divergent state")
    }
}

fn tokenize_symbol(itr: &mut FStream) -> JsishResult<Token> {
    let mut single_symbols: HashMap<char, Token> =
       vec![
          ('{', 	TkLbrace),
          ('}', 	TkRbrace),
          ('(', 	TkLparen),
          (')', 	TkRparen),
          ('[', 	TkLbracket),
          (']', 	TkRbracket),
          (',', 	TkComma),
          (';', 	TkSemi),
          ('?', 	TkQuestion),
          (':', 	TkColon),
          ('.', 	TkDot),
          ('+', 	TkPlus),
          ('-', 	TkMinus),
          ('*', 	TkTimes),
          ('/', 	TkDivide),
          ('%', 	TkMod),
        ].into_iter().collect()
    ;

    let mut opt_eq_symbols: HashMap<char, (Token, Token)> =
        vec![
          ('=', (TkAssign, TkEq)),
          ('<',	(TkLt, TkLe)),
          ('>',	(TkGt, TkGe)),
          ('!', (TkNot, TkNe)),
       ].into_iter().collect()
    ;

    let c = lookahead(itr)?;

    if let Some(tk) = single_symbols.remove(&c) {
        itr.next();
        return Ok(tk);
    }

    if c == '&' {
        itr.next();
        if lookahead(itr)? == '&' {
            itr.next();
            return Ok(TkAnd);
        }
    }

    if c == '|' {
        itr.next();
        if lookahead(itr)? == '|' {
            itr.next();
            return Ok(TkOr);
        }
    }

    if let Some((wo_eq, w_eq)) = opt_eq_symbols.remove(&c) {
        itr.next();
        if itr.peek().is_some() && lookahead(itr)? == '=' {
            itr.next();
            return Ok(w_eq);
        }
        else {
            return Ok(wo_eq);
        }
    }

    Err(JsishError::from("Unknown token type"))
}

fn recognize_keywords(tk_str: &str) -> Token {
    match tk_str {
      "else" => 		TkElse,
      "false" =>		TkFalse,
      "function" => 	TkFunction,
      "if" => 		    TkIf,
      "new" => 		    TkNew,
      "print" => 	    TkPrint,
      "return" => 	    TkReturn,
      "this" => 		TkThis,
      "true" => 		TkTrue,
      "typeof" => 	    TkTypeof,
      "undefined" =>    TkUndefined,
      "var" => 		    TkVar,
      "while" => 	    TkWhile,
      "gc" => 		    TkGc,
      "inUse" => 	    TkInUse,
      tk_str =>         TkId(String::from(tk_str))
    }
}

fn build_token(
    itr: &mut FStream,
    is_valid: &Fn (char) -> bool
    ) -> JsishResult<String> {

    let mut token_vec = Vec::new();

    loop {
        if is_valid(lookahead(itr)?) {
            token_vec.push(itr.next().expect("Itr Failure")? as u8);
        }
        else {
            break;
        }
    }

    match String::from_utf8(token_vec) {
        Ok(s) => Ok(s),
        Err(err) => Err(JsishError::from(err))
    }
}

fn tokenize_identifier(itr: &mut FStream) -> JsishResult<Token> {
    let id_token = build_token(itr, &(|x| x.is_alphanumeric()))?;

    Ok(recognize_keywords(&id_token))
}

fn tokenize_digits(itr: &mut FStream) -> JsishResult<Token> {
    let num_token = build_token(itr, &(|x| x.is_digit(10)))?;

    Ok(TkNum(i64::from_str_radix(&num_token, 10)?))
}

fn parse_escape(itr: &mut FStream) -> JsishResult<char> {
    itr.next();
    match itr.next() {
        None => Err(JsishError::from("Invalid String")),
        Some(Err(err)) => Err(JsishError::from(err)),
        Some(Ok(c)) => match c as char {
            '\\' => Ok('\\'),
            '\"' => Ok('"'),
            'n' => Ok('\n'),
            'r' => Ok('\r'),
            't' => Ok('\t'),
            'b' => Ok('\x08'),
            'v' => Ok('\x0b'),
            'f' => Ok('\x0c'),
            _ => Err(JsishError::from("Invalid Escape Sequence"))
        }
    }
}
fn tokenize_string(itr: &mut FStream) -> JsishResult<Token> {
    let mut token_vec = Vec::new();
    itr.next();

    loop {
        if let Some(&Ok(c)) = itr.peek() {
            match c as char {
                '\\' => token_vec.push(parse_escape(itr)? as u8),
                '\"' => {itr.next(); break},
                _ => token_vec.push(itr.next().expect("Itr Failure")? as u8)
            }
        }
        else {
            match itr.next() {
                None => return Err(JsishError::from("Invalid String")),
                Some(Err(err)) => return Err(JsishError::from(err)),
                _ => panic!("Peek and Next have divergent state")
            }
        }
    }

    match String::from_utf8(token_vec) {
        Ok(s) => Ok(TkString(s)),
        Err(err) => Err(JsishError::from(err))
    }
}

fn diversify_tokens(itr: &mut FStream) -> JsishResult<Token> {
    let next_char = lookahead(itr)?;

    if next_char.is_alphabetic() {
        tokenize_identifier(itr)
    }
    else if next_char.is_digit(10) {
        tokenize_digits(itr)
    }
    else if next_char == '"' {
        tokenize_string(itr)
    }
    else {
        tokenize_symbol(itr)
    }

}

fn recognize_first_token(itr: &mut FStream) -> JsishResult<Token> {
    if let Some(&Ok(_)) = itr.peek() {
        diversify_tokens(itr)
    }
    else {
        match itr.next() {
            None => Ok(TkEof),
            Some(Err(err)) => Err(JsishError::from(err)),
            _ => panic!("Peek and Next have divergent state")
        }
    }
}

fn clear_whitespace(itr: &mut FStream) -> () {
    loop {
        if let Some(&Ok(c)) = itr.peek() {
            if (c as char).is_whitespace() {
                itr.next();
            }
            else {
                break;
            }
        }
        else {
            break;
        }
    }
}

pub fn next_token(itr: &mut FStream) -> JsishResult<Token> {
    clear_whitespace(itr);
    recognize_first_token(itr)
}

pub fn create_file_stream(filename: &str) -> JsishResult<FStream> {
    match File::open(filename) {
        Ok(f) => Ok(f.bytes().peekable()),
        Err(e) => Err(JsishError::from(e))
    }
}
