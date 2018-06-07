pub mod combinator;
use parser::combinator::*;
pub mod syntax;

// //---- Expression --------------------------------------------------------------------
// pub struct Num {}
// impl Parser<syntax::Term> for Num {
//     fn parse(&self, input : String) -> Result<(syntax::Term, String), ParseError> {
//         let (_, input) = Spaces{}.parse(input)?;
//         let (num, input) = Digit{}.parse(input)?;
//         Ok((syntax::Term::Num(num), input))
//     }
// }

// pub struct Var {}
// impl Parser<syntax::Term> for Var {
//     fn parse(&self, input : String) -> Result<(syntax::Term, String), ParseError> {
//         let (_, input) = Spaces{}.parse(input)?;
//         let (var, input) = Many1{p: &Lower{}}.parse(input)?;
//         let name = var.into_iter().collect::<String>();
//         Ok((syntax::Term::Var(name), input))
//     }
// }

// pub struct Fun {}
// impl Parser<syntax::Term> for Fun {
//     fn parse(&self, input : String) -> Result<(syntax::Term, String), ParseError> {
//         let (names, input) = Between {
//             left_p: &Char{c: '|'},
//             right_p: &Char{c: '|'},
//             mid_p: &SepBy{
//                 p: &Many1{p: &Lower{}},
//                 sep: &Char{c: ','}
//             }
//         }.parse(input)?;
//         let (exp, input) = Expression{}.parse(input)?;

//         let mut names2 : Vec<String> = vec![];
//         for name in names {
//             let name = name.into_iter().collect::<String>();
//             names2.push(name)
//         }
//         Ok((syntax::Term::Function(names2, Box::new(exp)), input))
//     }
// }

// pub struct ParenedExpression {}
// impl Parser<syntax::Term> for ParenedExpression {
//     fn parse(&self, input : String) -> Result<(syntax::Term, String), ParseError> {
//         let (exp, input) = Between {
//             left_p: &Then::new(&Spaces{}, &Char{c:'('}),
//             mid_p: &Then::new(&Spaces{}, &Expression{}),
//             right_p: &Then::new(&Spaces{}, &Char{c:')'}),
//         }.parse(input)?;
//         Ok((syntax::Term::Paren(Box::new(exp)), input))
//     }    
// }

// pub struct Term {}
// impl Parser<syntax::Term> for Term {
//     fn parse(&self, input : String) -> Result<(syntax::Term, String), ParseError> {
//         Try {
//             ps: vec![
//                 &Num{},
//                 &Var{},
//                 &Fun{},
//                 &ParenedExpression{},
//             ]
//         }.parse(input)
//     }
// }

// pub struct AppExpression {}
// impl Parser<syntax::Exp5> for AppExpression {
//     fn parse(&self, input : String) -> Result<(syntax::Exp5, String), ParseError> {
//         let (_, input) = Spaces{}.parse(input)?;
//         let (term, input) = Term{}.parse(input)?;
//         let (_, input) = Spaces{}.parse(input)?;
//         let (exp5, input) = Expression5{}.parse(input)?;
//         Ok((syntax::Exp5::App(Box::new(term), Box::new(exp5)), input))
//     }
// }

// pub struct EmptyExpression5 {}
// impl Parser<syntax::Exp5> for EmptyExpression5 {
//     fn parse(&self, input : String) -> Result<(syntax::Exp5, String), ParseError> {
//         Ok((syntax::Exp5::Empty, input))
//     }
// }

// struct Expression5 {}
// impl Parser<syntax::Exp5> for Expression5 {
//     fn parse(&self, input : String) -> Result<(syntax::Exp5, String), ParseError> {
//         Try{ps: vec![
//             &AppExpression{},
//             &EmptyExpression5{},
//         ]}.parse(input)
//     }
// }

// pub struct Expression4 {}
// impl Parser<syntax::Exp4> for Expression4 {
//     fn parse(&self, input : String) -> Result<(syntax::Exp4, String), ParseError> {
//         let (_, input) = Spaces{}.parse(input)?;
//         let (term, input) = Term{}.parse(input)?;
//         let (_, input) = Spaces{}.parse(input)?;
//         let (exp5, input) = Expression5{}.parse(input)?;
//         Ok((syntax::Exp4::Exp4(Box::new(term), Box::new(exp5)), input))
//     }
// }

// pub struct MulExpression {}
// impl Parser<syntax::Exp3> for MulExpression {
//     fn parse(&self, input : String) -> Result<(syntax::Exp3, String), ParseError> {
//         let (_, input) = Spaces{}.parse(input)?;
//         let (_, input) = Char{c: '*'}.parse(input)?;
//         let (_, input) = Spaces{}.parse(input)?;
//         let (exp4, input) = Expression4{}.parse(input)?;
//         let (_, input) = Spaces{}.parse(input)?;
//         let (exp3, input) = Expression3{}.parse(input)?;
//         Ok((syntax::Exp3::Mul(Box::new(exp4), Box::new(exp3)), input))
//     }
// }

// pub struct DivExpression {}
// impl Parser<syntax::Exp3> for DivExpression {
//     fn parse(&self, input : String) -> Result<(syntax::Exp3, String), ParseError> {
//         let (_, input) = Spaces{}.parse(input)?;
//         let (_, input) = Char{c: '/'}.parse(input)?;
//         let (_, input) = Spaces{}.parse(input)?;
//         let (exp4, input) = Expression4{}.parse(input)?;
//         let (_, input) = Spaces{}.parse(input)?;
//         let (exp3, input) = Expression3{}.parse(input)?;
//         Ok((syntax::Exp3::Div(Box::new(exp4), Box::new(exp3)), input))
//     }
// }

// pub struct EmptyExpression3 {}
// impl Parser<syntax::Exp3> for EmptyExpression3 {
//     fn parse(&self, input : String) -> Result<(syntax::Exp3, String), ParseError> {
//         Ok((syntax::Exp3::Empty, input))
//     }
// }

// struct Expression3 {}
// impl Parser<syntax::Exp3> for Expression3 {
//     fn parse(&self, input : String) -> Result<(syntax::Exp3, String), ParseError> {
//         Try{ps: vec![
//             &MulExpression{},
//             &DivExpression{},
//             &EmptyExpression3{},
//         ]}.parse(input)
//     }
// }

// pub struct Expression2 {}
// impl Parser<syntax::Exp2> for Expression2 {
//     fn parse(&self, input : String) -> Result<(syntax::Exp2, String), ParseError> {
//         let (_, input) = Spaces{}.parse(input)?;
//         let (exp4, input) = Expression4{}.parse(input)?;
//         let (_, input) = Spaces{}.parse(input)?;
//         let (exp3, input) = Expression3{}.parse(input)?;
//         Ok((syntax::Exp2::Exp2(Box::new(exp4), Box::new(exp3)), input))
//     }
// }

// pub struct AddExpression {}
// impl Parser<syntax::Exp1> for AddExpression {
//     fn parse(&self, input : String) -> Result<(syntax::Exp1, String), ParseError> {
//         let (_, input) = Spaces{}.parse(input)?;
//         let (_, input) = Char{c: '+'}.parse(input)?;
//         let (exp2, input) = Expression2{}.parse(input)?;
//         let (exp1, input) = Expression1{}.parse(input)?;
//         Ok((syntax::Exp1::Add(Box::new(exp2), Box::new(exp1)), input))
//     }
// }

// pub struct SubExpression {}
// impl Parser<syntax::Exp1> for SubExpression {
//     fn parse(&self, input : String) -> Result<(syntax::Exp1, String), ParseError> {
//         let (_, input) = Spaces{}.parse(input)?;
//         let (_, input) = Char{c: '-'}.parse(input)?;
//         let (exp2, input) = Expression2{}.parse(input)?;
//         let (exp1, input) = Expression1{}.parse(input)?;
//         Ok((syntax::Exp1::Sub(Box::new(exp2), Box::new(exp1)), input))
//     }
// }

// pub struct EmptyExpression1 {}
// impl Parser<syntax::Exp1> for EmptyExpression1 {
//     fn parse(&self, input : String) -> Result<(syntax::Exp1, String), ParseError> {
//         Ok((syntax::Exp1::Empty, input))
//     }
// }

// struct Expression1 {}
// impl Parser<syntax::Exp1> for Expression1 {
//     fn parse(&self, input : String) -> Result<(syntax::Exp1, String), ParseError> {
//         Try{ps: vec![
//             &AddExpression{},
//             &SubExpression{},
//             &EmptyExpression1{},
//         ]}.parse(input)
//     }
// }

// pub struct Expression {}
// impl Parser<syntax::Exp> for Expression {
//     fn parse(&self, input : String) -> Result<(syntax::Exp, String), ParseError> {
//         let (_, input) = Spaces{}.parse(input)?;
//         let (exp2, input) = Expression2{}.parse(input)?;
//         let (_, input) = Spaces{}.parse(input)?;
//         let (exp1, input) = Expression1{}.parse(input)?;
//         Ok((syntax::Exp::Exp(Box::new(exp2), Box::new(exp1)), input))
//     }
// }

// //---- Statement --------------------------------------------------------------------
// pub struct ExpressionStatement {}
// impl Parser<syntax::Statement> for ExpressionStatement {
//     fn parse(&self, input : String) -> Result<(syntax::Statement, String), ParseError> {
//         let (_, input) = Spaces{}.parse(input)?;
//         let (exp, input) = Expression{}.parse(input)?;
//         Ok((syntax::Statement::ExpressionStatement(Box::new(exp)), input))
//     }   
// }

// pub struct AssignmentStatement {}
// impl Parser<syntax::Statement> for AssignmentStatement {
//     fn parse(&self, input : String) -> Result<(syntax::Statement, String), ParseError> {
//         let (_, input) = Spaces{}.parse(input)?;
//         let (var, input) = Many1{p: &Lower{}}.parse(input)?;
//         let (_, input) = Spaces{}.parse(input)?;
//         let (_, input) = Char{c: '='}.parse(input)?;
//         let (_, input) = Spaces{}.parse(input)?;
//         let (exp, input) = Expression{}.parse(input)?;
//         let name = var.iter().collect::<String>();
//         Ok((syntax::Statement::AssignmentStatement(name, Box::new(exp)), input))
//     }    
// }

// pub struct Statement {}
// impl Parser<syntax::Statement> for Statement {
//     fn parse(&self, input : String) -> Result<(syntax::Statement, String), ParseError> {
//         let (statement, input) = Try {ps: vec![
//             &AssignmentStatement{},
//             &ExpressionStatement{},
//         ]}.parse(input)?;
//         let (_, input) = Spaces{}.parse(input)?;
//         let (_, input) = Eof{}.parse(input)?;
//         Ok((statement, input))
//     }
// }
