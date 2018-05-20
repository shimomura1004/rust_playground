#[derive(Debug)]
pub enum Term {
    Num(i32),
    Paren(Box<Exp>),
}

#[derive(Debug)]
pub enum Exp1 {
    Add(Box<Term>, Box<Exp1>),
    Sub(Box<Term>, Box<Exp1>),
    Empty,
}

#[derive(Debug)]
pub enum Exp {
    Exp(Box<Term>, Box<Exp1>),
}
