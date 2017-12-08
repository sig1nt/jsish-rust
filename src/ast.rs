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

pub enum UnaryOperator {
    UopNot,
    UopTypeof,
    UopMinus
}

pub struct ExpBinaryData {
    opr: BinaryOperator,
    lft: Box<Expression>,
    rht: Box<Expression>
}

pub struct ExpUnaryData {
    opr: UnaryOperator,
    opnd: Box<Expression>
}

pub struct ExpCondData {
    guard: Box<Expression>,
    then_exp: Box<Expression>,
    else_exp: Box<Expression>
}

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

pub enum Statement {
    StExp(Expression)
}

pub enum SourceElement {
    Stmt(Statement)
}

pub enum Program {
    Program(Vec<SourceElement>)
}
