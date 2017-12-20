use types::{JsishResult, JsishError};

use ast::*;
use ast::Expression::*;
use ast::Statement::*;
use ast::SourceElement::*;
use ast::Program::*;
use ast::BinaryOperator::*;
use ast::UnaryOperator::*;

use std::fmt;

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
        _ => Err(JsishError::from("Type Error"))
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
        _ => Err(JsishError::from("Type Error"))
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
