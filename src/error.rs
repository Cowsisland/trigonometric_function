use thiserror::Error;

#[derive(Debug, Error)]
pub enum TrigError {
    #[error("Input value {0} is out of range for arcsin/arccos/tan. Valid range is [-1, 1]")]
    OutOfRange(f64),
}