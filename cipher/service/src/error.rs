use actix_web::{
    http::StatusCode,
    HttpResponse, 
    Error,
    ResponseError,
    error::{BlockingError, Error as ActixError},
};
use diesel::{
    r2d2::PoolError,
    result::{DatabaseErrorKind, Error as DBError},
};
use serde::Deserialize;
use serde_json::json;
use derive_more::Display;
use std::fmt;
use std::ops::Deref;
use uuid::Error as UuidError;
use tracing::{error};

#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub status_code: u16,
    pub message: String,
}

/// Convert Thread BlockingErrors to ApiErrors
impl From<BlockingError<ApiError>> for ApiError {
    fn from(error: BlockingError<ApiError>) -> ApiError {
        match error {
            BlockingError::Error(api_error) => api_error,
            BlockingError::Canceled => ApiError::new(805, "Thread blocking error".into()),
        }
    }
}

impl ApiError {
    pub fn new(status_code: u16, message: String ) -> ApiError {
        ApiError{status_code, message}
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.message.as_str())
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let status_code = match StatusCode::from_u16(self.status_code) {
            Ok(status_code) => status_code,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let message = match status_code.as_u16() > 500 {
            true => self.message.clone(),
            false => {
                error!("{}", self.message);
                "Internal server error".to_string()
            },
        };

        HttpResponse::build(status_code)
            .json(json!({ "message": message }))
    }
}

impl From<ActixError> for ApiError {
    fn from(error: ActixError) -> ApiError {
        ApiError::new(800, error.to_string())
    }
}


#[derive(Debug, Display, PartialEq, Serialize)]
pub enum ServError {
    BadRequest(String),
    BlockingError(String),
    CacheError(String),
    DecodeTokenError(String),
    EncodeTokenError(String),
    InternalServerError(String),
    NotFound(String),
    UuidError(String),
    DataBaseError(String),

    #[display(fmt = "")]
    ValidationError(Vec<String>),
    Unauthorized(String),
}


/// 自定义错误
impl ResponseError for ServError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServError::BadRequest(error) => {
                HttpResponse::BadRequest().json(ServError::BadRequest(error.to_owned()))
            }
            ServError::NotFound(message) => {
                HttpResponse::NotFound().json(ServError::NotFound(message.to_owned()))
            }
            ServError::ValidationError(errors) => HttpResponse::UnprocessableEntity()
                .json(ServError::ValidationError(errors.to_owned())),
            ServError::Unauthorized(error) => {
                HttpResponse::Unauthorized().json(ServError::Unauthorized(error.to_owned()))
            }
            _ => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

// Convert DBErrors to ServiceErrors
impl From<DBError> for ServError {
    fn from(error: DBError) -> ServError {
        // Right now we just care about UniqueViolation from diesel
        // But this would be helpful to easily map errors as our app grows
        match error {
            DBError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_string();
                    return ServError::BadRequest(message);
                }
                ServError::InternalServerError("Unknown database error".into())
            }
            _ => ServError::InternalServerError("Unknown database error".into()),
        }
    }
}

/// Convert PoolErrors to ServiceErrors
impl From<PoolError> for ServError {
    fn from(error: PoolError) -> ServError {
        ServError::DataBaseError(error.to_string())
    }
}

/// Convert ParseErrors to ServiceErrors
impl From<UuidError> for ServError {
    fn from(error: UuidError) -> ServError {
        ServError::UuidError(error.to_string())
    }
}

/// Convert BlockingError to ServiceErrors
impl From<BlockingError<ServError>> for ServError {
    fn from(error: BlockingError<ServError>) -> ServError {
        ServError::BlockingError(error.to_string())
    }
}