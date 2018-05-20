use std::fmt;
use std::string;

pub trait Parser<T> {
    fn parse<'a>(&self, input : &'a str) -> Result<(T, &'a str), ParseError>;
}

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

impl Parser<char> for Char {
    fn parse<'a>(&self, input : &'a str) -> Result<(char, &'a str), ParseError> {
        match input.chars().next() {
            Some(c) => {
                if c == self.c {
                    Ok((c, &input[1..]))
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

pub struct Many<'a, T: 'a> {
    pub p : &'a Parser<T>,
}

impl<'a, T: 'a> Parser<Vec<T>> for Many<'a, T> {
    fn parse<'b>(&self, input : &'b str) -> Result<(Vec<T>, &'b str), ParseError> {
        let mut result = Vec::new();
        let mut index = 0;
        
        loop {
            match self.p.parse(&input[index..]) {
                Ok((r, rest)) => {
                    result.push(r);
                    index = input.len() - rest.len();
                },
                Err(_) => break,
            };
        }

        Ok((result, &input[index..]))
    }
}

pub struct Many1<'a, T: 'a> {
    pub p : &'a Parser<T>,
}

impl<'a, T: 'a> Parser<Vec<T>> for Many1<'a, T> {
    fn parse<'b>(&self, input : &'b str) -> Result<(Vec<T>, &'b str), ParseError> {
        let (r, input) = self.p.parse(input)?;
        let many = Many {p: self.p};
        let (mut rs, input) = many.parse(input)?;
        rs.push(r);
        rs.rotate_right(1);
        Ok((rs, input))
    }
}

pub struct Try<T> {
    pub ps : Vec<Box<Parser<T>>>,
}

impl<T> Parser<T> for Try<T> {
    fn parse<'b>(&self, input : &'b str) -> Result<(T, &'b str), ParseError> {
        let mut r = self.ps[0].parse(input);
        if !r.is_ok() {
            for p in &self.ps {
                r = p.parse(input);
                if r.is_ok() {
                    break;
                }
            }
        }
        r
    }
}

pub struct OneOf {
    pub cs: Vec<char>,
}

impl OneOf {
    pub fn new(chars: &str) -> OneOf {
        OneOf {cs: chars.chars().collect::<Vec<char>>()}
    }
}

impl Parser<char> for OneOf {
    fn parse<'b>(&self, input : &'b str) -> Result<(char, &'b str), ParseError> {
        let mut ps : Vec<Box<Parser<char>>> = vec![];
        for c in &self.cs {
            ps.push(Box::new(Char{c: *c}));
        }
        Try{ps}.parse(input)
    }
}

pub struct Digit {}

impl Parser<i32> for Digit {
    fn parse<'b>(&self, input : &'b str) -> Result<(i32, &'b str), ParseError> {
        let (digit_str, input) = Many1{p: &OneOf::new("0123456789")}.parse(input)?;
        let digit_str : String = digit_str.into_iter().collect();
        let i = digit_str.parse::<i32>().unwrap();
        Ok((i, input))
    }
}

pub struct Between<'a, T1: 'a, T2: 'a, T3: 'a> {
    pub left_p: &'a Parser<T1>,
    pub mid_p: &'a Parser<T2>,
    pub right_p: &'a Parser<T3>,
}

impl<'a, T1, T2, T3> Parser<T2> for Between<'a, T1, T2, T3> {
    fn parse<'b>(&self, input : &'b str) -> Result<(T2, &'b str), ParseError> {
        let (_, input) = self.left_p.parse(input)?;
        let (r, input) = self.mid_p.parse(input)?;
        let (_, input) = self.right_p.parse(input)?;
        Ok((r, input))
    }
}

pub struct Eof {}
impl Parser<()> for Eof {
    fn parse<'b>(&self, input : &'b str) -> Result<((), &'b str), ParseError> {
        if input.len() == 0 {
            Ok(((), input))
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

pub struct True {}
impl Parser<()> for True {
    fn parse<'b>(&self, input : &'b str) -> Result<((), &'b str), ParseError> {
        Ok(((), input))
    }
}

pub struct False {}
impl Parser<()> for False {
    fn parse<'b>(&self, input : &'b str) -> Result<((), &'b str), ParseError> {
        Err(ParseError {
            filename: "stdin".to_string(),
            line: 0,
            char: 0,
            explanation: "FAIL".to_string(),
        })
    }
}