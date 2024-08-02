use thiserror::Error;

#[derive(Debug, Error)]
pub enum TrigError {
    #[error("Input value {0} is out of range for tan. Valid range is not (-0.5, 1.5)")]
    OutOfRangeForTan(f64),

    #[error("Input value {0} is out of range for arcsin. Valid range is [-1, 1]")]
    OutOfRangeForArcsin(f64),

    #[error("Input value {0} is out of range for arccos. Valid range is [-1, 1]")]
    OutOfRangeForArccos(f64),

    #[error("Overflow occurred during calculation")]
    Overflow,
}