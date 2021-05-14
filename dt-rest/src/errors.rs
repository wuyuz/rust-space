use crate::model::response::ResponseBody;
use serde_json::Error as JsonError;
use std::fmt;
use mongodb::error::Error as MongoErr;
use actix_web::{
    HttpResponse,
    ResponseError,
    Error,
    http::header,
    dev::HttpResponseBuilder,
    http::StatusCode,
    error::{BlockingError, Error as ActixError},
};
use derive_more::{Display, Error};


#[derive(Debug)]
pub struct ServiceError {
    pub http_status: StatusCode,
    pub body: ResponseBody<String>,
}

impl ServiceError {
    pub fn new(http_status: StatusCode, message: String) -> ServiceError {
        ServiceError {
            http_status,
            body: ResponseBody {
                message,
                data: String::new(),
            }
        }
    }

    pub fn response(&self) -> HttpResponse {
        HttpResponse::build(self.http_status).json(&self.body)
    }

    pub fn error_response(&self) -> Error {
        HttpResponse::build(self.http_status).json(&self.body).into()
    }
}


impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        let status_code = match StatusCode::from_u16(self.http_status.into()) {
            Ok(status_code) => status_code,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let message = match status_code.as_u16() > 500 {
            true => "Ok".to_string(),
            false => {
                error!("{:?}", self.body);
                // "Internal server error".to_string()
                self.body.message.to_string()
            },
        };

        HttpResponse::build(status_code)
            .json(json!({ "message": message }))
    }
}

impl From<ActixError> for ServiceError {
    fn from(error: ActixError) -> ServiceError {
        ServiceError::new(StatusCode::SERVICE_UNAVAILABLE, error.to_string())
    }
}




impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.http_status.as_str())
    }
}



impl From<JsonError> for ServiceError {
    fn from(error: JsonError) -> ServiceError {
        ServiceError::new(StatusCode::BAD_REQUEST, error.to_string())
    }
}


impl From<MongoErr> for ServiceError {
    fn from(error: MongoErr) -> ServiceError {
        match error {
            err => ServiceError::new(StatusCode::SERVICE_UNAVAILABLE, format!("Mongo error: {}",err)),
        }
    }
}

/// Convert BlockingError to ServiceErrors
impl From<BlockingError<ServiceError>> for ServiceError {
    fn from(error: BlockingError<ServiceError>) -> ServiceError {
        ServiceError::new(StatusCode::BAD_REQUEST, error.to_string())
    }
}