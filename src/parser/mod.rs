pub mod combinator;
use parser::combinator::*;
pub mod syntax;

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

pub struct MulExpression {}
impl Parser<syntax::Exp3> for MulExpression {
    fn parse<'a>(&self, input : &'a str) -> Result<(syntax::Exp3, &'a str), ParseError> {
        let (_, input) = Char{c: '*'}.parse(input)?;
        let (term, input) = Term{}.parse(input)?;
        let (exp3, input) = Expression3{}.parse(input)?;
        Ok((syntax::Exp3::Mul(Box::new(term), Box::new(exp3)), input))
    }
}

pub struct DivExpression {}
impl Parser<syntax::Exp3> for DivExpression {
    fn parse<'a>(&self, input : &'a str) -> Result<(syntax::Exp3, &'a str), ParseError> {
        let (_, input) = Char{c: '/'}.parse(input)?;
        let (term, input) = Term{}.parse(input)?;
        let (exp3, input) = Expression3{}.parse(input)?;
        Ok((syntax::Exp3::Div(Box::new(term), Box::new(exp3)), input))
    }
}

pub struct EmptyExpression3 {}
impl Parser<syntax::Exp3> for EmptyExpression3 {
    fn parse<'a>(&self, input : &'a str) -> Result<(syntax::Exp3, &'a str), ParseError> {
        Ok((syntax::Exp3::Empty, input))
    }
}

struct Expression3 {}
impl Parser<syntax::Exp3> for Expression3 {
    fn parse<'a>(&self, input : &'a str) -> Result<(syntax::Exp3, &'a str), ParseError> {
        Try{ps: vec![
            Box::new(MulExpression{}),
            Box::new(DivExpression{}),
            Box::new(EmptyExpression3{}),
        ]}.parse(input)
    }
}
pub struct Expression2 {}
impl Parser<syntax::Exp2> for Expression2 {
    fn parse<'a>(&self, input : &'a str) -> Result<(syntax::Exp2, &'a str), ParseError> {
        let (term, input) = Term{}.parse(input)?;
        let (exp3, input) = Expression3{}.parse(input)?;
        Ok((syntax::Exp2::Exp2(Box::new(term), Box::new(exp3)), input))
    }
}

pub struct AddExpression {}
impl Parser<syntax::Exp1> for AddExpression {
    fn parse<'a>(&self, input : &'a str) -> Result<(syntax::Exp1, &'a str), ParseError> {
        let (_, input) = Char{c: '+'}.parse(input)?;
        let (exp2, input) = Expression2{}.parse(input)?;
        let (exp1, input) = Expression1{}.parse(input)?;
        Ok((syntax::Exp1::Add(Box::new(exp2), Box::new(exp1)), input))
    }
}

pub struct SubExpression {}
impl Parser<syntax::Exp1> for SubExpression {
    fn parse<'a>(&self, input : &'a str) -> Result<(syntax::Exp1, &'a str), ParseError> {
        let (_, input) = Char{c: '-'}.parse(input)?;
        let (exp2, input) = Expression2{}.parse(input)?;
        let (exp1, input) = Expression1{}.parse(input)?;
        Ok((syntax::Exp1::Sub(Box::new(exp2), Box::new(exp1)), input))
    }
}

pub struct EmptyExpression1 {}
impl Parser<syntax::Exp1> for EmptyExpression1 {
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
            Box::new(EmptyExpression1{}),
        ]}.parse(input)
    }
}

pub struct Expression {}
impl Parser<syntax::Exp> for Expression {
    fn parse<'a>(&self, input : &'a str) -> Result<(syntax::Exp, &'a str), ParseError> {
        let (exp2, input) = Expression2{}.parse(input)?;
        let (exp1, input) = Expression1{}.parse(input)?;
        Ok((syntax::Exp::Exp(Box::new(exp2), Box::new(exp1)), input))
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