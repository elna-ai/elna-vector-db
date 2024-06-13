use candid::CandidType;

#[derive(Debug, thiserror::Error, PartialEq, CandidType)]
pub enum Error {
    #[error("Collection already exists")]
    UniqueViolation,

    #[error("Collection doesn't exist")]
    NotFound,

    #[error("The dimension of the vector doesn't match the dimension of the collection")]
    DimensionMismatch,
    #[error("User not authorized")]
    Unauthorized,
    #[error("Memory error")]
    MemoryError,
}
impl From<Error> for String {
    fn from(error: Error) -> Self {
        // Convert the Error to a String representation
        error.to_string()
    }
}
