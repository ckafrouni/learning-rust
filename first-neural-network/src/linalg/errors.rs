use thiserror::Error;

#[derive(Debug, Error)]
pub enum MatrixError {
    #[error("Dimension mismatch: the operation cannot be performed with matrices of different dimensions.")]
    DimensionMismatch,
    #[error("Singular matrix: the matrix does not have an inverse.")]
    SingularMatrix,
    #[error("Invalid operation: the operation is not valid for the given matrix dimensions.")]
    InvalidOperation,
}
