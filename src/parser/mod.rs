pub mod combinator;
use parser::combinator::*;
pub mod syntax;

//---- Expression --------------------------------------------------------------------
pub struct Num {}
impl<'a> Parser<'a, syntax::Term<'a>> for Num {
    fn parse(&self, input : &'a str) -> Result<(syntax::Term<'a>, &'a str), ParseError> {
        let (_, input) = Spaces::new().parse(input)?;
        let (num, input) = Digit{}.parse(input)?;
        Ok((syntax::Term::Num(num), input))
    }
}

pub struct Var {}
impl<'a> Parser<'a, syntax::Term<'a>> for Var {
    fn parse(&self, input : &'a str) -> Result<(syntax::Term<'a>, &'a str), ParseError> {
        let (_, input) = Spaces::new().parse(input)?;
        let (var, input) = Many1{p: &Lower{}}.parse(input)?;
        let name = var.into_iter().collect::<String>();
        Ok((syntax::Term::Var(&*name), input))
    }
}

pub struct Fun {}
impl<'a> Parser<'a, syntax::Term<'a>> for Fun {
    fn parse(&self, input : &'a str) -> Result<(syntax::Term<'a>, &'a str), ParseError> {
        let (names, input) = Between {
            left_p: &Char{c: '|'},
            right_p: &Char{c: '|'},
            mid_p: &SepBy{
                p: &Many1{p: &Lower{}},
                sep: &Char{c: ','}
            }
        }.parse(input)?;
        let (exp, input) = Expression{}.parse(input)?;

        let names2 : Vec<&str> = vec![];
        for name in names {
            let name = name.into_iter().collect::<String>();
            names2.push(&name)
        }
        Ok((syntax::Term::Function(names2, Box::new(exp)), input))
    }
}

pub struct ParenedExpression {}
impl<'a> Parser<'a, syntax::Term<'a>> for ParenedExpression {
    fn parse(&self, input : &'a str) -> Result<(syntax::Term<'a>, &'a str), ParseError> {
        let (exp, input) = Between {
            left_p: &Then::new(&Spaces::new(), &Char{c:'('}),
            mid_p: &Then::new(&Spaces::new(), &Expression{}),
            right_p: &Then::new(&Spaces::new(), &Char{c:')'}),
        }.parse(input)?;
        Ok((syntax::Term::Paren(Box::new(exp)), input))
    }    
}

pub struct Term {}
impl<'a> Parser<'a, syntax::Term<'a>> for Term {
    fn parse(&self, input : &'a str) -> Result<(syntax::Term<'a>, &'a str), ParseError> {
        Try {
            ps: vec![
                Box::new(Num{}),
                Box::new(Var{}),
                Box::new(Fun{}),
                Box::new(ParenedExpression{}),
            ]
        }.parse(input)
    }
}

pub struct AppExpression {}
impl<'a> Parser<'a, syntax::Exp5<'a>> for AppExpression {
    fn parse(&self, input : &'a str) -> Result<(syntax::Exp5<'a>, &'a str), ParseError> {
        let (_, input) = Spaces::new().parse(input)?;
        let (term, input) = Term{}.parse(input)?;
        let (_, input) = Spaces::new().parse(input)?;
        let (exp5, input) = Expression5{}.parse(input)?;
        Ok((syntax::Exp5::App(Box::new(term), Box::new(exp5)), input))
    }
}

pub struct EmptyExpression5 {}
impl<'a> Parser<'a, syntax::Exp5<'a>> for EmptyExpression5 {
    fn parse(&self, input : &'a str) -> Result<(syntax::Exp5<'a>, &'a str), ParseError> {
        Ok((syntax::Exp5::Empty, input))
    }
}

struct Expression5 {}
impl<'a> Parser<'a, syntax::Exp5<'a>> for Expression5 {
    fn parse(&self, input : &'a str) -> Result<(syntax::Exp5<'a>, &'a str), ParseError> {
        Try{ps: vec![
            Box::new(AppExpression{}),
            Box::new(EmptyExpression5{}),
        ]}.parse(input)
    }
}

pub struct Expression4 {}
impl<'a> Parser<'a, syntax::Exp4<'a>> for Expression4 {
    fn parse(&self, input : &'a str) -> Result<(syntax::Exp4<'a>, &'a str), ParseError> {
        let (_, input) = Spaces::new().parse(input)?;
        let (term, input) = Term{}.parse(input)?;
        let (_, input) = Spaces::new().parse(input)?;
        let (exp5, input) = Expression5{}.parse(input)?;
        Ok((syntax::Exp4::Exp4(Box::new(term), Box::new(exp5)), input))
    }
}

pub struct MulExpression {}
impl<'a> Parser<'a, syntax::Exp3<'a>> for MulExpression {
    fn parse(&self, input : &'a str) -> Result<(syntax::Exp3<'a>, &'a str), ParseError> {
        let (_, input) = Spaces::new().parse(input)?;
        let (_, input) = Char{c: '*'}.parse(input)?;
        let (_, input) = Spaces::new().parse(input)?;
        let (exp4, input) = Expression4{}.parse(input)?;
        let (_, input) = Spaces::new().parse(input)?;
        let (exp3, input) = Expression3{}.parse(input)?;
        Ok((syntax::Exp3::Mul(Box::new(exp4), Box::new(exp3)), input))
    }
}

pub struct DivExpression {}
impl<'a> Parser<'a, syntax::Exp3<'a>> for DivExpression {
    fn parse(&self, input : &'a str) -> Result<(syntax::Exp3<'a>, &'a str), ParseError> {
        let (_, input) = Spaces::new().parse(input)?;
        let (_, input) = Char{c: '/'}.parse(input)?;
        let (_, input) = Spaces::new().parse(input)?;
        let (exp4, input) = Expression4{}.parse(input)?;
        let (_, input) = Spaces::new().parse(input)?;
        let (exp3, input) = Expression3{}.parse(input)?;
        Ok((syntax::Exp3::Div(Box::new(exp4), Box::new(exp3)), input))
    }
}

pub struct EmptyExpression3 {}
impl<'a> Parser<'a, syntax::Exp3<'a>> for EmptyExpression3 {
    fn parse(&self, input : &'a str) -> Result<(syntax::Exp3<'a>, &'a str), ParseError> {
        Ok((syntax::Exp3::Empty, input))
    }
}

struct Expression3 {}
impl<'a> Parser<'a, syntax::Exp3<'a>> for Expression3 {
    fn parse(&self, input : &'a str) -> Result<(syntax::Exp3<'a>, &'a str), ParseError> {
        Try{ps: vec![
            Box::new(MulExpression{}),
            Box::new(DivExpression{}),
            Box::new(EmptyExpression3{}),
        ]}.parse(input)
    }
}

pub struct Expression2 {}
impl<'a> Parser<'a, syntax::Exp2<'a>> for Expression2 {
    fn parse(&self, input : &'a str) -> Result<(syntax::Exp2<'a>, &'a str), ParseError> {
        let (_, input) = Spaces::new().parse(input)?;
        let (exp4, input) = Expression4{}.parse(input)?;
        let (_, input) = Spaces::new().parse(input)?;
        let (exp3, input) = Expression3{}.parse(input)?;
        Ok((syntax::Exp2::Exp2(Box::new(exp4), Box::new(exp3)), input))
    }
}

pub struct AddExpression {}
impl<'a> Parser<'a, syntax::Exp1<'a>> for AddExpression {
    fn parse(&self, input : &'a str) -> Result<(syntax::Exp1<'a>, &'a str), ParseError> {
        let (_, input) = Spaces::new().parse(input)?;
        let (_, input) = Char{c: '+'}.parse(input)?;
        let (exp2, input) = Expression2{}.parse(input)?;
        let (exp1, input) = Expression1{}.parse(input)?;
        Ok((syntax::Exp1::Add(Box::new(exp2), Box::new(exp1)), input))
    }
}

pub struct SubExpression {}
impl<'a> Parser<'a, syntax::Exp1<'a>> for SubExpression {
    fn parse(&self, input : &'a str) -> Result<(syntax::Exp1<'a>, &'a str), ParseError> {
        let (_, input) = Spaces::new().parse(input)?;
        let (_, input) = Char{c: '-'}.parse(input)?;
        let (exp2, input) = Expression2{}.parse(input)?;
        let (exp1, input) = Expression1{}.parse(input)?;
        Ok((syntax::Exp1::Sub(Box::new(exp2), Box::new(exp1)), input))
    }
}

pub struct EmptyExpression1 {}
impl<'a> Parser<'a, syntax::Exp1<'a>> for EmptyExpression1 {
    fn parse(&self, input : &'a str) -> Result<(syntax::Exp1<'a>, &'a str), ParseError> {
        Ok((syntax::Exp1::Empty, input))
    }
}

struct Expression1 {}
impl<'a> Parser<'a, syntax::Exp1<'a>> for Expression1 {
    fn parse(&self, input : &'a str) -> Result<(syntax::Exp1<'a>, &'a str), ParseError> {
        Try{ps: vec![
            Box::new(AddExpression{}),
            Box::new(SubExpression{}),
            Box::new(EmptyExpression1{}),
        ]}.parse(input)
    }
}

pub struct Expression {}
impl<'a> Parser<'a, syntax::Exp<'a>> for Expression {
    fn parse(&self, input : &'a str) -> Result<(syntax::Exp<'a>, &'a str), ParseError> {
        let (_, input) = Spaces::new().parse(input)?;
        let (exp2, input) = Expression2{}.parse(input)?;
        let (_, input) = Spaces::new().parse(input)?;
        let (exp1, input) = Expression1{}.parse(input)?;
        Ok((syntax::Exp::Exp(Box::new(exp2), Box::new(exp1)), input))
    }
}

//---- Statement --------------------------------------------------------------------
pub struct ExpressionStatement {}
impl<'a> Parser<'a, syntax::Statement<'a>> for ExpressionStatement {
    fn parse(&self, input : &'a str) -> Result<(syntax::Statement<'a>, &'a str), ParseError> {
        let (_, input) = Spaces::new().parse(input)?;
        let (exp, input) = Expression{}.parse(input)?;
        Ok((syntax::Statement::ExpressionStatement(Box::new(exp)), input))
    }   
}

pub struct AssignmentStatement {}
impl<'a> Parser<'a, syntax::Statement<'a>> for AssignmentStatement {
    fn parse(&self, input : &'a str) -> Result<(syntax::Statement<'a>, &'a str), ParseError> {
        let (_, input) = Spaces::new().parse(input)?;
        let (var, input) = Many1{p: &Lower{}}.parse(input)?;
        let (_, input) = Spaces::new().parse(input)?;
        let (_, input) = Char{c: '='}.parse(input)?;
        let (_, input) = Spaces::new().parse(input)?;
        let (exp, input) = Expression{}.parse(input)?;
        let name = var.iter().collect::<String>();
        Ok((syntax::Statement::AssignmentStatement(&name, Box::new(exp)), input))
    }    
}

pub struct Statement {}
impl<'a> Parser<'a, syntax::Statement<'a>> for Statement {
    fn parse(&self, input : &'a str) -> Result<(syntax::Statement<'a>, &'a str), ParseError> {
        let (statement, input) = Try {ps: vec![
            Box::new(AssignmentStatement{}),
            Box::new(ExpressionStatement{}),
        ]}.parse(input)?;
        let (_, input) = Spaces::new().parse(input)?;
        let (_, input) = Eof{}.parse(input)?;
        Ok((statement, input))
    }
}
