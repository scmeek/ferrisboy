use num_enum::{TryFromPrimitive, TryFromPrimitiveError};
use std::io;
use thiserror::Error;

// TODO: Separate crate exposed errors and internal errors
#[derive(Debug, Error)]
pub enum FerrisBoyError {
    #[error("Something went wrong: {0}")]
    GeneralError(&'static str),

    #[error(transparent)]
    IoError(#[from] io::Error),

    #[error("Invalid input: expected {expected} bytes but got {actual} bytes")]
    InvalidInput { expected: usize, actual: usize },

    #[error("Internal error: {0}")]
    InternalError(&'static str),

    #[error("Invalid enum value: {0}")]
    InvalidEnumValue(&'static str, u16), // u8-compatible
}

impl<T> From<TryFromPrimitiveError<T>> for FerrisBoyError
where
    T: TryFromPrimitive,
    T::Primitive: Into<u16> + Copy,
{
    fn from(err: TryFromPrimitiveError<T>) -> Self {
        FerrisBoyError::InvalidEnumValue(std::any::type_name::<T>(), err.number.into())
    }
}
