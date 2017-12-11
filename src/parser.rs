use types::{FStream, JsishResult, JsishError};

use tokenizer::*;
use tokenizer::Token::*;

use ast::*;
use ast::Expression::*;
use ast::Statement::*;
use ast::SourceElement::*;
use ast::Program::*;
use ast::BinaryOperator::*;

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
fn parse_binary_op(
    itr: &mut FStream,
    tk: Token,
    op_pairs: &Vec<(Token, BinaryOperator)>
    ) -> JsishResult<(BinaryOperator, Token)> {

    for &(ref tk1, ref op) in op_pairs {
        if tk == *tk1 {
            return Ok((op.clone(), next_token(itr)?));
        }
    }

    Err(JsishError::from("Could not find token in pair list"))
}

fn parse_binary_expression(
    itr: &mut FStream,
    tk: Token,
    parse_opnd: &Fn(&mut FStream, Token) -> JsishResult<(Expression, Token)>,
    is_opr: &Fn(&Token) -> bool,
    op_pairs: Vec<(Token, BinaryOperator)>
    ) -> JsishResult<(Expression, Token)> {

    let (mut lft, tk1) = parse_opnd(itr, tk)?;
    let mut tk_cursor = tk1;

    while is_opr(&tk_cursor) {
        let (opr, tk2) = parse_binary_op(itr, tk_cursor, &op_pairs)?;
        let (rht, tk3) = parse_opnd(itr, tk2)?;

        lft = ExpBinary(ExpBinaryData {opr: opr, lft: Box::new(lft),
            rht:Box::new(rht)});

        tk_cursor = tk3;
    }

    Ok((lft, tk_cursor))
}

fn parse_expression(
    itr: &mut FStream,
    tk: Token
    ) -> JsishResult<(Expression, Token)> {

    parse_binary_expression(itr, tk, &parse_assignment_expression,
                            &(|x| *x == TkComma),
                            vec![(TkComma, BopComma)])
}

fn parse_assignment_expression(
    itr: &mut FStream,
    tk: Token
    ) -> JsishResult<(Expression, Token)> {

    parse_conditional_expression(itr, tk)

}

fn parse_conditional_expression(
    itr: &mut FStream,
    tk: Token
    ) -> JsishResult<(Expression, Token)> {

    let (guard, tk1) = parse_logical_or_expression(itr, tk)?;

    if tk1 == TkQuestion {
        let tk2 = match_tk(itr, tk1, TkQuestion)?;
        let (then_exp, tk3) = parse_assignment_expression(itr, tk2)?;
        let tk4 = match_tk(itr, tk3, TkColon)?;
        let (else_exp, tk5) = parse_assignment_expression(itr, tk4)?;

        Ok((ExpCond(ExpCondData {guard: Box::new(guard), 
            then_exp: Box::new(then_exp), else_exp: Box::new(else_exp)}), tk5))
    }
    else {
        Ok((guard, tk1))
    }
}


fn parse_logical_or_expression(
    itr: &mut FStream,
    tk: Token
    ) -> JsishResult<(Expression, Token)> {

    parse_binary_expression(itr, tk, &parse_logical_and_expression,
                            &(|x| *x == TkOr),
                            vec![(TkOr, BopOr)])
}

fn parse_logical_and_expression(
    itr: &mut FStream,
    tk: Token
    ) -> JsishResult<(Expression, Token)> {

    parse_binary_expression(itr, tk, &parse_equality_expression,
                            &(|x| *x == TkAnd),
                            vec![(TkAnd, BopAnd)])
}

fn parse_equality_expression(
    itr: &mut FStream,
    tk: Token
    ) -> JsishResult<(Expression, Token)> {

    parse_binary_expression(itr, tk, &parse_relational_expression,
                            &(|x| vec![TkNe, TkEq].contains(x)),
                            vec![(TkNe, BopNe), (TkEq, BopEq)])
}

fn parse_relational_expression(
    itr: &mut FStream,
    tk: Token
    ) -> JsishResult<(Expression, Token)> {

    parse_binary_expression(itr, tk, &parse_additive_expression,
                            &(|x| vec![TkLe, TkLt, TkGt, TkGe].contains(x)),
                            vec![(TkLe, BopLe),
                                 (TkLt, BopLt),
                                 (TkGt, BopGt),
                                 (TkGe, BopGe)])
}

fn parse_additive_expression(
    itr: &mut FStream,
    tk: Token
    ) -> JsishResult<(Expression, Token)> {

    parse_binary_expression(itr, tk, &parse_multiplicative_expression,
                            &(|x| vec![TkPlus, TkMinus].contains(x)),
                            vec![(TkPlus, BopPlus),
                                 (TkMinus, BopMinus)])
}


fn parse_multiplicative_expression(
    itr: &mut FStream,
    tk: Token
    ) -> JsishResult<(Expression, Token)> {

    parse_binary_expression(itr, tk, &parse_unary_expression,
                            &(|x| vec![TkTimes, TkDivide, TkMod].contains(x)),
                            vec![(TkTimes, BopTimes),
                                 (TkDivide, BopDivide),
                                 (TkMod, BopMod)])
}

fn parse_unary_expression(
    itr: &mut FStream,
    tk: Token
    ) -> JsishResult<(Expression, Token)> {

    parse_left_hand_side_expression(itr, tk)
}

fn parse_left_hand_side_expression(
    itr: &mut FStream,
    tk: Token
    ) -> JsishResult<(Expression, Token)> {

    parse_call_expression(itr, tk)
}

fn parse_call_expression(
    itr: &mut FStream,
    tk: Token
    ) -> JsishResult<(Expression, Token)> {

    parse_member_expression(itr, tk)
}

fn parse_member_expression(
    itr: &mut FStream,
    tk: Token
    ) -> JsishResult<(Expression, Token)> {

    parse_primary_expression(itr, tk)
}

fn parse_parenthesized_expression(
    itr: &mut FStream,
    tk: Token
    ) -> JsishResult<(Expression, Token)> {

    let tk1 = match_tk(itr, tk, TkLparen)?;
    let (exp, tk2) = parse_expression(itr, tk1)?;
    let tk3 = match_tk(itr, tk2, TkRparen)?;

    Ok((exp, tk3))
}

fn parse_primary_expression(
    itr: &mut FStream,
    tk: Token
    ) -> JsishResult<(Expression, Token)> {
        
    if tk == TkLparen {
        parse_parenthesized_expression(itr, TkLparen)
    }
    else {
        let exp = match tk {
            TkNum(n) => Ok(ExpNum(n)),
            TkTrue => Ok(ExpTrue),
            TkFalse => Ok(ExpFalse),
            TkString(s) => Ok(ExpString(s)),
            TkUndefined => Ok(ExpUndefined),
            _ => Err(JsishError::from(format!("expected 'value', found '{}'", tk)))
        };

        Ok((exp?, next_token(itr)?))
    }
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

    while is_source_element(&tk_cursor) {
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
