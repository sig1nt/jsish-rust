use types::{JsishResult, JsishError};

use ast::*;
use ast::Expression::*;
use ast::Statement::*;
use ast::SourceElement::*;
use ast::Program::*;
use ast::BinaryOperator::*;
use ast::UnaryOperator::*;

use std::fmt;

#[derive(PartialEq)]
enum Value {
    NumValue(i64),
    StringValue(String),
    BoolValue(bool),
    UndefinedValue
}

use self::Value::*;

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NumValue(ref n) => write!(f, "{}", n),
            StringValue(ref s) => write!(f, "{}", s),
            BoolValue(ref b) => write!(f, "{}", b),
            UndefinedValue => write!(f, "undefined"),
        }
    }
}

fn value_type_strings(val: &Value) -> String {
    let s = match *val {
        NumValue(_) => "number",
        StringValue(_) => "string",
        BoolValue(_) => "boolean",
        UndefinedValue => "undefined"
    };

    String::from(s)
}

fn eval_unary_expression(
    opr: UnaryOperator,
    opnd: Expression
    ) -> JsishResult<Value> {

    let val = eval_expression(opnd)?;
    match (opr, val) {
        (UopNot, BoolValue(b)) => Ok(BoolValue(!b)),
        (UopMinus, NumValue(n)) => Ok(NumValue(-n)),
        (UopTypeof, v) => Ok(StringValue(value_type_strings(&v))),
        (opr, val) => Err(JsishError::from(format!("Type Error: {}", opr)))
    }
}

fn special_divide(num: i64, denom: i64) -> i64 {
    if denom == 0 {
        panic!("Cannot divide by zero");
    }

    if (num.is_negative() || denom.is_negative()) && num % denom != 0 {
        ((num as f64) / (denom as f64)).floor() as i64
    }
    else {
        num / denom
    }
}

fn eval_binary_expression(
    opr: BinaryOperator,
    lft: Expression,
    rht: Expression
    ) -> JsishResult<Value> {

    let lft_val = eval_expression(lft)?;
    let rht_val = eval_expression(rht)?;

    match (opr, lft_val, rht_val) {
        (BopPlus, NumValue(l), NumValue(r)) => Ok(NumValue(l + r)),
        (BopPlus, StringValue(l), StringValue(r)) => Ok(StringValue(l + &r)),
        (BopMinus, NumValue(l), NumValue(r)) => Ok(NumValue(l - r)),
        (BopTimes, NumValue(l), NumValue(r)) => Ok(NumValue(l * r)),
        (BopDivide, NumValue(l), NumValue(r)) => 
            Ok(NumValue(special_divide(l, r))),
        (BopMod, NumValue(l), NumValue(r)) => Ok(NumValue(l % r)),
        (BopEq, l, r) => Ok(BoolValue(l == r)),
        (BopNe, l, r) => Ok(BoolValue(l != r)),
        (BopLt, NumValue(l), NumValue(r)) => Ok(BoolValue(l < r)),
        (BopGt, NumValue(l), NumValue(r)) => Ok(BoolValue(l > r)),
        (BopGe, NumValue(l), NumValue(r)) => Ok(BoolValue(l >= r)),
        (BopLe, NumValue(l), NumValue(r)) => Ok(BoolValue(l <= r)),
        (BopAnd, BoolValue(l), BoolValue(r)) => Ok(BoolValue(l && r)),
        (BopOr, BoolValue(l), BoolValue(r)) => Ok(BoolValue(l || r)),
        (BopComma, _, r) => Ok(r),
        (opr, left_val, right_val) => 
            Err(JsishError::from(format!("Type Error: {}", opr)))
    }
}

fn eval_conditional_expression(
    guard: Expression,
    then_exp: Expression,
    else_exp: Expression
    ) -> JsishResult<Value> {

    match eval_expression(guard)? {
        BoolValue(true) => eval_expression(then_exp),
        BoolValue(false) => eval_expression(else_exp),
        g_val => Err(JsishError::from(format!("Type Error: {}", g_val)))
    }
}

fn eval_expression(exp: Expression) -> JsishResult<Value> {
    match exp {
        ExpNum(n) => Ok(NumValue(n)),
        ExpString(s) => Ok(StringValue(s)),
        ExpTrue => Ok(BoolValue(true)),
        ExpFalse => Ok(BoolValue(false)),
        ExpUnary(ExpUnaryData {opr, opnd})  => 
            eval_unary_expression(opr, *opnd),
        ExpBinary(ExpBinaryData {opr, lft, rht}) =>
            eval_binary_expression(opr, *lft, *rht),
        ExpCond(ExpCondData {guard, then_exp, else_exp}) =>
            eval_conditional_expression(*guard, *then_exp, *else_exp),
        _ => Ok(UndefinedValue)
    }
}

fn eval_statement(stmt: Statement) -> JsishResult<()> {
    match stmt {
        StPrint(exp) => Ok(print!("{}", eval_expression(exp)?)),
        StExp(exp) => {eval_expression(exp)?; Ok(())}
    }
}

fn eval_source_element(se: SourceElement) -> JsishResult<()> {
    match se {
        Stmt(s) => eval_statement(s)
    }
}

fn eval_program(prog: Program) -> JsishResult<()>{
    let Prog(ses) = prog;

    for se in ses {
        eval_source_element(se)?;
    }

    Ok(())
}

pub fn interpret(p: Program) -> JsishResult<()> {
    eval_program(p)
}
