use actix_web::http::StatusCode;
use derive_more::{Display, Error, From};



#[derive(Debug, Display, Error, From)]
pub enum PersistenceError {
    EmptyEmail,
    EmptyUsername,
    EmptyPassword,
    BcryptError(bcrypt::BcryptError),
    MysqlError(mysql::Error),
    UnknownUser,
    Unknown,
}

//matches a PersistenceError to a StatusCode
impl actix_web::ResponseError for PersistenceError {
    fn status_code(&self) -> StatusCode {
        match self {
            PersistenceError::EmptyEmail => StatusCode::BAD_REQUEST,
            PersistenceError::EmptyUsername => StatusCode::BAD_REQUEST,
            PersistenceError::UnknownUser => StatusCode::UNAUTHORIZED,
            PersistenceError::EmptyPassword => StatusCode::BAD_REQUEST,
            PersistenceError::BcryptError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            PersistenceError::MysqlError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            PersistenceError::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}