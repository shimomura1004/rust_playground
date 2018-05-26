use std::collections::HashMap;
use parser::syntax::*;

type Fun = Box<Fn(Data) -> Data>;

pub enum Data {
    Num(i32),
    Fun(Fun),
}

pub fn eval_exp_ast<'a>(ast : &ExpAst, env: &'a HashMap<&'a str, &'a Data>) -> Option<&'a Data> {
    match ast {
        ExpAst::Add(t1, t2) => {
            match (eval_exp_ast(&*t1, env)?, eval_exp_ast(&*t2, env)?) {
                (Data::Num(n1), Data::Num(n2)) => Some(&Data::Num(n1 + n2)),
                _ => None,
            }
        },
        ExpAst::Sub(t1, t2) => {
            match (eval_exp_ast(&*t1, env)?, eval_exp_ast(&*t2, env)?) {
                (Data::Num(n1), Data::Num(n2)) => Some(&Data::Num(n1 - n2)),
                _ => None,
            }
        },
        ExpAst::Mul(t1, t2) => {
            match (eval_exp_ast(&*t1, env)?, eval_exp_ast(&*t2, env)?) {
                (Data::Num(n1), Data::Num(n2)) => Some(&Data::Num(n1 * n2)),
                _ => None,
            }
        },
        ExpAst::Div(t1, t2) => {
            match (eval_exp_ast(&*t1, env)?, eval_exp_ast(&*t2, env)?) {
                (Data::Num(n1), Data::Num(n2)) => Some(&Data::Num(n1 / n2)),
                _ => None,
            }
        },
        ExpAst::App(t1, t2) => {
            match eval_exp_ast(&*t1, env)? {
                Data::Fun(fun) => {
                    let val = eval_exp_ast(&*t2, env)?;
                    Some(&fun(*val))
                }
                _ => None
            }
        },
        ExpAst::Var(name) => {
            let v = env.get(name)?;
            Some(*v)
        },
        ExpAst::Num(num) => Some(&Data::Num(*num)),
    }
}

pub fn eval_statement_ast<'a>(ast : &'a StatementAst, env: &'a mut HashMap<&'a str, &'a Data>) -> Option<(&'a Data, &'a mut HashMap<&'a str, &'a Data>)> {
    match ast {
        StatementAst::Exp(exp_ast) => {
            let val = eval_exp_ast(exp_ast, env);
            match val {
                Some(val) => Some((val, env)),
                None => None,
            }
        },
        StatementAst::Assign(name, exp_ast) => {
            let val = eval_exp_ast(exp_ast, env);
            match val {
                Some(val) => {
                    env.insert(name, val);
                    Some((val, env))
                },
                None => None,
            }
        },
    }
}