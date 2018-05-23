#[derive(Debug)]
pub enum Term {
    Num(i32),
    Var(String),
    Function(Vec<String>, Box<Exp>),
    Paren(Box<Exp>),
}

#[derive(Debug)]
pub enum Exp5 {
    App(Box<Term>, Box<Exp5>),
    Empty,
}

#[derive(Debug)]
pub enum Exp4 {
    Exp4(Box<Term>, Box<Exp5>),
}

#[derive(Debug)]
pub enum Exp3 {
    Mul(Box<Exp4>, Box<Exp3>),
    Div(Box<Exp4>, Box<Exp3>),
    Empty,
}

#[derive(Debug)]
pub enum Exp2 {
    Exp2(Box<Exp4>, Box<Exp3>),
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
    AssignmentStatement(String, Box<Exp>),
}

#[derive(Debug)]
pub enum Ast {
    Add(Box<Ast>, Box<Ast>),
    Sub(Box<Ast>, Box<Ast>),
    Mul(Box<Ast>, Box<Ast>),
    Div(Box<Ast>, Box<Ast>),
    App(Box<Ast>, Box<Ast>),
    Var(String),
    Num(i32),

    Assign(String, Box<Ast>),
}

fn term_to_ast(term : Term) -> Ast {
    match term {
        Term::Num(num) => Ast::Num(num),
        Term::Paren(exp) => exp_to_ast(*exp),
        // todo
        Term::Var(name) => Ast::Var(name),
        Term::Function(vars, exp) => Ast::Num(0),
    }
}

fn exp5_to_ast(exp5 : Exp5, ast : Ast) -> Ast {
    match exp5 {
        Exp5::App(term, exp5) => {
            let term_ast = term_to_ast(*term);
            let ast = Ast::App(Box::new(ast), Box::new(term_ast));
            exp5_to_ast(*exp5, ast)
        },
        Exp5::Empty => ast,
    }
}

fn exp4_to_ast(exp4 : Exp4) -> Ast {
    let Exp4::Exp4(term, exp5) = exp4;
    let term_ast = term_to_ast(*term);
    exp5_to_ast(*exp5, term_ast)
}

fn exp3_to_ast(exp3 : Exp3, ast : Ast) -> Ast {
    match exp3 {
        Exp3::Mul(exp4, exp3) => {
            let term_ast = exp4_to_ast(*exp4);
            let ast = Ast::Mul(Box::new(ast), Box::new(term_ast));
            exp3_to_ast(*exp3, ast)
        },
        Exp3::Div(exp4, exp3) => {
            let term_ast = exp4_to_ast(*exp4);
            let ast = Ast::Div(Box::new(ast), Box::new(term_ast));
            exp3_to_ast(*exp3, ast)
        },
        Exp3::Empty => ast,
    }
}

fn exp2_to_ast(exp2 : Exp2) -> Ast {
    let Exp2::Exp2(exp4, exp3) = exp2;
    let term_ast = exp4_to_ast(*exp4);
    exp3_to_ast(*exp3, term_ast)
}

fn exp1_to_ast(exp1 : Exp1, ast : Ast) -> Ast {
    match exp1 {
        Exp1::Add(exp2, exp1) => {
            let exp2_ast = exp2_to_ast(*exp2);
            let ast = Ast::Add(Box::new(ast), Box::new(exp2_ast));
            exp1_to_ast(*exp1, ast)
        },
        Exp1::Sub(exp2, exp1) => {
            let exp2_ast = exp2_to_ast(*exp2);
            let ast = Ast::Sub(Box::new(ast), Box::new(exp2_ast));
            exp1_to_ast(*exp1, ast)
        },
        Exp1::Empty => ast,
    }
}

pub fn exp_to_ast(exp : Exp) -> Ast {
    let Exp::Exp(exp2, exp1) = exp;
    let exp2_ast = exp2_to_ast(*exp2);
    exp1_to_ast(*exp1, exp2_ast)
}

pub fn statement_to_ast(statement : Statement) -> Ast {
    match statement {
        Statement::ExpressionStatement(exp) => exp_to_ast(*exp),
        Statement::AssignmentStatement(name, exp) => Ast::Assign(name, Box::new(exp_to_ast(*exp))),
    }
}
