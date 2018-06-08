use std::collections::HashMap;
use parser::syntax::*;

type Fun = Box<Fn(Data) -> Data>;

pub enum Data {
    Num(i32),
    Fun(Fun),
}

pub struct Interpreter {
    env: HashMap<String, Data>,
}
impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter{env: HashMap::new()}
    }

    fn eval_exp_ast(&self, ast : &ExpAst) -> Option<Data> {
        match ast {
            ExpAst::Add(t1, t2) => {
                match (self.eval_exp_ast(t1)?, self.eval_exp_ast(t2)?) {
                    (Data::Num(n1), Data::Num(n2)) => {
                        let n = n1 + n2;
                        Some(Data::Num(n))
                        //Some(&Data::Num(n1 + n2))
                    },
                    _ => None,
                }
            },
            // ExpAst::Sub(t1, t2) => {
            //     match (self.eval_exp_ast(t1)?, self.eval_exp_ast(t2)?) {
            //         (Data::Num(n1), Data::Num(n2)) => Some(&Data::Num(n1 - n2)),
            //         _ => None,
            //     }
            // },
            // ExpAst::Mul(t1, t2) => {
            //     match (*eval_exp_ast(t1, env)?, *eval_exp_ast(t2, env)?) {
            //         (Data::Num(n1), Data::Num(n2)) => Some(Box::new(&Data::Num(n1 * n2))),
            //         _ => None,
            //     }
            // },
            // ExpAst::Div(t1, t2) => {
            //     match (*eval_exp_ast(t1, env)?, *eval_exp_ast(t2, env)?) {
            //         (Data::Num(n1), Data::Num(n2)) => Some(Box::new(&Data::Num(n1 / n2))),
            //         _ => None,
            //     }
            // },
            // ExpAst::App(t1, t2) => {
            //     match (*eval_exp_ast(t1, env)?, *eval_exp_ast(t2, env)?) {
            //         (Data::Fun(fun), v2) => {
            //             Some(Box::new(&fun(*v2)))
            //         }
            //         _ => None
            //     }
            // },
            ExpAst::Var(name) => {
                let v = self.env.get(name)?;
                Some(*v)
            },
            ExpAst::Num(num) => Some(Data::Num(*num)),
            _ => None
        }
    }

    pub fn eval(&mut self, ast : &StatementAst) -> Option<Data> {
        match ast {
            StatementAst::Exp(exp_ast) => self.eval_exp_ast(&exp_ast),
            StatementAst::Assign(name, exp_ast) => {
                match self.eval_exp_ast(&exp_ast) {
                    Some(val) => {
                        // self.env.insert(name.to_string(), val);
                        None
                    },
                    None => None,
                }
            },
        }
    }
}

// pub fn eval_exp_ast<'a>(ast : &ExpAst, env: &'a Environment) -> Option<Box<&'a Data>> {
//     match ast {
//         ExpAst::Add(t1, t2) => {
//             match (*eval_exp_ast(t1, env)?, *eval_exp_ast(t2, env)?) {
//                 (Data::Num(n1), Data::Num(n2)) => Some(Box::new(&Data::Num(n1 + n2))),
//                 _ => None,
//             }
//         },
//         ExpAst::Sub(t1, t2) => {
//             match (*eval_exp_ast(t1, env)?, *eval_exp_ast(t2, env)?) {
//                 (Data::Num(n1), Data::Num(n2)) => Some(Box::new(&Data::Num(n1 - n2))),
//                 _ => None,
//             }
//         },
//         ExpAst::Mul(t1, t2) => {
//             match (*eval_exp_ast(t1, env)?, *eval_exp_ast(t2, env)?) {
//                 (Data::Num(n1), Data::Num(n2)) => Some(Box::new(&Data::Num(n1 * n2))),
//                 _ => None,
//             }
//         },
//         ExpAst::Div(t1, t2) => {
//             match (*eval_exp_ast(t1, env)?, *eval_exp_ast(t2, env)?) {
//                 (Data::Num(n1), Data::Num(n2)) => Some(Box::new(&Data::Num(n1 / n2))),
//                 _ => None,
//             }
//         },
//         ExpAst::App(t1, t2) => {
//             match (*eval_exp_ast(t1, env)?, *eval_exp_ast(t2, env)?) {
//                 (Data::Fun(fun), v2) => {
//                     Some(Box::new(&fun(*v2)))
//                 }
//                 _ => None
//             }
//         },
//         ExpAst::Var(name) => {
//             let v = env.get(name.to_string())?;
//             Some(Box::new(&*v))
//         },
//         ExpAst::Num(num) => Some(Box::new(&Data::Num(*num))),
//         // _ => None
//     }
// }

// pub fn eval_statement_ast<'a>(ast : StatementAst, env: &'a mut Environment) -> Option<Box<&'a Data>> {
//     match ast {
//         StatementAst::Exp(exp_ast) => eval_exp_ast(&exp_ast, &env),
//         StatementAst::Assign(name, exp_ast) => {
//             match eval_exp_ast(&exp_ast, &env) {
//                 Some(val) => {
//                     //env.insert(name.to_string(), *val);
                    
//                     None
//                 },
//                 None => None,
//             }
//         },
//     }
// }