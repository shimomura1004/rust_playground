use std::collections::HashMap;
use parser::syntax::*;

type Environment = HashMap<String, Data>;

#[derive(Debug, Clone)]
pub enum Data {
    Num(i32),
    Fun(String, ExpAst),
}

pub struct Interpreter {
    env: Environment,
}
impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter{env: HashMap::new()}
    }

    fn eval_exp_ast(&self, ast : ExpAst, bind : &Environment) -> Option<Data> {
        match ast {
            ExpAst::Add(t1, t2) => {
                match (self.eval_exp_ast(*t1, bind)?, self.eval_exp_ast(*t2, bind)?) {
                    (Data::Num(n1), Data::Num(n2)) => Some(Data::Num(n1 + n2)),
                    _ => None,
                }
            },
            ExpAst::Sub(t1, t2) => {
                match (self.eval_exp_ast(*t1, bind)?, self.eval_exp_ast(*t2, bind)?) {
                    (Data::Num(n1), Data::Num(n2)) => Some(Data::Num(n1 - n2)),
                    _ => None,
                }
            },
            ExpAst::Mul(t1, t2) => {
                match (self.eval_exp_ast(*t1, bind)?, self.eval_exp_ast(*t2, bind)?) {
                    (Data::Num(n1), Data::Num(n2)) => Some(Data::Num(n1 * n2)),
                    _ => None,
                }
            },
            ExpAst::Div(t1, t2) => {
                match (self.eval_exp_ast(*t1, bind)?, self.eval_exp_ast(*t2, bind)?) {
                    (Data::Num(n1), Data::Num(n2)) => Some(Data::Num(n1 / n2)),
                    _ => None,
                }
            },
            ExpAst::App(t1, t2) => {
                match (self.eval_exp_ast(*t1, bind)?, self.eval_exp_ast(*t2, bind)?) {
                    (Data::Fun(var, body), v2) => {
                        let mut new_bind = bind.clone();
                        new_bind.insert(var, v2);
                        self.eval_exp_ast(body, &new_bind)
                    }
                    _ => None
                }
            },
            ExpAst::Var(name) => {
                match bind.get(&name) {
                    Some(v) => Some(v.clone()),
                    None => {
                        let v = self.env.get(&name)?;
                        Some(v.clone())
                    }
                }
            },
            ExpAst::Fun(vars, exp) => Some(Data::Fun(vars, *exp)),
            ExpAst::Num(num) => Some(Data::Num(num)),
            _ => None,
        }
    }

    pub fn eval(&mut self, ast : StatementAst) -> Option<Data> {
        match ast {
            StatementAst::Exp(exp_ast) => self.eval_exp_ast(*exp_ast, &HashMap::new()),
            StatementAst::Assign(name, exp_ast) => {
                match self.eval_exp_ast(*exp_ast, &HashMap::new()) {
                    Some(val) => {
                        self.env.insert(name, val.clone());
                        Some(val)
                    },
                    None => None,
                }
            },
        }
    }
}
