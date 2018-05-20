pub trait Parser<T, E> {
    fn parse<'a>(&self, input : &'a str) -> Result<(T, &'a str), E>;
}

pub struct ParserChar {
    pub c : char,
}

impl Parser<char, ()> for ParserChar {
    fn parse<'a>(&self, input : &'a str) -> Result<(char, &'a str), ()> {
        match input.chars().next() {
            Some(c) => {
                if c == self.c {
                    Ok((c, &input[1..]))
                }
                else {
                    Err(())
                }
            }
            None => Err(())
        }
    }
}

pub struct ParserMany<'a, T: 'a, E: 'a> {
    pub p : &'a Parser<T, E>,
}

impl<'a, T: 'a, E: 'a> Parser<Vec<T>, E> for ParserMany<'a, T, E> {
    fn parse<'b>(&self, input : &'b str) -> Result<(Vec<T>, &'b str), E> {
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

pub struct ParserMany1<'a, T: 'a, E: 'a> {
    pub p : &'a Parser<T, E>,
}

impl<'a, T: 'a, E: 'a> Parser<Vec<T>, E> for ParserMany1<'a, T, E> {
    fn parse<'b>(&self, input : &'b str) -> Result<(Vec<T>, &'b str), E> {
        let (r, input) = self.p.parse(input)?;
        let many = ParserMany {p: self.p};
        let (mut rs, input) = many.parse(input)?;
        rs.push(r);
        Ok((rs, input))
    }
}

pub struct ParserTry<'a, T: 'a, E: 'a> {
    pub ps : Vec<&'a Parser<T, E>>,
}

impl<'a, T: 'a, E: 'a> Parser<T, E> for ParserTry<'a, T, E> {
    fn parse<'b>(&self, input : &'b str) -> Result<(T, &'b str), E> {
        let mut r = self.ps[0].parse(input);
        if !r.is_ok() {
            for &p in &self.ps {
                r = p.parse(input);
                if r.is_ok() {
                    break;
                }
            }
        }
        r
    }
}
