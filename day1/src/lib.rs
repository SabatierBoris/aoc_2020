use std::io;
use std::num;

#[derive(Debug)]
pub enum ReadNumberError {
    IoError(io::Error),
    ParseError(num::ParseIntError),
}
impl From<io::Error> for ReadNumberError {
    fn from(error: io::Error) -> Self {
        ReadNumberError::IoError(error)
    }
}
impl From<num::ParseIntError> for ReadNumberError {
    fn from(error: num::ParseIntError) -> Self {
        ReadNumberError::ParseError(error)
    }
}

pub fn read_number() -> Result<i32, ReadNumberError> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;
    let number = input.trim().parse::<i32>()?;

    Ok(number)
}
