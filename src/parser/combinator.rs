use std::fmt;
use std::string;

// todo: implement Parser with function? Fn<T> (&str) -> Result((T, &str), ParseError)
pub trait Parser<'a, T> {
    fn parse(&'a self, input : &'a str) -> Result<(T, &'a str), ParseError>;
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

impl<'a> Parser<'a, char> for Char {
    fn parse(&self, input : &'a str) -> Result<(char, &'a str), ParseError> {
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
    pub p : &'a Parser<'a, T>,
}
impl<'a, T: 'a> Parser<'a, Vec<T>> for Many<'a, T> {
    fn parse(&'a self, input : &'a str) -> Result<(Vec<T>, &'a str), ParseError> {
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

pub struct SkipMany<'a, T: 'a> {
    // pub p : &'a Parser<'a, T>,
    many : &'a Many<'a, T>,
}
impl<'a, T: 'a> SkipMany<'a, T> {
    pub fn new(p : &'a Parser<'a, T>) -> SkipMany<'a, T> {
        SkipMany{many: &Many{p}}
    }
}
impl<'a, T: 'a> Parser<'a, ()> for SkipMany<'a, T> {
    fn parse(&'a self, input : &'a str) -> Result<((), &'a str), ParseError> {
        // let many = Many{p: self.p};
        // let aaa = &many;
        let (_, input) = self.many.parse(input)?;
        Ok(((), input))
    }
}

pub struct Many1<'a, T: 'a> {
    pub p : &'a Parser<'a, T>,
}
impl<'a, T: 'a> Parser<'a, Vec<T>> for Many1<'a, T> {
    fn parse(&self, input : &'a str) -> Result<(Vec<T>, &'a str), ParseError> {
        let (r, input) = self.p.parse(input)?;
        let many = Many {p: self.p};
        let (mut rs, input) = many.parse(input)?;
        rs.push(r);
        rs.rotate_right(1);
        Ok((rs, input))
    }
}

pub struct Try<'a, T: 'a> {
    pub ps : Vec<Box<Parser<'a, T>>>,
}
impl<'a, T> Parser<'a, T> for Try<'a, T> {
    fn parse(&'a self, input : &'a str) -> Result<(T, &'a str), ParseError> {
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

pub struct Then<'a, T1: 'a, T2: 'a> {
    pub p1 : &'a Parser<'a, T1>,
    pub p2 : &'a Parser<'a, T2>,
}
impl<'a, T1, T2> Then<'a, T1, T2> {
    pub fn new(p1: &'a Parser<'a, T1>, p2: &'a Parser<'a, T2>) -> Then<'a, T1, T2> {
        Then {p1, p2}
    }
}
impl<'a, T1, T2> Parser<'a, T2> for Then<'a, T1, T2> {
    fn parse(&self, input : &'a str) -> Result<(T2, &'a str), ParseError> {
        let (_, input) = self.p1.parse(input)?;
        self.p2.parse(input)
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
impl<'a> Parser<'a, char> for OneOf {
    fn parse(&self, input : &'a str) -> Result<(char, &'a str), ParseError> {
        let mut ps : Vec<Box<Parser<char>>> = vec![];
        for c in &self.cs {
            ps.push(Box::new(Char{c: *c}));
        }
        Try{ps}.parse(input)
    }
}

pub struct Digit {}
impl<'a> Parser<'a, i32> for Digit {
    fn parse(&self, input : &'a str) -> Result<(i32, &'a str), ParseError> {
        let (digit_str, input) = Many1{p: &OneOf::new("0123456789")}.parse(input)?;
        let digit_str : String = digit_str.into_iter().collect();
        let i = digit_str.parse::<i32>().unwrap();
        Ok((i, input))
    }
}

pub struct Lower {}
impl<'a> Parser<'a, char> for Lower {
    fn parse(&self, input : &'a str) -> Result<(char, &'a str), ParseError> {
        OneOf::new("abcdefghijklmnopqrstuvwxyz").parse(input)
    }
}

pub struct Upper {}
impl<'a> Parser<'a, char> for Upper {
    fn parse(&self, input : &'a str) -> Result<(char, &'a str), ParseError> {
        OneOf::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ").parse(input)
    }
}

pub struct Letter {}
impl<'a> Parser<'a, char> for Letter {
    fn parse(&self, input : &'a str) -> Result<(char, &'a str), ParseError> {
        Try{ps: vec![Box::new(Upper{}), Box::new(Lower{})]}.parse(input)
    }
}

pub struct Space {}
impl<'a> Parser<'a, char> for Space {
    fn parse(&self, input : &'a str) -> Result<(char, &'a str), ParseError> {
        OneOf::new(" \t\n\r").parse(input)
    }
}

pub struct Spaces<'a> {
    skipmany: &'a SkipMany<'a, char>,
}
impl<'a> Spaces<'a> {
    pub fn new() -> Spaces<'a> {
        Spaces {skipmany: &SkipMany::new(&Space{})}
    }
}
impl<'a> Parser<'a, ()> for Spaces<'a> {
    fn parse(&self, input : &'a str) -> Result<((), &'a str), ParseError> {
        // SkipMany{p: &Space{}}.parse(input)
        self.skipmany.parse(input)
        // SkipMany::new(&Space{}).parse(input)
    }
}

pub struct Between<'a, T1: 'a, T2: 'a, T3: 'a> {
    pub left_p: &'a Parser<'a, T1>,
    pub mid_p: &'a Parser<'a, T2>,
    pub right_p: &'a Parser<'a, T3>,
}
impl<'a, T1, T2, T3> Parser<'a, T2> for Between<'a, T1, T2, T3> {
    fn parse(&self, input : &'a str) -> Result<(T2, &'a str), ParseError> {
        let (_, input) = self.left_p.parse(input)?;
        let (r, input) = self.mid_p.parse(input)?;
        let (_, input) = self.right_p.parse(input)?;
        Ok((r, input))
    }
}

pub struct Eof {}
impl<'a> Parser<'a, ()> for Eof {
    fn parse(&self, input : &'a str) -> Result<((), &'a str), ParseError> {
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

pub struct SepBy<'a, T1: 'a, T2: 'a> {
    pub p: &'a Parser<'a, T1>,
    pub sep: &'a Parser<'a, T2>,
}
impl<'a, T1, T2> Parser<'a, Vec<T1>> for SepBy<'a, T1, T2> {
    fn parse(&self, input : &'a str) -> Result<(Vec<T1>, &'a str), ParseError> {
        let mut results = vec![];
        let mut rest = &input[..];
        loop {
            let (result, input) = self.p.parse(rest)?;
            rest = input;
            results.push(result);

            match self.sep.parse(rest) {
                Ok((_, input)) => rest = input,
                Err(_) => break,
            }
        }
        Ok((results, rest))
    }
}

pub struct True {}
impl<'a> Parser<'a, ()> for True {
    fn parse(&self, input : &'a str) -> Result<((), &'a str), ParseError> {
        Ok(((), input))
    }
}

pub struct False {}
impl<'a> Parser<'a, ()> for False {
    fn parse(&self, input : &'a str) -> Result<((), &'a str), ParseError> {
        Err(ParseError {
            filename: "stdin".to_string(),
            line: 0,
            char: 0,
            explanation: "FAIL".to_string(),
        })
    }
}