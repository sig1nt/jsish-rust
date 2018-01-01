use types::{JsishResult, JsishError};

use ast::*;
use ast::Expression::*;
use ast::Statement::*;
use ast::SourceElement::*;
use ast::Program::*;
use ast::BinaryOperator::*;
use ast::UnaryOperator::*;

use std::fmt;

use std::collections::HashMap;

#[derive(Clone, PartialEq)]
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

type Environment<'a> = &'a mut HashMap<String, Value>;

// struct Result<'a> {
//     val: Value,
//     env: Environment<'a>
// }

fn value_type_strings(val: &Value) -> String {
    let s = match *val {
        NumValue(_) => "number",
        StringValue(_) => "string",
        BoolValue(_) => "boolean",
        UndefinedValue => "undefined"
    };

    String::from(s)
}

fn unary_error(
    symbol: &str,
    exp: &str,
    act: Value
    ) -> JsishError {

    JsishError::from(format!("unary operator '{}' requires {}, found {}",
                             symbol,
                             exp,
                             value_type_strings(&act)))
}

fn eval_unary_expression(
    opr: UnaryOperator,
    opnd: Expression,
    env: Environment
    ) -> JsishResult<Value> {

    let val = eval_expression(opnd, env)?;
    match (opr, val) {
        (UopNot, BoolValue(b)) => Ok(BoolValue(!b)),
        (UopNot, val) => Err(unary_error("!", "boolean", val)),
        (UopMinus, NumValue(n)) => Ok(NumValue(-n)),
        (UopMinus, val) => Err(unary_error("-", "number", val)),
        (UopTypeof, v) => Ok(StringValue(value_type_strings(&v))),
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

fn handle_short_circuit(
    sc_value: bool,
    symbol: &str,
    lft: Expression,
    rht: Expression,
    env: Environment
    ) -> JsishResult<Value> {

    let lft_val = eval_expression(lft, env)?;

    if let BoolValue(b) = lft_val {
        if b == sc_value {
            Ok(BoolValue(sc_value))
        }
        else {
            let rht_val = eval_expression(rht, env)?;
            if let BoolValue(b) = rht_val {
                Ok(BoolValue(b))
            }
            else {
                Err(JsishError::from(format!("operator '{}' requires \
                                             boolean * boolean, found {} * {}",
                                             symbol,
                                             value_type_strings(&lft_val),
                                             value_type_strings(&rht_val))))
            }
        }
    }
    else {
        Err(JsishError::from(format!("operator '{}' requires boolean, found {}",
                                     symbol,
                                     value_type_strings(&lft_val))))
    }
}

fn eval_binary_expression(
    opr: BinaryOperator,
    lft: Expression,
    rht: Expression,
    env: Environment
    ) -> JsishResult<Value> {

    if opr == BopAnd {
        return handle_short_circuit(false, "&&", lft, rht, env);
    }

    if opr == BopOr {
        return handle_short_circuit(true, "||", lft, rht, env);
    }

    let lft_val = eval_expression(lft, env)?;
    let rht_val = eval_expression(rht, env)?;

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
        (BopComma, _, r) => Ok(r),
        (BopPlus, l, r) =>
            Err(JsishError::from(format!("operator '+' requires number * \
                                         number or string * string, \
                                         found {} * {}",
                                         value_type_strings(&l),
                                         value_type_strings(&r)))),
        (opr, l, r) =>
            Err(JsishError::from(format!("operator '{}' requires number * \
                                         number, found {} * {}",
                                         opr,
                                         value_type_strings(&l),
                                         value_type_strings(&r)))),
    }
}

fn eval_conditional_expression(
    guard: Expression,
    then_exp: Expression,
    else_exp: Expression,
    env: Environment
    ) -> JsishResult<Value> {

    match eval_expression(guard, env)? {
        BoolValue(true) => eval_expression(then_exp, env),
        BoolValue(false) => eval_expression(else_exp, env),
        g_val =>
            Err(JsishError::from(format!("boolean guard required for 'cond' \
                                         expression, found {}",
                                         value_type_strings(&g_val))))
    }
}

fn eval_assignment_expression(
    lft: Expression,
    rht: Expression,
    env: Environment
    ) -> JsishResult<Value> {

    let rht_value = eval_expression(rht, env)?;

    match lft {
        ExpId(id) => {env.insert(id, rht_value.clone()); Ok(rht_value)}
        _ => Err(JsishError::from("unexpected target of assignment\n"))
    }
}

fn eval_expression(exp: Expression, env: Environment) -> JsishResult<Value> {
    match exp {
        ExpId(id) =>
            match env.get(&id) {
                None => Err(JsishError::from(
                        format!("variable '{}' not found", id))),
                Some(v) => Ok(v.clone())
            }
        ExpNum(n) => Ok(NumValue(n)),
        ExpString(s) => Ok(StringValue(s)),
        ExpTrue => Ok(BoolValue(true)),
        ExpFalse => Ok(BoolValue(false)),
        ExpUndefined => Ok(UndefinedValue),
        ExpUnary(ExpUnaryData {opr, opnd})  =>
            eval_unary_expression(opr, *opnd, env),
        ExpBinary(ExpBinaryData {opr, lft, rht}) =>
            eval_binary_expression(opr, *lft, *rht, env),
        ExpCond(ExpCondData {guard, then_exp, else_exp}) =>
            eval_conditional_expression(*guard, *then_exp, *else_exp, env),
        ExpAssign(ExpAssignData {lft, rht}) =>
            eval_assignment_expression(*lft, *rht, env),
        // _ => Ok(UndefinedValue)
    }
}

fn eval_block_statement(
    stmts: Vec<Statement>,
    env: Environment
    ) -> JsishResult<()> {

    for stmt in stmts {
        eval_statement(stmt, env)?;
    }

    Ok(())
}

fn eval_if_statement(
    guard: Expression,
    th: Statement,
    el: Statement,
    env: Environment
    ) -> JsishResult<()> {

    match eval_expression(guard, env)? {
        BoolValue(true) => {eval_statement(th, env)?;},
        BoolValue(false) => {eval_statement(el, env)?;},
        g_val =>
            return Err(JsishError::from(
                    format!("boolean guard required for 'if' statement, \
                            found {}",
                            value_type_strings(&g_val))))
    }
    Ok(())
}

fn eval_while_statement(
    guard: Expression,
    body: Statement,
    env: Environment
    ) -> JsishResult<()> {

    loop {
        match eval_expression(guard.clone(), env)? {
            BoolValue(true) => {eval_statement(body.clone(), env)?;},
            BoolValue(false) => break,
            g_val =>
                return Err(JsishError::from(
                        format!("boolean guard required for 'while' \
                                statement, found {}",
                                value_type_strings(&g_val))))
        }
    }
    Ok(())
}

fn eval_statement(
    stmt: Statement,
    env: Environment
    ) -> JsishResult<(Environment)> {

    match stmt {
        StPrint(exp) => print!("{}", eval_expression(exp, env)?),
        StExp(exp) => {eval_expression(exp, env)?;},
        StBlock(stmts) => eval_block_statement(stmts, env)?,
        StIf(StIfData { guard, th, el }) =>
            eval_if_statement(guard, *th, *el, env)?,
        StWhile(StWhileData { guard, body }) =>
            eval_while_statement(guard, *body, env)?,
        // _ => return Err(JsishError::from("Not yet implemented"))
    }

    Ok(env)
}

fn eval_source_element(
    se: SourceElement,
    env: Environment
    ) -> JsishResult<(Environment)> {

    match se {
        Stmt(s) => eval_statement(s, env)
    }
}

fn eval_program(prog: Program, env: Environment) -> JsishResult<Environment>{
    let Prog(ses) = prog;

    for se in ses {
        eval_source_element(se, env)?;
    }

    Ok(env)
}

pub fn interpret(p: Program) -> JsishResult<()> {
    let mut tle = HashMap::new();
    eval_program(p, &mut tle)?;
    Ok(())
}
