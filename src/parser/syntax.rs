#[derive(Debug)]
pub enum Term<'a> {
    Num(i32),
    Var(&'a str),
    Function(Vec<&'a str>, Box<Exp<'a>>),
    Paren(Box<Exp<'a>>),
}

#[derive(Debug)]
pub enum Exp5<'a> {
    App(Box<Term<'a>>, Box<Exp5<'a>>),
    Empty,
}

#[derive(Debug)]
pub enum Exp4<'a> {
    Exp4(Box<Term<'a>>, Box<Exp5<'a>>),
}

#[derive(Debug)]
pub enum Exp3<'a> {
    Mul(Box<Exp4<'a>>, Box<Exp3<'a>>),
    Div(Box<Exp4<'a>>, Box<Exp3<'a>>),
    Empty,
}

#[derive(Debug)]
pub enum Exp2<'a> {
    Exp2(Box<Exp4<'a>>, Box<Exp3<'a>>),
}

#[derive(Debug)]
pub enum Exp1<'a> {
    Add(Box<Exp2<'a>>, Box<Exp1<'a>>),
    Sub(Box<Exp2<'a>>, Box<Exp1<'a>>),
    Empty,
}

#[derive(Debug)]
pub enum Exp<'a> {
    Exp(Box<Exp2<'a>>, Box<Exp1<'a>>),
}

#[derive(Debug)]
pub enum Statement<'a> {
    ExpressionStatement(Box<Exp<'a>>),
    AssignmentStatement(&'a str, Box<Exp<'a>>),
}

#[derive(Debug)]
pub enum ExpAst<'a> {
    Add(Box<ExpAst<'a>>, Box<ExpAst<'a>>),
    Sub(Box<ExpAst<'a>>, Box<ExpAst<'a>>),
    Mul(Box<ExpAst<'a>>, Box<ExpAst<'a>>),
    Div(Box<ExpAst<'a>>, Box<ExpAst<'a>>),
    App(Box<ExpAst<'a>>, Box<ExpAst<'a>>),
    Var(&'a str),
    Num(i32),
}

pub enum StatementAst<'a> {
    Exp(Box<ExpAst<'a>>),
    Assign(&'a str, Box<ExpAst<'a>>),
}

fn term_to_ast<'a>(term : Term<'a>) -> ExpAst<'a> {
    match term {
        Term::Num(num) => ExpAst::Num(num),
        Term::Paren(exp) => exp_to_ast(*exp),
        // todo
        Term::Var(name) => ExpAst::Var(name),
        Term::Function(vars, exp) => ExpAst::Num(0),
    }
}

fn exp5_to_ast<'a>(exp5 : Exp5<'a>, ast : ExpAst<'a>) -> ExpAst<'a> {
    match exp5 {
        Exp5::App(term, exp5) => {
            let term_ast = term_to_ast(*term);
            let ast = ExpAst::App(Box::new(ast), Box::new(term_ast));
            exp5_to_ast(*exp5, ast)
        },
        Exp5::Empty => ast,
    }
}

fn exp4_to_ast<'a>(exp4 : Exp4<'a>) -> ExpAst<'a> {
    let Exp4::Exp4(term, exp5) = exp4;
    let term_ast = term_to_ast(*term);
    exp5_to_ast(*exp5, term_ast)
}

fn exp3_to_ast<'a>(exp3 : Exp3<'a>, ast : ExpAst<'a>) -> ExpAst<'a> {
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

fn exp2_to_ast<'a>(exp2 : Exp2<'a>) -> ExpAst<'a> {
    let Exp2::Exp2(exp4, exp3) = exp2;
    let term_ast = exp4_to_ast(*exp4);
    exp3_to_ast(*exp3, term_ast)
}

fn exp1_to_ast<'a>(exp1 : Exp1<'a>, ast : ExpAst<'a>) -> ExpAst<'a> {
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

pub fn exp_to_ast<'a>(exp : Exp<'a>) -> ExpAst<'a> {
    let Exp::Exp(exp2, exp1) = exp;
    let exp2_ast = exp2_to_ast(*exp2);
    exp1_to_ast(*exp1, exp2_ast)
}

pub fn statement_to_ast<'a>(statement : Statement<'a>) -> StatementAst<'a> {
    match statement {
        Statement::ExpressionStatement(exp) => StatementAst::Exp(Box::new(exp_to_ast(*exp))),
        Statement::AssignmentStatement(name, exp) => StatementAst::Assign(name, Box::new(exp_to_ast(*exp))),
    }
}
