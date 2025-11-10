use thiserror::Error;

#[derive(Error, Debug)]
pub enum KzgError {
    #[error("polynomial length {polynomial_len} does not match powers of s length {powers_len}")]
    PolynomialLengthMismatch {
        polynomial_len: usize,
        powers_len: usize,
    },
    #[error("failed to divide polynomial by (x - z) during proof generation")]
    PolynomialDivisionFailed,
    #[error("invalid evaluation point")]
    InvalidEvaluationPoint,
}

pub type Result<T> = std::result::Result<T, KzgError>;
