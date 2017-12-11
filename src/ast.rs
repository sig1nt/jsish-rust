#[derive(Clone, Debug)]
pub enum BinaryOperator {
    BopPlus,
    BopMinus,
    BopTimes,
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
    pub opr: BinaryOperator,
    pub lft: Box<Expression>,
    pub rht: Box<Expression>
}

#[derive(Clone, Debug)]
pub struct ExpUnaryData {
    pub opr: UnaryOperator,
    pub opnd: Box<Expression>
}

#[derive(Clone, Debug)]
pub struct ExpCondData {
    pub guard: Box<Expression>,
    pub then_exp: Box<Expression>,
    pub else_exp: Box<Expression>
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
