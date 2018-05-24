use std::collections::HashMap;
use parser::syntax::*;

type Fun = fn(Data) -> Data;

pub enum Data {
    Num(i32),
    Fun(Fun),
}

pub fn eval_ast(ast : &Ast, env: &mut HashMap<&str, Data>) -> Option<Data> {
    match ast {
        Ast::Add(t1, t2) => {
            match (eval_ast(&*t1, env)?, eval_ast(&*t2, env)?) {
                (Data::Num(n1), Data::Num(n2)) => Some(Data::Num(n1 + n2)),
                _ => None,
            }
        },
        Ast::Sub(t1, t2) => {
            match (eval_ast(&*t1, env)?, eval_ast(&*t2, env)?) {
                (Data::Num(n1), Data::Num(n2)) => Some(Data::Num(n1 - n2)),
                _ => None,
            }
        },
        Ast::Mul(t1, t2) => {
            match (eval_ast(&*t1, env)?, eval_ast(&*t2, env)?) {
                (Data::Num(n1), Data::Num(n2)) => Some(Data::Num(n1 * n2)),
                _ => None,
            }
        },
        Ast::Div(t1, t2) => {
            match (eval_ast(&*t1, env)?, eval_ast(&*t2, env)?) {
                (Data::Num(n1), Data::Num(n2)) => Some(Data::Num(n1 / n2)),
                _ => None,
            }
        },
        Ast::App(t1, t2) => {
            match eval_ast(&*t1, env)? {
                Data::Fun(fun) => {
                    let val = eval_ast(&*t2, env)?;
                    Some(fun(val))
                }
                _ => None
            }
        },
        Ast::Var(name) => {
            // let v = env.get(name)?;
            // Some(**v)
            None
        },
        Ast::Num(num) => Some(Data::Num(*num)),

        Ast::Assign(name, ast) => {
            let v = eval_ast(ast, env)?;
            //env.insert(name.to_string(), &v);
            Some(v)
        },
    }
}
