use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct Ley {
    structures: HashMap<&'static str, Box<dyn Structure>>
}
impl Ley {
    pub fn new() -> Self {
        Default::default()
    }
    /// Add a ley line, parsing the contents using the structure specified
    pub fn ley_line<'a>(&mut self, name: &'a str, structure: &'a str, content: &'a str) {
        println!("Got !{}: {} {{ {:?} }}", name, structure, content);
    }
}

/// An implementation of a ley line kind
/// A `Structure` handles both the parsing and content generation of a ley line
pub trait Structure: std::fmt::Debug {
    fn parse_line(&mut self, ) -> Vec<Contents>;
}

#[derive(Debug)]
pub struct LeyLine<'a> {
    name: &'a str,
    structure: &'a str,
    content: Vec<Contents<'a>>
}

#[derive(Debug)]
pub enum Contents<'a> {
    Exact(&'a str),
    Paragraph(Vec<Grammar<'a>>),
    LeyLine(LeyLine<'a>)
}

/// An individual piece of grammar for reconstruction as formatted natural language
#[derive(Debug)]
pub enum Grammar<'a> {
    Word(&'a str),
    SentenceEnd(&'a str),
    Comma,
    Hyphen,
    Quotation
}