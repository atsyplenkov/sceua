use std::{error::Error, fmt};

#[derive(Debug, Clone, PartialEq)]
pub enum SceuaError {
    EmptyProblem,
    BoundsLengthMismatch {
        lower: usize,
        upper: usize,
    },
    InitialPointLengthMismatch {
        expected: usize,
        actual: usize,
    },
    InvalidBounds {
        index: usize,
        lower: f64,
        upper: f64,
    },
    InvalidConfig(&'static str),
    NonFiniteObjective {
        value: f64,
    },
}

impl fmt::Display for SceuaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyProblem => write!(f, "at least one parameter is required"),
            Self::BoundsLengthMismatch { lower, upper } => write!(
                f,
                "lower and upper bounds must have the same length, got {lower} and {upper}"
            ),
            Self::InitialPointLengthMismatch { expected, actual } => {
                write!(f, "initial point has length {actual}, expected {expected}")
            }
            Self::InvalidBounds {
                index,
                lower,
                upper,
            } => write!(
                f,
                "invalid bounds at index {index}: lower={lower}, upper={upper}"
            ),
            Self::InvalidConfig(message) => write!(f, "invalid SCE-UA configuration: {message}"),
            Self::NonFiniteObjective { value } => {
                write!(f, "objective returned a non-finite value: {value}")
            }
        }
    }
}

impl Error for SceuaError {}
