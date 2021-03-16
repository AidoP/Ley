use std::{collections::HashMap, ops::Not};
use crate::error::{Error, Result};

#[derive(Default, Debug)]
pub struct Ley<'a> {
    structures: HashMap<&'static str, Box<dyn Structure>>,
    lines: Vec<LeyLine<'a>>
}
impl<'a> Ley<'a> {
    pub fn parse(mut stream: &'a str) -> Result<Self> {
        let structures = Default::default();
        let mut lines = Vec::new();
        while { stream = stream.trim_start(); !stream.is_empty() } {
            if let Some(line) = LeyLine::parse(&mut stream)? {
                lines.push(line)
            }
        }
        Ok(Self {
            structures,
            lines
        })
    }
    pub fn lookup(&mut self, structure: &str) -> Option<&mut Box<dyn Structure>> {
        self.structures.get_mut(structure)
    }
}

/// An implementation of a ley line kind
/// A `Structure` handles both the parsing and content generation of a ley line
pub trait Structure: std::fmt::Debug {
    //fn parse<'a>(&mut self, stream: &'a mut str) -> Vec<Contents<'a>>;
}

#[derive(Debug)]
pub struct LeyLine<'a> {
    name: &'a str,
    structure: &'a str,
    content: Content<'a>
}
impl<'a> LeyLine<'a> {
    pub fn parse(stream: &mut &'a str) -> Result<Option<Self>> {
        #[derive(PartialEq, Eq)]
        pub enum Kind {
            Comment(usize),
            Normal(usize),
            None
        }
        impl Kind {
            fn is_comment(&self) -> bool {
                match self {
                    Kind::Comment(_) => true,
                    _ => false
                }
            }
        }
        let (name, comment) = match stream.find(|c| c != '!') {
            Some(0) => Err(Error::InvalidLeyLine)?,
            Some(open_count) => {
                let mut kind = Kind::None;
                *stream = &stream[open_count..];
                stream.char_indices().find_map(|(i, c)| {
                    match (c, &mut kind) {
                        (':', Kind::Normal(count)) | (';', Kind::Comment(count)) => *count += 1,
                        (':', _) => kind = Kind::Normal(1),
                        (';', _) => kind = Kind::Comment(1),
                        _ => kind = Kind::None
                    }
                    match kind {
                        Kind::Comment(c) | Kind::Normal(c) if c == open_count => {
                            let (content, s) = stream.split_at(i+1);
                            *stream = s;
                            Some((&content[..content.len() - open_count], kind.is_comment()))
                        },
                        _ => None
                    }
                }).ok_or(Error::UnexpectedEoF)?
            },
            None => Err(Error::UnexpectedEoF)?
        };
        let structure = {
            let i = stream.find(['{', '['].as_ref()).ok_or(Error::UnexpectedEoF)?;
            let (structure, s) = stream.split_at(i);
            *stream = s;
            structure.trim()
        };
        let content = Content::parse(stream)?;

        Ok(comment.not().then(|| Self {
            name,
            structure,
            content
        }))
    }
}

#[derive(Debug)]
pub enum Content<'a> {
    Text(&'a str),
    Nested(Vec<LeyLine<'a>>)
}
impl<'a> Content<'a> {
    fn parse(stream: &mut &'a str) -> Result<Self> {
        match stream.chars().next() {
            Some('{') => {
                let mut nested_lines = Vec::new();
                match stream.find(|c| c != '{') {
                    Some(0) => Err(Error::InvalidLeyLine)?,
                    Some(open_count) => {
                        *stream = &stream[open_count..];
                        while let Some('!') = { *stream = stream.trim_start(); stream.chars().next() } {
                            if let Some(line) = LeyLine::parse(stream)? {
                                nested_lines.push(line)
                            }
                        }
                        if stream.len() < open_count {
                            Err(Error::UnexpectedEoF)?
                        }
                        let (close_braces, s) = stream.split_at(open_count);
                        *stream = s;
                        if close_braces != "}".repeat(open_count) {
                            Err(Error::InvalidLeyLine)?
                        }
                    },
                    None => Err(Error::UnexpectedEoF)?
                }
                Ok(Self::Nested(nested_lines))
            }
            Some('[') => {
                let text = match stream.find(|c| c != '[') {
                    Some(0) => Err(Error::InvalidLeyLine)?,
                    Some(open_count) => {
                        *stream = &stream[open_count..];
                        let i = stream.find(&"]".repeat(open_count)).ok_or(Error::UnexpectedEoF)?;
                        let (text, s) = stream.split_at(i);
                        *stream = &s[open_count..];
                        text
                    },
                    None => Err(Error::UnexpectedEoF)?
                };
                Ok(Self::Text(text))
            }
            Some(_) => unreachable!(),
            None => Err(Error::UnexpectedEoF)
        }
    }
}