#[derive(Debug)]
pub enum Term {
    Num(i32),
    Var(String),
    Function(Vec<String>, Box<Exp>),
    Paren(Box<Exp>),
}

#[derive(Debug)]
pub enum Exp3 {
    Mul(Box<Term>, Box<Exp3>),
    Div(Box<Term>, Box<Exp3>),
    Empty,
}

#[derive(Debug)]
pub enum Exp2 {
    Exp2(Box<Term>, Box<Exp3>),
}

#[derive(Debug)]
pub enum Exp1 {
    Add(Box<Exp2>, Box<Exp1>),
    Sub(Box<Exp2>, Box<Exp1>),
    Empty,
}

#[derive(Debug)]
pub enum Exp {
    Exp(Box<Exp2>, Box<Exp1>),
}

#[derive(Debug)]
pub enum Statement {
    ExpressionStatement(Box<Exp>),
    Assignment(String, Box<Exp>),
}