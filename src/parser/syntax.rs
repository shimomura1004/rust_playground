#[derive(Debug)]
pub enum Term {
    Num(i32),
    Var(String),
    Function(String, Box<Exp>),
    Paren(Box<Exp>),
    If(Box<Exp>, Box<Exp>, Box<Exp>),
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

#[derive(Debug, Clone)]
pub enum ExpAst {
    Add(Box<ExpAst>, Box<ExpAst>),
    Sub(Box<ExpAst>, Box<ExpAst>),
    Mul(Box<ExpAst>, Box<ExpAst>),
    Div(Box<ExpAst>, Box<ExpAst>),
    App(Box<ExpAst>, Box<ExpAst>),
    Var(String),
    Num(i32),
    Fun(String, Box<ExpAst>),
    If(Box<ExpAst>, Box<ExpAst>, Box<ExpAst>),
}

#[derive(Debug, Clone)]
pub enum StatementAst {
    Exp(Box<ExpAst>),
    Assign(String, Box<ExpAst>),
}

fn term_to_ast(term : Term) -> ExpAst {
    match term {
        Term::Num(num) => ExpAst::Num(num),
        Term::Paren(exp) => exp_to_ast(*exp),
        Term::Var(name) => ExpAst::Var(name),
        Term::Function(var, exp) => ExpAst::Fun(var, Box::new(exp_to_ast(*exp))),
        Term::If(cond, then_exp, else_exp) => {
            let cond_ast = exp_to_ast(*cond);
            let then_exp_ast = exp_to_ast(*then_exp);
            let else_exp_ast = exp_to_ast(*else_exp);
            ExpAst::If(Box::new(cond_ast), Box::new(then_exp_ast), Box::new(else_exp_ast))
        },
    }
}

fn exp5_to_ast(exp5 : Exp5, ast : ExpAst) -> ExpAst {
    match exp5 {
        Exp5::App(term, exp5) => {
            let term_ast = term_to_ast(*term);
            let ast = ExpAst::App(Box::new(ast), Box::new(term_ast));
            exp5_to_ast(*exp5, ast)
        },
        Exp5::Empty => ast,
    }
}

fn exp4_to_ast(exp4 : Exp4) -> ExpAst {
    let Exp4::Exp4(term, exp5) = exp4;
    let term_ast = term_to_ast(*term);
    exp5_to_ast(*exp5, term_ast)
}

fn exp3_to_ast(exp3 : Exp3, ast : ExpAst) -> ExpAst {
    match exp3 {
        Exp3::Mul(exp4, exp3) => {
            let term_ast = exp4_to_ast(*exp4);
            let ast = ExpAst::Mul(Box::new(ast), Box::new(term_ast));
            exp3_to_ast(*exp3, ast)
        },
        Exp3::Div(exp4, exp3) => {
            let term_ast = exp4_to_ast(*exp4);
            let ast = ExpAst::Div(Box::new(ast), Box::new(term_ast));
            exp3_to_ast(*exp3, ast)
        },
        Exp3::Empty => ast,
    }
}

fn exp2_to_ast(exp2 : Exp2) -> ExpAst {
    let Exp2::Exp2(exp4, exp3) = exp2;
    let term_ast = exp4_to_ast(*exp4);
    exp3_to_ast(*exp3, term_ast)
}

fn exp1_to_ast(exp1 : Exp1, ast : ExpAst) -> ExpAst {
    match exp1 {
        Exp1::Add(exp2, exp1) => {
            let exp2_ast = exp2_to_ast(*exp2);
            let ast = ExpAst::Add(Box::new(ast), Box::new(exp2_ast));
            exp1_to_ast(*exp1, ast)
        },
        Exp1::Sub(exp2, exp1) => {
            let exp2_ast = exp2_to_ast(*exp2);
            let ast = ExpAst::Sub(Box::new(ast), Box::new(exp2_ast));
            exp1_to_ast(*exp1, ast)
        },
        Exp1::Empty => ast,
    }
}

pub fn exp_to_ast(exp : Exp) -> ExpAst {
    let Exp::Exp(exp2, exp1) = exp;
    let exp2_ast = exp2_to_ast(*exp2);
    exp1_to_ast(*exp1, exp2_ast)
}

pub fn statement_to_ast(statement : Statement) -> StatementAst {
    match statement {
        Statement::ExpressionStatement(exp) => StatementAst::Exp(Box::new(exp_to_ast(*exp))),
        Statement::AssignmentStatement(name, exp) => StatementAst::Assign(name, Box::new(exp_to_ast(*exp))),
    }
}
