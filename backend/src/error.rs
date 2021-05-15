use deadpool_redis::PoolError;
use redis::RedisError;
use rocket::response::Responder;
use rocket_contrib::json::Json;
use serde::Serialize;
use strum;

#[derive(Serialize, Debug)]
pub struct Error {
    // status: Status, #TODO: do we need to repeat the status code ?
    message: String,
    description: Option<String>,
}

impl From<String> for Error {
    fn from(message: String) -> Error {
        Error {
            message,
            description: None,
        }
    }
}

#[derive(Responder, Debug)]
pub enum ApiError {
    #[response(status = 400, content_type = "json")]
    UserError(Json<Error>),

    #[response(status = 500, content_type = "json")]
    RedisPoolError(Json<Error>),

    #[response(status = 500, content_type = "json")]
    RedisError(Json<Error>),

    #[response(status = 500, content_type = "json")]
    EmptyModelError(Json<Error>),

    #[response(status = 500, content_type = "json")]
    FormParsingError(Json<Error>),

    #[response(status = 500, content_type = "json")]
    IOError(Json<Error>),
}
impl From<PoolError> for ApiError {
    fn from(err: PoolError) -> ApiError {
        ApiError::RedisPoolError(Json(Error {
            message: String::from("RedisPoolError: Unable to pool a connection to redis"),
            description: Some(format!("From: {}", err.to_string())),
        }))
    }
}
impl From<RedisError> for ApiError {
    fn from(err: RedisError) -> ApiError {
        ApiError::RedisError(Json(Error {
            message: String::from("RedisError: Unable to parse result or missing key"),
            description: Some(format!("From: {}", err.to_string())),
        }))
    }
}

impl From<strum::ParseError> for ApiError {
    fn from(err: strum::ParseError) -> ApiError {
        ApiError::FormParsingError(Json(Error {
            message: String::from("FormParsingError: Unable to parse the incoming form"),
            description: Some(format!("From: {}", err.to_string())),
        }))
    }
}

impl From<std::io::Error> for ApiError {
    fn from(err: std::io::Error) -> ApiError {
        ApiError::IOError(Json(Error {
            message: String::from("IOError: Unable to read the bytes from model"),
            description: Some(format!("From: {}", err.to_string())),
        }))
    }
}
