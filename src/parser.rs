use types::{FStream, JsishResult, JsishError};

use tokenizer::Token;
use tokenizer::Token::*;
use tokenizer::next_token;

fn match_tk(
    itr: &mut FStream,
    tk: Token,
    expected: Token
) -> JsishResult<Token> {

    if tk == expected {
        next_token(itr)
    }
    else {
        Err(JsishError::from("Bad token"))
    }
}

fn match_num(itr: &mut FStream, tk: Token) -> JsishResult<(i64, Token)> {
    match tk {
        TkNum(n) => Ok((n, next_token(itr)?)),
        _ => Err(JsishError::from("Expected number"))
    }
}

fn match_str(itr: &mut FStream, tk: Token) -> JsishResult<(String, Token)> {
    match tk {
        TkString(s) => Ok((s, next_token(itr)?)),
        _ => Err(JsishError::from("Expected string"))
    }
}

fn match_eof(itr: &mut FStream, tk: Token) -> JsishResult<Token> {
    match tk {
        TkEof => Ok(TkEof),
        _ => Err(JsishError::from("Expected EOF"))
    }
}
