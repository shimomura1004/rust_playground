pub mod combinator;
use parser::combinator::*;
pub mod syntax;

//---- Expression --------------------------------------------------------------------
pub struct Num {}
impl Num {
    pub fn new() -> Box<Parser<syntax::Term>> {
        Box::new(Num{})
    }
}
impl Parser<syntax::Term> for Num {
    fn parse(&self, input : &mut String) -> Result<syntax::Term, ParseError> {
        Spaces::new().parse(input)?;
        let num = Digit::new().parse(input)?;
        Ok(syntax::Term::Num(num))
    }
}

pub struct Var {}
impl Var {
    pub fn new() -> Box<Parser<syntax::Term>> {
        Box::new(Var{})
    }
}
impl Parser<syntax::Term> for Var {
    fn parse(&self, input : &mut String) -> Result<syntax::Term, ParseError> {
        Spaces::new().parse(input)?;
        let var = Many1::new(Lower::new()).parse(input)?;
        let name = var.into_iter().collect::<String>();
        Ok(syntax::Term::Var(name))
    }
}

pub struct Fun {}
impl Fun {
    pub fn new() -> Box<Parser<syntax::Term>> {
        Box::new(Fun{})
    }
}
impl Parser<syntax::Term> for Fun {
    fn parse(&self, input : &mut String) -> Result<syntax::Term, ParseError> {
        let names = Between::new(
            Char::new('|'),
            SepBy::new(Many1::new(Lower::new()), Char::new(',')),
            Char::new('|')
        ).parse(input)?;
        let exp = Expression::new().parse(input)?;

        let mut names2 : Vec<String> = vec![];
        for name in names {
            let name = name.into_iter().collect::<String>();
            names2.push(name)
        }
        Ok(syntax::Term::Function(names2, Box::new(exp)))
    }
}

pub struct ParenedExpression {}
impl ParenedExpression {
    pub fn new() -> Box<Parser<syntax::Term>> {
        Box::new(ParenedExpression{})
    }
}
impl Parser<syntax::Term> for ParenedExpression {
    fn parse(&self, input : &mut String) -> Result<syntax::Term, ParseError> {
        let exp = Between::new(
            Then::new(Spaces::new(), Char::new('(')),
            Then::new(Spaces::new(), Expression::new()),
            Then::new(Spaces::new(), Char::new(')')),
        ).parse(input)?;
        Ok(syntax::Term::Paren(Box::new(exp)))
    }    
}

pub struct Term {}
impl Term {
    pub fn new() -> Box<Parser<syntax::Term>> {
        Box::new(Term{})
    }
}
impl Parser<syntax::Term> for Term {
    fn parse(&self, input : &mut String) -> Result<syntax::Term, ParseError> {
        Try::new(vec![
            Num::new(),
            Var::new(),
            Fun::new(),
            ParenedExpression::new(),
        ]).parse(input)
    }
}

pub struct AppExpression {}
impl AppExpression {
    pub fn new() -> Box<Parser<syntax::Exp5>> {
        Box::new(AppExpression{})
    }
}
impl Parser<syntax::Exp5> for AppExpression {
    fn parse(&self, input : &mut String) -> Result<syntax::Exp5, ParseError> {
        Spaces::new().parse(input)?;
        let term = Term::new().parse(input)?;
        Spaces::new().parse(input)?;
        let exp5 = Expression5::new().parse(input)?;
        Ok(syntax::Exp5::App(Box::new(term), Box::new(exp5)))
    }
}

pub struct EmptyExpression5 {}
impl EmptyExpression5 {
    pub fn new() -> Box<Parser<syntax::Exp5>> {
        Box::new(EmptyExpression5{})
    }
}
impl Parser<syntax::Exp5> for EmptyExpression5 {
    fn parse(&self, input : &mut String) -> Result<syntax::Exp5, ParseError> {
        Ok(syntax::Exp5::Empty)
    }
}

struct Expression5 {}
impl Expression5 {
    pub fn new() -> Box<Parser<syntax::Exp5>> {
        Box::new(Expression5{})
    }
}
impl Parser<syntax::Exp5> for Expression5 {
    fn parse(&self, input : &mut String) -> Result<syntax::Exp5, ParseError> {
        Try::new(vec![
            AppExpression::new(),
            EmptyExpression5::new(),
        ]).parse(input)
    }
}

pub struct Expression4 {}
impl Expression4 {
    pub fn new() -> Box<Parser<syntax::Exp4>> {
        Box::new(Expression4{})
    }
}
impl Parser<syntax::Exp4> for Expression4 {
    fn parse(&self, input : &mut String) -> Result<syntax::Exp4, ParseError> {
        Spaces::new().parse(input)?;
        let term = Term::new().parse(input)?;
        Spaces::new().parse(input)?;
        let exp5 = Expression5::new().parse(input)?;
        Ok(syntax::Exp4::Exp4(Box::new(term), Box::new(exp5)))
    }
}

pub struct MulExpression {}
impl MulExpression {
    pub fn new() -> Box<Parser<syntax::Exp3>> {
        Box::new(MulExpression{})
    }
}
impl Parser<syntax::Exp3> for MulExpression {
    fn parse(&self, input : &mut String) -> Result<syntax::Exp3, ParseError> {
        Spaces::new().parse(input)?;
        Char::new('*').parse(input)?;
        Spaces::new().parse(input)?;
        let exp4 = Expression4::new().parse(input)?;
        Spaces::new().parse(input)?;
        let exp3 = Expression3::new().parse(input)?;
        Ok(syntax::Exp3::Mul(Box::new(exp4), Box::new(exp3)))
    }
}

pub struct DivExpression {}
impl DivExpression {
    pub fn new() -> Box<Parser<syntax::Exp3>> {
        Box::new(DivExpression{})
    }
}
impl Parser<syntax::Exp3> for DivExpression {
    fn parse(&self, input : &mut String) -> Result<syntax::Exp3, ParseError> {
        Spaces::new().parse(input)?;
        Char::new('/').parse(input)?;
        Spaces::new().parse(input)?;
        let exp4 = Expression4::new().parse(input)?;
        Spaces::new().parse(input)?;
        let exp3 = Expression3::new().parse(input)?;
        Ok(syntax::Exp3::Div(Box::new(exp4), Box::new(exp3)))
    }
}

pub struct EmptyExpression3 {}
impl EmptyExpression3 {
    pub fn new() -> Box<Parser<syntax::Exp3>> {
        Box::new(EmptyExpression3{})
    }
}
impl Parser<syntax::Exp3> for EmptyExpression3 {
    fn parse(&self, input : &mut String) -> Result<syntax::Exp3, ParseError> {
        Ok(syntax::Exp3::Empty)
    }
}

struct Expression3 {}
impl Expression3 {
    pub fn new() -> Box<Parser<syntax::Exp3>> {
        Box::new(Expression3{})
    }
}
impl Parser<syntax::Exp3> for Expression3 {
    fn parse(&self, input : &mut String) -> Result<syntax::Exp3, ParseError> {
        Try::new(vec![
            MulExpression::new(),
            DivExpression::new(),
            EmptyExpression3::new(),
        ]).parse(input)
    }
}

pub struct Expression2 {}
impl Expression2 {
    pub fn new() -> Box<Parser<syntax::Exp2>> {
        Box::new(Expression2{})
    }
}
impl Parser<syntax::Exp2> for Expression2 {
    fn parse(&self, input : &mut String) -> Result<syntax::Exp2, ParseError> {
        Spaces::new().parse(input)?;
        let exp4 = Expression4::new().parse(input)?;
        Spaces::new().parse(input)?;
        let exp3 = Expression3::new().parse(input)?;
        Ok(syntax::Exp2::Exp2(Box::new(exp4), Box::new(exp3)))
    }
}

pub struct AddExpression {}
impl AddExpression {
    pub fn new() -> Box<Parser<syntax::Exp1>> {
        Box::new(AddExpression{})
    }
}
impl Parser<syntax::Exp1> for AddExpression {
    fn parse(&self, input : &mut String) -> Result<syntax::Exp1, ParseError> {
        Spaces::new().parse(input)?;
        Char::new('+').parse(input)?;
        let exp2 = Expression2::new().parse(input)?;
        let exp1 = Expression1::new().parse(input)?;
        Ok(syntax::Exp1::Add(Box::new(exp2), Box::new(exp1)))
    }
}

pub struct SubExpression {}
impl SubExpression {
    pub fn new() -> Box<Parser<syntax::Exp1>> {
        Box::new(SubExpression{})
    }
}
impl Parser<syntax::Exp1> for SubExpression {
    fn parse(&self, input : &mut String) -> Result<syntax::Exp1, ParseError> {
        Spaces::new().parse(input)?;
        Char::new('-').parse(input)?;
        let exp2 = Expression2::new().parse(input)?;
        let exp1 = Expression1::new().parse(input)?;
        Ok(syntax::Exp1::Sub(Box::new(exp2), Box::new(exp1)))
    }
}

pub struct EmptyExpression1 {}
impl EmptyExpression1 {
    pub fn new() -> Box<Parser<syntax::Exp1>> {
        Box::new(EmptyExpression1{})
    }
}
impl Parser<syntax::Exp1> for EmptyExpression1 {
    fn parse(&self, input : &mut String) -> Result<syntax::Exp1, ParseError> {
        Ok(syntax::Exp1::Empty)
    }
}

struct Expression1 {}
impl Expression1 {
    pub fn new() -> Box<Parser<syntax::Exp1>> {
        Box::new(Expression1{})
    }
}
impl Parser<syntax::Exp1> for Expression1 {
    fn parse(&self, input : &mut String) -> Result<syntax::Exp1, ParseError> {
        Try::new(vec![
            AddExpression::new(),
            SubExpression::new(),
            EmptyExpression1::new(),
        ]).parse(input)
    }
}

pub struct Expression {}
impl Expression {
    pub fn new() -> Box<Parser<syntax::Exp>> {
        Box::new(Expression{})
    }
}
impl Parser<syntax::Exp> for Expression {
    fn parse(&self, input : &mut String) -> Result<syntax::Exp, ParseError> {
        Spaces::new().parse(input)?;
        let exp2 = Expression2::new().parse(input)?;
        Spaces::new().parse(input)?;
        let exp1 = Expression1::new().parse(input)?;
        Ok(syntax::Exp::Exp(Box::new(exp2), Box::new(exp1)))
    }
}

//---- Statement --------------------------------------------------------------------
pub struct ExpressionStatement {}
impl ExpressionStatement {
    pub fn new() -> Box<Parser<syntax::Statement>> {
        Box::new(ExpressionStatement{})
    }
}
impl Parser<syntax::Statement> for ExpressionStatement {
    fn parse(&self, input : &mut String) -> Result<syntax::Statement, ParseError> {
        Spaces::new().parse(input)?;
        let exp = Expression::new().parse(input)?;
        Ok(syntax::Statement::ExpressionStatement(Box::new(exp)))
    }   
}

pub struct AssignmentStatement {}
impl AssignmentStatement {
    pub fn new() -> Box<Parser<syntax::Statement>> {
        Box::new(AssignmentStatement{})
    }
}
impl Parser<syntax::Statement> for AssignmentStatement {
    fn parse(&self, input : &mut String) -> Result<syntax::Statement, ParseError> {
        Spaces::new().parse(input)?;
        let var = Many1::new(Lower::new()).parse(input)?;
        Spaces::new().parse(input)?;
        Char::new('=').parse(input)?;
        Spaces::new().parse(input)?;
        let exp = Expression::new().parse(input)?;

        let name = var.iter().collect::<String>();
        Ok(syntax::Statement::AssignmentStatement(name, Box::new(exp)))
    }
}

pub struct Statement {}
impl Statement {
    pub fn new() -> Box<Parser<syntax::Statement>> {
        Box::new(Statement{})
    }
}
impl Parser<syntax::Statement> for Statement {
    fn parse(&self, input : &mut String) -> Result<syntax::Statement, ParseError> {
        let statement = Try::new(vec![
            AssignmentStatement::new(),
            ExpressionStatement::new(),
        ]).parse(input)?;
        Spaces::new().parse(input)?;
        Eof::new().parse(input)?;
        Ok(statement)
    }
}
