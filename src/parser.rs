use types::{FStream, JsishResult, JsishError};

use tokenizer::*;
use tokenizer::Token::*;

use ast::*;
use ast::Expression::*;
use ast::Statement::*;
use ast::SourceElement::*;
use ast::Program::*;
use ast::BinaryOperator::*;
use ast::UnaryOperator::*;

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

fn is_expression_statement(tk: &Token) -> bool {
    TkFunction != *tk && TkLbrace != *tk && is_expression(tk)
}

fn is_valid_lhs(tk: &Expression) -> bool {
    match tk {
        &ExpId(_) => true,
        _ => false
    }
}

fn is_statement(tk: &Token) -> bool {
    is_expression_statement(tk) || *tk == TkPrint
}

fn is_source_element(tk: &Token) -> bool {
    is_statement(tk)
}


// Parsing Functions
fn search_for_op<'a, T>(
    tk: &Token,
    op_pairs: &'a Vec<(Token, T)>
    ) -> Option<&'a (Token, T)> {

    for pair in op_pairs {
        let &(ref tk1, _) = pair;
        if *tk == *tk1 {
            return Some(pair);
        }
    }

    None
}

fn parse_binary_expression(
    itr: &mut FStream,
    tk: Token,
    parse_opnd: &Fn(&mut FStream, Token) -> JsishResult<(Expression, Token)>,
    op_pairs: Vec<(Token, BinaryOperator)>
    ) -> JsishResult<(Expression, Token)> {

    let (mut lft, tk1) = parse_opnd(itr, tk)?;
    let mut tk_cursor = tk1;

    loop {
        if let Some(&(_, ref opr)) = search_for_op(&tk_cursor, &op_pairs) {
            let tk2 = next_token(itr)?;
            let (rht, tk3) = parse_opnd(itr, tk2.clone())?;

            lft = ExpBinary(ExpBinaryData {opr: opr.clone(), lft: Box::new(lft),
                rht:Box::new(rht)});

            tk_cursor = tk3;
        }
        else {
            break;
        }
    }

    Ok((lft, tk_cursor))
}

fn parse_expression(
    itr: &mut FStream,
    tk: Token
    ) -> JsishResult<(Expression, Token)> {

    parse_binary_expression(itr, tk, &parse_assignment_expression,
                            vec![(TkComma, BopComma)])
}

fn parse_assignment_expression(
    itr: &mut FStream,
    tk: Token
    ) -> JsishResult<(Expression, Token)> {

    let (lhs, tk1) = parse_conditional_expression(itr, tk)?;

    if tk1 != TkAssign {
        Ok((lhs, tk1))
    } 
    else if !is_valid_lhs(&lhs) {
        Err(JsishError::from("unexpected token '='"))
    }
    else {
        let tk2 = match_tk(itr, tk1, TkAssign)?;
        let (rhs, tk3) = parse_assignment_expression(itr, tk2)?;
        Ok(((ExpAssign(ExpAssignData {lft: Box::new(lhs), 
            rht: Box::new(rhs)})), tk3))
    }
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
                            vec![(TkOr, BopOr)])
}

fn parse_logical_and_expression(
    itr: &mut FStream,
    tk: Token
    ) -> JsishResult<(Expression, Token)> {

    parse_binary_expression(itr, tk, &parse_equality_expression,
                            vec![(TkAnd, BopAnd)])
}

fn parse_equality_expression(
    itr: &mut FStream,
    tk: Token
    ) -> JsishResult<(Expression, Token)> {

    parse_binary_expression(itr, tk, &parse_relational_expression,
                            vec![(TkNe, BopNe), (TkEq, BopEq)])
}

fn parse_relational_expression(
    itr: &mut FStream,
    tk: Token
    ) -> JsishResult<(Expression, Token)> {

    parse_binary_expression(itr, tk, &parse_additive_expression,
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
                            vec![(TkPlus, BopPlus),
                                 (TkMinus, BopMinus)])
}


fn parse_multiplicative_expression(
    itr: &mut FStream,
    tk: Token
    ) -> JsishResult<(Expression, Token)> {

    parse_binary_expression(itr, tk, &parse_unary_expression,
                            vec![(TkTimes, BopTimes),
                                 (TkDivide, BopDivide),
                                 (TkMod, BopMod)])
}

fn parse_unary_expression(
    itr: &mut FStream,
    tk: Token
    ) -> JsishResult<(Expression, Token)> {

    let op_pairs = vec![(TkNot, UopNot), (TkTypeof, UopTypeof),
                        (TkMinus, UopMinus)];

    if let Some(&(_, ref opr)) = search_for_op(&tk, &op_pairs) {
        let tk1 = next_token(itr)?;
        let (opnd, tk2) = parse_left_hand_side_expression(itr, tk1)?;
        Ok((ExpUnary(ExpUnaryData {opr: (*opr).clone(), 
                                   opnd: Box::new(opnd)}), 
            tk2))
    }
    else {
        parse_left_hand_side_expression(itr, tk)
    }
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
            TkId(s) => ExpId(s),
            TkNum(n) => ExpNum(n),
            TkTrue => ExpTrue,
            TkFalse => ExpFalse,
            TkString(s) => ExpString(s),
            TkUndefined => ExpUndefined,
            _ => 
                return Err(JsishError::from(
                        format!("expected 'value', found '{}'", tk)))
        };

        Ok((exp, next_token(itr)?))
    }
}

fn parse_print_statement(
    itr: &mut FStream,
    tk: Token
    ) -> JsishResult<(Statement, Token)> {

    let tk1 = match_tk(itr, tk, TkPrint)?;
    let (exp, tk2) = parse_expression(itr, tk1)?;
    let tk3 = match_tk(itr, tk2, TkSemi)?;

    Ok((StPrint(exp), tk3))
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

    if tk == TkPrint {
        parse_print_statement(itr, tk)
    }
    else if is_expression(&tk) {
        parse_expression_statement(itr, tk)
    }
    else {
        Err(JsishError::from("Expected statement"))
    }
}

// fn parse_variable_elements(
//     itr: &mut FStream,
//     tk: Token
//     ) -> JsishResult<(Vec<Declaration>, Token)> {

//     let mut decls: Vec<Declaration> = Vec::new();
//     let tk1 = match_tk(itr, tk, TkVar)?;

//     Err(JsishError::from("Expected declaration"))
// }

fn parse_source_element(
    itr: &mut FStream,
    tk: Token
    ) -> JsishResult<(SourceElement, Token)> {

    // if tk == TkVar {
    //     let (decl, tk1) = parse_variable_elements(itr, tk)?;
    //     Ok((VarDecl(decl), tk1))
    // }
    // else {
        let (stmt, tk1) = parse_statement(itr, tk)?;
        Ok((Stmt(stmt), tk1))
    // }
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
