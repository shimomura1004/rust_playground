pub mod combinator;
use parser::combinator::*;
mod syntax;

//---- TERM --------------------------------------------------------------------
pub struct Num {}
impl Parser<syntax::Term> for Num {
    fn parse<'a>(&self, input : &'a str) -> Result<(syntax::Term, &'a str), ParseError> {
        let (num, input) = Digit{}.parse(input)?;
        Ok((syntax::Term::Num(num), input))
    }
}

pub struct ParenedExpression {}
impl Parser<syntax::Term> for ParenedExpression {
    fn parse<'a>(&self, input : &'a str) -> Result<(syntax::Term, &'a str), ParseError> {
        let (exp, input) = Between {
            left_p: &Char{c:'('},
            mid_p: &Expression{},
            right_p: &Char{c:')'},
        }.parse(input)?;
        Ok((syntax::Term::Paren(Box::new(exp)), input))
    }    
}

pub struct Term {}
impl Parser<syntax::Term> for Term {
    fn parse<'a>(&self, input : &'a str) -> Result<(syntax::Term, &'a str), ParseError> {
        Try {
            ps: vec![
                Box::new(Num{}),
                Box::new(ParenedExpression{}),
            ]
        }.parse(input)
    }
}

pub struct AddExpression {}
impl Parser<syntax::Exp1> for AddExpression {
    fn parse<'a>(&self, input : &'a str) -> Result<(syntax::Exp1, &'a str), ParseError> {
        let (_, input) = Char{c: '+'}.parse(input)?;
        let (term, input) = Term{}.parse(input)?;
        let (exp1, input) = Expression1{}.parse(input)?;
        Ok((syntax::Exp1::Add(Box::new(term), Box::new(exp1)), input))
    }
}

pub struct SubExpression {}
impl Parser<syntax::Exp1> for SubExpression {
    fn parse<'a>(&self, input : &'a str) -> Result<(syntax::Exp1, &'a str), ParseError> {
        let (_, input) = Char{c: '-'}.parse(input)?;
        let (term, input) = Term{}.parse(input)?;
        let (exp1, input) = Expression1{}.parse(input)?;
        Ok((syntax::Exp1::Sub(Box::new(term), Box::new(exp1)), input))
    }
}

pub struct EmptyExpression {}
impl Parser<syntax::Exp1> for EmptyExpression {
    fn parse<'a>(&self, input : &'a str) -> Result<(syntax::Exp1, &'a str), ParseError> {
        Ok((syntax::Exp1::Empty, input))
    }
}

struct Expression1 {}
impl Parser<syntax::Exp1> for Expression1 {
    fn parse<'a>(&self, input : &'a str) -> Result<(syntax::Exp1, &'a str), ParseError> {
        Try{ps: vec![
            Box::new(AddExpression{}),
            Box::new(SubExpression{}),
            Box::new(EmptyExpression{}),
        ]}.parse(input)
    }
}

pub struct Expression {}
impl Parser<syntax::Exp> for Expression {
    fn parse<'a>(&self, input : &'a str) -> Result<(syntax::Exp, &'a str), ParseError> {
        let (term, input) = Term{}.parse(input)?;
        let (exp1, input) = Expression1{}.parse(input)?;
        Ok((syntax::Exp::Exp(Box::new(term), Box::new(exp1)), input))
    }
}

// //---- EXPRESSION --------------------------------------------------------------------
// pub struct TermExpression {}
// impl Parser<syntax::Expression> for TermExpression {
//     fn parse<'a>(&self, input : &'a str) -> Result<(syntax::Expression, &'a str), ParseError> {
//         let (term, input) = Term{}.parse(input)?;
//         Ok((syntax::Expression::Term(Box::new(term)), input))
//     }
// }

// pub struct Plus {}
// impl Parser<syntax::Expression> for Plus {
//     fn parse<'a>(&self, input : &'a str) -> Result<(syntax::Expression, &'a str), ParseError> {
//         let (exp, input) = Expression{}.parse(input)?;
//         let (_, input) = Char{c: '+'}.parse(input)?;
//         let (term, input) = Term{}.parse(input)?;
//         Ok((syntax::Expression::Add(Box::new(exp), Box::new(term)), input))
//     }
// }

// pub struct Expression {}
// impl Parser<syntax::Expression> for Expression {
//     fn parse<'a>(&self, input : &'a str) -> Result<(syntax::Expression, &'a str), ParseError> {
//         Try {
//             ps: vec![
//                 Box::new(TermExpression{}),
//                 Box::new(Plus{}),
//             ]
//         }.parse(input)
//     }
// }

// //---- STATEMENT --------------------------------------------------------------------
// pub struct ExpressionStatement {}
// impl Parser<syntax::Statement> for ExpressionStatement {
//     fn parse<'a>(&self, input : &'a str) -> Result<(syntax::Statement, &'a str), ParseError> {
//         let (exp, input) = Expression{}.parse(input)?;
//         Ok((syntax::Statement::Expression(Box::new(exp)), input))
//     }   
// }


// pub struct Statement {}
// impl Parser<syntax::Statement> for Statement {
//     fn parse<'a>(&self, input : &'a str) -> Result<(syntax::Statement, &'a str), ParseError> {
//         let (statement, input) = ExpressionStatement{}.parse(input)?;
//         Eof{}.parse(input)?;
//         Ok((statement, input))
//     }
// }