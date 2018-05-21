use parser::syntax::*;

#[derive(Debug)]
pub enum Ast {
    Add(Box<Ast>, Box<Ast>),
    Sub(Box<Ast>, Box<Ast>),
    Num(i32),
}

fn term_to_ast(term : Term) -> Ast {
    match term {
        Term::Num(num) => Ast::Num(num),
        Term::Paren(exp) => exp_to_ast(*exp),
    }
}

fn exp1_to_ast(exp1 : Exp1, ast : Ast) -> Ast {
    match exp1 {
        Exp1::Add(term, exp1) => {
            let term_ast = term_to_ast(*term);
            let ast = Ast::Add(Box::new(ast), Box::new(term_ast));
            exp1_to_ast(*exp1, ast)
        },
        Exp1::Sub(term, exp1) => {
            let term_ast = term_to_ast(*term);
            let ast = Ast::Sub(Box::new(ast), Box::new(term_ast));
            exp1_to_ast(*exp1, ast)
        },
        Exp1::Empty => ast,
    }
}

pub fn exp_to_ast(exp : Exp) -> Ast {
    let Exp::Exp(term, exp1) = exp;
    let term_ast = term_to_ast(*term);
    exp1_to_ast(*exp1, term_ast)
}
