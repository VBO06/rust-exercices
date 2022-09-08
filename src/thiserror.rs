use thiserror::Error;

enum LoginError {
    #[error("database error")]
    DatabaseError(#[from] SqlError),

    #[error("password expired")]
    PasswordExpired,

    #[error("user not found")]
    UserNotFound,

    #[error("network connection error")]
    NetworkError (#[from] std::io::Error),

    #[error("wrong password")]
    WrongPassowrd,
}