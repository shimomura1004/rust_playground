use std::fmt;
use std::string;

// todo: implement Parser with function? Fn<T> (&str) -> Result((T, &str), ParseError)
pub trait Parser<T> {
    fn parse(&self, input : &mut String) -> Result<T, ParseError>;
}
type ParserB<T> = Box<Parser<T>>;

pub struct ParseError {
    pub filename: string::String,
    pub line: u32,
    pub char: u32,
    pub explanation: string::String,
}

impl fmt::Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}:{} {}", self.filename, self.line, self.char, self.explanation)
    }
}

pub struct Char {
    pub c : char,
}
impl Char {
    pub fn new(c : char) -> Box<Parser<char>> {
        Box::new(Char{c})
    }
}
impl Parser<char> for Char {
    fn parse(&self, input : &mut String) -> Result<char, ParseError> {
        match input.chars().next() {
            Some(c) => {
                if c == self.c {
                    input.remove(0);
                    Ok(c)
                }
                else {
                    Err(ParseError {
                        filename: "stdin".to_string(),
                        line: 0,
                        char: 0,
                        explanation: format!("expected '{}' but got '{}'", self.c, c),
                    })
                }
            }
            None => Err(ParseError {
                filename: "stdin".to_string(),
                line: 0,
                char: 0,
                explanation: format!("expected '{}' but got EOF", self.c),
            })
        }
    }
}

pub struct Many<T> {
    pub p : ParserB<T>,
}
impl<T: 'static> Many<T> {
    pub fn new(p : Box<Parser<T>>) -> Box<Parser<Vec<T>>> {
        Box::new(Many{p})
    }
}
impl<T> Parser<Vec<T>> for Many<T> {
    fn parse(&self, input : &mut String) -> Result<Vec<T>, ParseError> {
        let mut result = Vec::new();
        
        loop {
            match self.p.parse(input) {
                Ok(r) => {
                    result.push(r);
                },
                Err(_) => break,
            };
        }

        Ok(result)
    }
}

pub struct SkipMany<T> {
    pub p: ParserB<T>,
}
impl<T: 'static> SkipMany<T> {
    pub fn new(p : Box<Parser<T>>) -> Box<Parser<()>> {
        Box::new(SkipMany{p: Many::new(p)})
    }
}
impl<T> Parser<()> for SkipMany<T> {
    fn parse(&self, input : &mut String) -> Result<(), ParseError> {
        self.p.parse(input)?;
        Ok(())
    }
}

pub struct Many1<T> {
    pub p : ParserB<T>,
}
impl<T: 'static> Many1<T> {
    pub fn new(p : Box<Parser<T>>) -> Box<Parser<Vec<T>>> {
        Box::new(Many1{p})
    }
}
impl<T> Parser<Vec<T>> for Many1<T> {
    fn parse(&self, input : &mut String) -> Result<Vec<T>, ParseError> {
        let mut rs = vec![];
        rs.push(self.p.parse(input)?);

        loop {
            match self.p.parse(input) {
                Ok(r) => rs.push(r),
                Err(_) => break,
            };
        }

        Ok(rs)
    }
}

pub struct Try<T> {
    pub ps : Vec<ParserB<T>>,
}
impl<T: 'static> Try<T> {
    pub fn new(ps : Vec<Box<Parser<T>>>) -> Box<Parser<T>> {
        Box::new(Try{ps})
    }
}
impl<T> Parser<T> for Try<T> {
    fn parse(&self, input : &mut String) -> Result<T, ParseError> {
        let mut input_clone = input.clone();
        let r = self.ps[0].parse(&mut input_clone);
        if !r.is_ok() {
            for p in &self.ps[1..] {
                let mut input_clone = input.clone();
                let r = p.parse(&mut input_clone);
                if r.is_ok() {
                    *input = input_clone;
                    return r;
                }
            }
        }
        *input = input_clone;
        r
    }
}

pub struct Then<T1, T2> {
    pub p1 : Box<Parser<T1>>,
    pub p2 : Box<Parser<T2>>,
}
impl<T1: 'static, T2: 'static> Then<T1, T2> {
    pub fn new(p1: Box<Parser<T1>>, p2: Box<Parser<T2>>) -> Box<Parser<T2>> {
        Box::new(Then{p1, p2})
    }
}
impl<T1, T2> Parser<T2> for Then<T1, T2> {
    fn parse(&self, input : &mut String) -> Result<T2, ParseError> {
        self.p1.parse(input)?;
        self.p2.parse(input)
    }
}

pub struct OneOf {
    pub p: Box<Parser<char>>,
}
impl OneOf {
    pub fn new(chars: &str) -> Box<Parser<char>> {
        let mut ps = vec![];
        for c in chars.chars().collect::<Vec<char>>() {
            ps.push(Char::new(c));
        }
        Box::new(OneOf{p: Try::new(ps)})
    }
}
impl Parser<char> for OneOf {
    fn parse(&self, input : &mut String) -> Result<char, ParseError> {
        self.p.parse(input)
    }
}

pub struct Digit {
    pub p: Box<Parser<Vec<char>>>,
}
impl Digit {
    pub fn new() -> Box<Parser<i32>> {
        Box::new(Digit{p: Many1::new(OneOf::new("0123456789"))})
    }
}
impl Parser<i32> for Digit {
    fn parse(&self, input : &mut String) -> Result<i32, ParseError> {
        let digit_str = self.p.parse(input)?;
        let digit_str : String = digit_str.into_iter().collect();
        Ok(digit_str.parse::<i32>().unwrap())
    }
}

pub struct Lower {
    pub p: Box<Parser<char>>,
}
impl Lower {
    pub fn new() -> Box<Parser<char>> {
        Box::new(Lower{p: OneOf::new("abcdefghijklmnopqrstuvwxyz")})
    }
}
impl Parser<char> for Lower {
    fn parse(&self, input : &mut String) -> Result<char, ParseError> {
        self.p.parse(input)
    }
}

// pub struct Upper {}
// impl Parser<char> for Upper {
//     fn parse(&self, input : String) -> Result<(char, String), ParseError> {
//         OneOf::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ").parse(input)
//     }
// }

// pub struct Letter {}
// impl Parser<char> for Letter {
//     fn parse(&self, input : String) -> Result<(char, String), ParseError> {
//         Try{ps: vec![&Upper{}, &Lower{}]}.parse(input)
//     }
// }

pub struct Space {
    pub p: Box<Parser<char>>,
}
impl Space {
    pub fn new() -> Box<Parser<char>> {
        Box::new(Space{p: OneOf::new(" \t\n\r")})
    }
}
impl Parser<char> for Space {
    fn parse(&self, input : &mut String) -> Result<char, ParseError> {
        self.p.parse(input)
    }
}

pub struct Spaces {
    pub p: Box<Parser<()>>,
}
impl Spaces {
    pub fn new() -> Box<Parser<()>> {
        Box::new(Spaces{p: SkipMany::new(Space::new())})
    }
}
impl Parser<()> for Spaces {
    fn parse(&self, input : &mut String) -> Result<(), ParseError> {
        self.p.parse(input)
    }
}

pub struct Between<T1, T2, T3> {
    pub left_p: Box<Parser<T1>>,
    pub mid_p: Box<Parser<T2>>,
    pub right_p: Box<Parser<T3>>,
}
impl<T1: 'static, T2: 'static, T3: 'static> Between<T1, T2, T3> {
    pub fn new(left_p : Box<Parser<T1>>, mid_p: Box<Parser<T2>>, right_p: Box<Parser<T3>>) -> Box<Parser<T2>> {
        Box::new(Between{left_p, mid_p, right_p})
    }
}
impl<T1, T2, T3> Parser<T2> for Between<T1, T2, T3> {
    fn parse(&self, input : &mut String) -> Result<T2, ParseError> {
        self.left_p.parse(input)?;
        let r = self.mid_p.parse(input)?;
        self.right_p.parse(input)?;
        Ok(r)
    }
}

pub struct Eof {}
impl Eof {
    pub fn new() -> Box<Parser<()>> {
        Box::new(Eof{})
    }
}
impl Parser<()> for Eof {
    fn parse(&self, input : &mut String) -> Result<(), ParseError> {
        if input.len() == 0 {
            Ok(())
        }
        else {
            Err(ParseError {
                filename: "stdin".to_string(),
                line: 0,
                char: 0,
                explanation: format!("expected EOF but got {}", input.chars().next().unwrap()),
            })
        }
    }
}

pub struct SepBy<T1, T2> {
    pub p: Box<Parser<T1>>,
    pub sep: Box<Parser<T2>>,
}
impl<T1: 'static, T2: 'static> SepBy<T1, T2> {
    pub fn new(p : Box<Parser<T1>>, sep : Box<Parser<T2>>) -> Box<Parser<Vec<T1>>> {
        Box::new(SepBy{p, sep})
    }
}
impl<T1, T2> Parser<Vec<T1>> for SepBy<T1, T2> {
    fn parse(&self, input : &mut String) -> Result<Vec<T1>, ParseError> {
        let mut results = vec![];

        loop {
            let result = self.p.parse(input)?;
            results.push(result);

            let sep_result = self.sep.parse(input);
            if sep_result.is_err() {
                break;
            }
        }

        Ok(results)
    }
}
