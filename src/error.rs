#[derive(Debug, PartialEq, thiserror::Error)]
pub enum EnvDeserializationError {
    #[error("An error occured during deserialization: {}", .0)]
    GenericDeserialization(String),
    #[error("An unsupported variant was tried to be deserialized. Only structs and maps are currently supported.")]
    UnsupportedValue,
    #[error("Tried to nest values while a simple value was expected")]
    InvalidNestedValues,
}

impl serde::de::Error for EnvDeserializationError {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        EnvDeserializationError::GenericDeserialization(msg.to_string())
    }
}
