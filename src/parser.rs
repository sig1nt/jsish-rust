use types::{FStream, JsishResult, JsishError};

use tokenizer::*;
use tokenizer::Token::*;

use ast::*;
use ast::Expression::*;
use ast::Statement::*;
use ast::SourceElement::*;
use ast::Program::*;

fn match_tk(
    itr: &mut FStream,
    tk: Token,
    expected: Token
) -> JsishResult<Token> {

    if tk == expected {
        next_token(itr)
    }
    else {
        Err(JsishError::from(
        format!("expected '{}', found '{}'", expected, tk)))
    }
}

fn match_num(itr: &mut FStream, tk: Token) -> JsishResult<(i64, Token)> {
    match tk {
        TkNum(n) => Ok((n, next_token(itr)?)),
        _ => Err(JsishError::from(format!("expected number, found '{}'", tk)))
    }
}

fn match_str(itr: &mut FStream, tk: Token) -> JsishResult<(String, Token)> {
    match tk {
        TkString(s) => Ok((s, next_token(itr)?)),
        _ => Err(JsishError::from(format!("expected string, found '{}'", tk)))
    }
}

fn match_eof(itr: &mut FStream, tk: Token) -> JsishResult<Token> {
    match tk {
        TkEof => Ok(next_token(itr)?),
        _ => Err(JsishError::from(format!("expected 'eof', found '{}'", tk)))
    }
}

// First Sets
fn is_expression(tk: &Token) -> bool {
    match *tk {
        TkLparen => true,
        TkId(_) => true,
        TkNum(_) => true,
        TkString(_) => true,
        TkTrue => true,
        TkFalse => true,
        TkUndefined => true,
        TkNot => true,
        TkTypeof => true,
        TkMinus => true,
        TkFunction => true,
        TkNew => true,
        TkLbrace => true,
        TkThis => true,
        _ => false
    }
}

fn is_assignment_expression(tk: &Token) -> bool {
    is_expression(tk)
}

fn is_expression_statement(tk: &Token) -> bool {
    TkFunction != *tk && TkLbrace != *tk && is_expression(tk)
}

fn is_statement(tk: &Token) -> bool {
    is_expression_statement(tk)
}

fn is_source_element(tk: &Token) -> bool {
    is_statement(tk)
}


// Parsing Functions
fn parse_op(
    itr: &mut FStream,
    tk: Token,
    op_pairs: Vec<(Token, Expression)>
    ) -> JsishResult<(Expression, Token)> {

    for (tk1, exp) in op_pairs {
        if tk == tk1 {
            return Ok((exp, match_tk(itr, tk, tk1)?));
        }
    }

    Err(JsishError::from("Could not find token in pair list"))
}

fn parse_expression(
    itr: &mut FStream,
    tk: Token
    ) -> JsishResult<(Expression, Token)> {

    parse_primary_expression(itr, tk)
}

fn parse_primary_expression(
    itr: &mut FStream,
    tk: Token
    ) -> JsishResult<(Expression, Token)> {

    let exp = match tk {
        TkNum(n) => Ok(ExpNum(n)),
        _ => Err(JsishError::from("Expected value"))
    };

    Ok((exp?, next_token(itr)?))
}

fn parse_expression_statement(
    itr: &mut FStream,
    tk: Token
    ) -> JsishResult<(Statement, Token)> {

    let (exp, tk1) = parse_expression(itr, tk)?;
    let tk2 = match_tk(itr, tk1, TkSemi)?;
    
    Ok((StExp(exp), tk2))
}

fn parse_statement(
    itr: &mut FStream,
    tk: Token
    ) -> JsishResult<(Statement, Token)> {

    if is_expression(&tk) {
        parse_expression_statement(itr, tk)
    }
    else {
        Err(JsishError::from("Expected statement"))
    }
}

fn parse_source_element(
    itr: &mut FStream,
    tk: Token
    ) -> JsishResult<(SourceElement, Token)> {

    let (stmt, tk1) = parse_statement(itr, tk)?;
    Ok((Stmt(stmt), tk1))
}

fn parse_program(
    itr: &mut FStream,
    tk: Token
    ) -> JsishResult<(Program, Token)> {

    let mut elems: Vec<SourceElement> = Vec::new();
    let mut tk_cursor = tk;

    while is_statement(&tk_cursor) {
        let (elem, tk_temp) = parse_source_element(itr, tk_cursor)?;
        elems.push(elem);
        tk_cursor = tk_temp;
    }

    let tk1 = match_eof(itr, tk_cursor)?;

    Ok((Prog(elems), tk1))
}

pub fn parse_stream(itr: &mut FStream) -> JsishResult<Program>{
    let first_token = next_token(itr)?;
    let (prog, _) = parse_program(itr, first_token)?;
    Ok(prog)
}

pub fn parse(filename: &str) -> JsishResult<Program> {
    let mut fstr = create_file_stream(filename)?;
    parse_stream(&mut fstr)
}
