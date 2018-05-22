use std::collections::HashMap;
use parser::syntax::*;

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

pub fn statement_to_ast(statement : Statement) -> Ast {
    match statement {
        Statement::ExpressionStatement(exp) => exp_to_ast(*exp),
        Statement::AssignmentStatement(name, exp) => Ast::Assign(name, Box::new(exp_to_ast(*exp))),
    }
}

// pub fn eval_ast(ast : &Ast) -> i32 {
//     match ast {
//         Ast::Add(t1, t2) => eval_ast(&*t1) + eval_ast(&*t2),
//         Ast::Sub(t1, t2) => eval_ast(&*t1) - eval_ast(&*t2),
//         Ast::Mul(t1, t2) => eval_ast(&*t1) * eval_ast(&*t2),
//         Ast::Div(t1, t2) => eval_ast(&*t1) / eval_ast(&*t2),
//         // todo
//         Ast::App(t1, t2) => 0,
//         Ast::Var(name) => 0,
//         Ast::Num(num) => *num,
//     }
// }

pub fn eval_ast(ast : &Ast, env: &mut HashMap<String, Box<Ast>>) -> Option<i32> {
    match ast {
        Ast::Add(t1, t2) => Some(eval_ast(&*t1, env)? + eval_ast(&*t2, env)?),
        Ast::Sub(t1, t2) => Some(eval_ast(&*t1, env)? - eval_ast(&*t2, env)?),
        Ast::Mul(t1, t2) => Some(eval_ast(&*t1, env)? * eval_ast(&*t2, env)?),
        Ast::Div(t1, t2) => Some(eval_ast(&*t1, env)? / eval_ast(&*t2, env)?),
        // todo
        Ast::App(t1, t2) => Some(0),
        Ast::Var(name) => {
            let num = env.get(name)?;
            match **num {
                Ast::Num(num) => Some(num),
                // todo
                _ => None,
            }
        },
        Ast::Num(num) => Some(*num),

        Ast::Assign(name, ast) => {
            let v = eval_ast(ast, env)?;
            env.insert(name.to_string(), Box::new(Ast::Num(v)));
            None
        },
    }
}
