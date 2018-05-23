use std::collections::HashMap;
use parser::syntax::*;

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
            Some(v)
        },
    }
}
