use std::collections::HashMap;
use parser::syntax::*;

type Fun = Box<Fn(Data) -> Data>;

pub enum Data {
    Num(i32),
    Fun(Fun),
}

pub fn eval_exp_ast(ast : &ExpAst, env: &HashMap<String, Data>) -> Option<Data> {
    match ast {
        ExpAst::Add(t1, t2) => {
            match (eval_exp_ast(t1, env)?, eval_exp_ast(t2, env)?) {
                (Data::Num(n1), Data::Num(n2)) => Some(Data::Num(n1 + n2)),
                _ => None,
            }
        },
        ExpAst::Sub(t1, t2) => {
            match (eval_exp_ast(t1, env)?, eval_exp_ast(t2, env)?) {
                (Data::Num(n1), Data::Num(n2)) => Some(Data::Num(n1 - n2)),
                _ => None,
            }
        },
        ExpAst::Mul(t1, t2) => {
            match (eval_exp_ast(t1, env)?, eval_exp_ast(t2, env)?) {
                (Data::Num(n1), Data::Num(n2)) => Some(Data::Num(n1 * n2)),
                _ => None,
            }
        },
        ExpAst::Div(t1, t2) => {
            match (eval_exp_ast(t1, env)?, eval_exp_ast(t2, env)?) {
                (Data::Num(n1), Data::Num(n2)) => Some(Data::Num(n1 / n2)),
                _ => None,
            }
        },
        ExpAst::App(t1, t2) => {
            match (eval_exp_ast(t1, env)?, eval_exp_ast(t2, env)?) {
                (Data::Fun(fun), v2) => {
                    Some(fun(v2))
                }
                _ => None
            }
        },
        ExpAst::Var(name) => {
            let v = env.get(name)?;
            Some(v)
        },
        ExpAst::Num(num) => Some(Data::Num(*num)),
    }
}

pub fn eval_statement_ast(ast : &StatementAst, env: &mut HashMap<String, Data>) -> Option<Data> {
    match ast {
        StatementAst::Exp(exp_ast) => eval_exp_ast(exp_ast, env),
        StatementAst::Assign(name, exp_ast) => {
            let val = eval_exp_ast(exp_ast, env)?;
            env.insert(name.to_string(), val);
            None
        },
    }
}