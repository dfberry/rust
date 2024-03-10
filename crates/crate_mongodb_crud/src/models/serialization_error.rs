use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum MySerializationError {
    SerializationError(String),
}

impl fmt::Display for MySerializationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MySerializationError::SerializationError(e) => write!(f, "Serialization error: {}", e),
        }
    }
}
impl Error for MySerializationError {}