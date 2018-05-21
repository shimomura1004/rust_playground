use parser::syntax::*;

#[derive(Debug)]
pub enum Ast {
    Add(Box<Ast>, Box<Ast>),
    Sub(Box<Ast>, Box<Ast>),
    Mul(Box<Ast>, Box<Ast>),
    Div(Box<Ast>, Box<Ast>),
    Num(i32),
}

fn term_to_ast(term : Term) -> Ast {
    match term {
        Term::Num(num) => Ast::Num(num),
        Term::Paren(exp) => exp_to_ast(*exp),
    }
}

fn exp3_to_ast(exp3 : Exp3, ast : Ast) -> Ast {
    match exp3 {
        Exp3::Mul(term, exp3) => {
            let term_ast = term_to_ast(*term);
            let ast = Ast::Mul(Box::new(ast), Box::new(term_ast));
            exp3_to_ast(*exp3, ast)
        },
        Exp3::Div(term, exp3) => {
            let term_ast = term_to_ast(*term);
            let ast = Ast::Div(Box::new(ast), Box::new(term_ast));
            exp3_to_ast(*exp3, ast)
        },
        Exp3::Empty => ast,
    }
}

fn exp2_to_ast(exp2 : Exp2) -> Ast {
    let Exp2::Exp2(term, exp3) = exp2;
    let term_ast = term_to_ast(*term);
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

pub fn eval_ast(ast : &Ast) -> i32 {
    match ast {
        Ast::Add(t1, t2) => eval_ast(&*t1) + eval_ast(&*t2),
        Ast::Sub(t1, t2) => eval_ast(&*t1) - eval_ast(&*t2),
        Ast::Mul(t1, t2) => eval_ast(&*t1) * eval_ast(&*t2),
        Ast::Div(t1, t2) => eval_ast(&*t1) / eval_ast(&*t2),
        Ast::Num(num) => *num,
    }
}