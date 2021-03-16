use crate::Ley;
use crate::error::{Error, Result};

pub struct Parser<'a> {
    stream: &'a str,
    context: &'a str,
    position: usize
}
impl<'a> Parser<'a> {
    pub fn new(stream: &'a str) -> Self {
        Self {
            stream,
            context: stream,
            position: 0
        }
    }
    pub fn parse(mut self) -> Result<Ley> {
        let mut ley = Ley::new();
        while { self.stream = self.stream.trim_start(); !self.stream.is_empty() } {
            let name = self.content_between('!', ':')?;
            let structure = self.until('{')?;
            let content = self.content_between('{', '}')?;
            ley.ley_line(name, structure, content)
        }
        Ok(ley)
    }

    pub fn expect(&mut self, pattern: &str) -> Result<()> {
        if self.stream.starts_with(pattern) {
            self.stream = &self.stream[pattern.len()..];
            self.position += pattern.len();
            Ok(())
        } else {
            Err(Error::Expected(pattern.to_string()))
        }
    }
    /// Expect one or more instance of char, returning the count
    pub fn expect_many(&mut self, character: char) -> Result<usize> {
        for (i, c) in self.stream.char_indices() {
            if c != character {
                let i = i-1;
                if i == 0 { break }
                self.stream = &self.stream[i..];
                self.position += i;
                return Ok(i)
            }
        }
        Err(Error::UnexpectedEoF(format!("expecting one or more {:?} before further content", character)))
    }
    /// Collect the stream up until character is encountered
    pub fn until(&mut self, character: char) -> Result<&'a str> {
        if let Some(i) = self.stream.char_indices().find_map(|(i, c)| if c == character { Some(i) } else { None }) {
            let (content, stream) = self.stream.split_at(i);
            self.stream = stream;
            Ok(content)
        } else {
            Err(Error::UnexpectedEoF(format!("{:?}", character)))
        }
    }
    /// Get the string between n*l and n*r
    ///
    /// Eg. for `l=a` and `r=b`, `aa this is an apple bb` -> ` this is an apple `
    pub fn content_between(&mut self, l: char, r: char) -> Result<&'a str> {
        let mut i = 0;
        let n = self.stream.chars().find_map(|c| {
            i += 1;
            if c == l { None } else { Some(i-1) }
        });
        match n {
            Some(0) => Err(Error::Expected(format!("one or more {:?} with content contained between an equal number of {:?}....{:?}", l, r, self.stream))),
            Some(n) => {
                self.stream = &self.stream[l.len_utf8() * n..];
                if let Some(end) = self.stream.find(&r.to_string().repeat(n)) {
                    let (content, stream) = self.stream.split_at(end);
                    self.stream = &stream[r.len_utf8() * n..];
                    Ok(content)
                } else {
                    Err(Error::Expected(format!("{} * {:?}", n, r)))
                }
            },
            None => Err(Error::UnexpectedEoF(format!("expected an equal number of {:?} after {:?}", r, l)))
        }
    }
}