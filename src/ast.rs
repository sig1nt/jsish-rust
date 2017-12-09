#[derive(Clone, Debug)]
pub enum BinaryOperator {
    BopPlus,
    BopMinus,
    BopDivide,
    BopMod,
    BopEq,
    BopNe,
    BopLt,
    BopGt,
    BopGe,
    BopLe,
    BopAnd,
    BopOr,
    BopComma
}

#[derive(Clone, Debug)]
pub enum UnaryOperator {
    UopNot,
    UopTypeof,
    UopMinus
}

#[derive(Clone, Debug)]
pub struct ExpBinaryData {
    opr: BinaryOperator,
    lft: Box<Expression>,
    rht: Box<Expression>
}

#[derive(Clone, Debug)]
pub struct ExpUnaryData {
    opr: UnaryOperator,
    opnd: Box<Expression>
}

#[derive(Clone, Debug)]
pub struct ExpCondData {
    guard: Box<Expression>,
    then_exp: Box<Expression>,
    else_exp: Box<Expression>
}

#[derive(Clone, Debug)]
pub enum Expression {
    ExpNum(i64),
    ExpString(String),
    ExpTrue,
    ExpFalse,
    ExpUndefined,
    ExpBinary(ExpBinaryData),
    ExpUnary(ExpUnaryData),
    ExpCond(ExpCondData)
}

#[derive(Clone, Debug)]
pub enum Statement {
    StExp(Expression)
}

#[derive(Clone, Debug)]
pub enum SourceElement {
    Stmt(Statement)
}

#[derive(Clone, Debug)]
pub enum Program {
    Prog(Vec<SourceElement>)
}
