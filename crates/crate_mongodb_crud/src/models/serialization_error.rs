use std::fmt;
use std::error::Error;
use serde_json::Error as SerdeError;
use serde::Deserialize;

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

pub struct MyJsonSerializationError {
    message: String,
}

impl From<SerdeError> for MyJsonSerializationError {
    fn from(error: SerdeError) -> Self {
        MyJsonSerializationError {
            message: error.to_string(),
        }
    }
}