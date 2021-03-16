pub type Result<T> = std::result::Result<T, Error>;
#[derive(Debug)]
pub enum Error {
    UnexpectedEoF(String),
    Expected(String),
}