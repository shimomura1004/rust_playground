use std::fmt;
use std::string;

// todo: implement Parser with function? Fn<T> (&str) -> Result((T, &str), ParseError)
pub trait Parser<T> {
    fn parse(&self, input : &mut String) -> Result<T, ParseError>;
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

pub struct Many<'a, T: 'a> {
    pub p : &'a Parser<T>,
}
impl<'a, T> Parser<Vec<T>> for Many<'a, T> {
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

pub struct SkipMany<'a, T: 'a> {
    pub p: &'a Parser<T>,
}
impl<'a, T> Parser<()> for SkipMany<'a, T> {
    fn parse(&self, input : &mut String) -> Result<(), ParseError> {
        Many{p: self.p}.parse(input)?;
        Ok(())
    }
}

pub struct Many1<'a, T: 'a> {
    pub p : &'a Parser<T>,
}
impl<'a, T> Parser<Vec<T>> for Many1<'a, T> {
    fn parse(&self, input : &mut String) -> Result<Vec<T>, ParseError> {
        let r = self.p.parse(input)?;
        let many = Many {p: self.p};
        let mut rs = many.parse(input)?;
        rs.push(r);
        rs.rotate_right(1);
        Ok(rs)
    }
}

pub struct Try<'a, T: 'a> {
    pub ps : Vec<&'a Parser<T>>,
}
impl<'a, T> Parser<T> for Try<'a, T> {
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

// pub struct Then<'a, T1: 'a, T2: 'a> {
//     pub p1 : &'a Parser<T1>,
//     pub p2 : &'a Parser<T2>,
// }
// impl<'a, T1, T2> Then<'a, T1, T2> {
//     pub fn new(p1: &'a Parser<T1>, p2: &'a Parser<T2>) -> Then<'a, T1, T2> {
//         Then {p1, p2}
//     }
// }
// impl<'a, T1, T2> Parser<T2> for Then<'a, T1, T2> {
//     fn parse(&self, input : String) -> Result<(T2, String), ParseError> {
//         let (_, input) = self.p1.parse(input)?;
//         self.p2.parse(input)
//     }
// }

pub struct OneOf {
    pub cs: Vec<char>,
}
impl OneOf {
    pub fn new(chars: &str) -> OneOf {
        OneOf {cs: chars.chars().collect::<Vec<char>>()}
    }
}
impl Parser<char> for OneOf {
    fn parse(&self, input : &mut String) -> Result<char, ParseError> {
        let mut ps : Vec<&Parser<char>> = vec![];
        // for c in &self.cs {
        //     ps.push(&Char{c: *c});
        // }

        Try{ps}.parse(input)
    }
}

// pub struct Digit {}
// impl Parser<i32> for Digit {
//     fn parse(&self, input : String) -> Result<(i32, String), ParseError> {
//         let (digit_str, input) = Many1{p: &OneOf::new("0123456789")}.parse(input)?;
//         let digit_str : String = digit_str.into_iter().collect();
//         let i = digit_str.parse::<i32>().unwrap();
//         Ok((i, input))
//     }
// }

// pub struct Lower {}
// impl Parser<char> for Lower {
//     fn parse(&self, input : String) -> Result<(char, String), ParseError> {
//         OneOf::new("abcdefghijklmnopqrstuvwxyz").parse(input)
//     }
// }

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

// pub struct Space {}
// impl Parser<char> for Space {
//     fn parse(&self, input : String) -> Result<(char, String), ParseError> {
//         OneOf::new(" \t\n\r").parse(input)
//     }
// }

// pub struct Spaces {}
// impl Parser<()> for Spaces {
//     fn parse(&self, input : String) -> Result<((), String), ParseError> {
//         SkipMany::new(&Space{}).parse(input)
//     }
// }

// pub struct Between<'a, T1: 'a, T2: 'a, T3: 'a> {
//     pub left_p: &'a Parser<T1>,
//     pub mid_p: &'a Parser<T2>,
//     pub right_p: &'a Parser<T3>,
// }
// impl<'a, T1, T2, T3> Parser<T2> for Between<'a, T1, T2, T3> {
//     fn parse(&self, input : String) -> Result<(T2, String), ParseError> {
//         let (_, input) = self.left_p.parse(input)?;
//         let (r, input) = self.mid_p.parse(input)?;
//         let (_, input) = self.right_p.parse(input)?;
//         Ok((r, input))
//     }
// }

// pub struct Eof {}
// impl<'a> Parser<()> for Eof {
//     fn parse(&self, input : String) -> Result<((), String), ParseError> {
//         if input.len() == 0 {
//             Ok(((), input))
//         }
//         else {
//             Err(ParseError {
//                 filename: "stdin".to_string(),
//                 line: 0,
//                 char: 0,
//                 explanation: format!("expected EOF but got {}", input.chars().next().unwrap()),
//             })
//         }
//     }
// }

// pub struct SepBy<'a, T1: 'a, T2: 'a> {
//     pub p: &'a Parser<T1>,
//     pub sep: &'a Parser<T2>,
// }
// impl<'a, T1, T2> Parser<Vec<T1>> for SepBy<'a, T1, T2> {
//     fn parse(&self, input : String) -> Result<(Vec<T1>, String), ParseError> {
//         let mut results = vec![];
//         let mut rest = input;

//         loop {
//             let (result, input) = self.p.parse(rest)?;
//             rest = input;
//             results.push(result);

//             match self.sep.parse(rest.clone()) {
//                 Ok((_, input)) => rest = input,
//                 Err(_) => break,
//             }
//         }
//         Ok((results, rest.clone()))
//     }
// }
