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
            ExpAst::If(cond_ast, then_ast, else_ast) => {
                match self.eval_exp_ast(*cond_ast, bind)? {
                    Data::Num(num) => {
                        if num != 0 {
                            Some(self.eval_exp_ast(*then_ast, bind)?)
                        }
                        else {
                            Some(self.eval_exp_ast(*else_ast, bind)?)
                        }
                    },
                    _ => None,
                }
            },
            _ => {
                println!("??? {:?}", ast);
                None
            },
        }
    }

    pub fn eval_statement_ast(&mut self, ast : StatementAst) -> Option<Data> {
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

    pub fn eval(&mut self, ast : BlockAst) -> Option<Data> {
        match ast {
            BlockAst::Block(statement_asts) => {
                let mut val = None;
                for statement_ast in statement_asts {
                    val = self.eval_statement_ast(statement_ast);
                }
                val
            },
        }
    }
}

use parser;
#[test]
fn test_function_definition_sum() {
    let mut interpreter = Interpreter::new();
    let mut input = "sum = |n| if n then sum (n - 1) + n else 0 end".to_string();

    let statement = parser::Statement::new().parse(&mut input);
    assert!(statement.is_ok());
    let ast =  parser::syntax::statement_to_ast(statement.unwrap());
    let v = interpreter.eval(ast.clone());
    assert!(v.is_some());
    match v.unwrap() {
        Data::Fun(_, _) => assert!(true),
        _ => assert!(false),
    }

    let mut input = "sum(10)".to_string();
    let statement = parser::Statement::new().parse(&mut input);
    assert!(statement.is_ok());
    let ast =  parser::syntax::statement_to_ast(statement.unwrap());
    let v = interpreter.eval(ast.clone());
    assert!(v.is_some());
    match v.unwrap() {
        Data::Num(num) => assert_eq!(num, 55),
        _ => assert!(false),
    }
}

#[test]
fn test_function_definition_fib() {
    let mut interpreter = Interpreter::new();
    let mut input = "fib = |n| if n then if n-1 then fib(n-1) + fib(n-2) else 1 end else 1 end".to_string();

    let statement = parser::Statement::new().parse(&mut input);
    assert!(statement.is_ok());
    let ast =  parser::syntax::statement_to_ast(statement.unwrap());
    let v = interpreter.eval(ast.clone());
    assert!(v.is_some());
    match v.unwrap() {
        Data::Fun(_, _) => assert!(true),
        _ => assert!(false),
    }

    let mut input = "fib(6)".to_string();
    let statement = parser::Statement::new().parse(&mut input);
    assert!(statement.is_ok());
    let ast =  parser::syntax::statement_to_ast(statement.unwrap());
    let v = interpreter.eval(ast.clone());
    assert!(v.is_some());
    match v.unwrap() {
        Data::Num(num) => assert_eq!(num, 13),
        _ => assert!(false),
    }
}
