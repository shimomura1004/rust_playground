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

impl Parser<char> for OneOf {
    fn parse<'b>(&self, input : &'b str) -> Result<(char, &'b str), ParseError> {
        let mut ps : Vec<Box<Parser<char>>> = vec![];
        for c in &self.cs {
            ps.push(Box::new(Char{c: *c}));
        }
        Try{ps}.parse(input)

        // let p0 : &Parser<char> = &Char{c: self.cs[0]};
        // let p1 = Char{c: self.cs[0]};
        // let ps = vec![p0, &p1];
        // Try{ps: &ps}.parse(input)
    }
}

// pub struct Digit {}

// impl Parser<i32> for Digit {
//     fn parse<'b>(&self, input : &'b str) -> Result<(T, &'b str), ParseError> {
//         Many{p: }
//     }
// }