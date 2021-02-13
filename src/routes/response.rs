use rocket::*;
use rocket::response;
use rocket::http::{ContentType, Status};
use rocket::response::{Responder, Response};
use rocket_contrib::json::Json;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ApiError {
    pub reason: String,
    pub message: String
}

pub enum ApiResult<T, E>{
    Ok(T),
    Error(E)
}

pub struct ApiResponse<T, E> {
    status: Status,
    result: ApiResult<T, E>
}

impl<T> ApiResponse<T, ApiError> {

    pub fn ok(value: T) -> Self {
        ApiResponse{
            status: Status::Ok,
            result: ApiResult::Ok(value)
        }
    }

    pub fn created(value: T) -> Self {
        ApiResponse{
            status: Status::Created,
            result: ApiResult::Ok(value)
        }
    }

    pub fn no_content(value: T) -> Self {
        ApiResponse{
            status: Status::NoContent,
            result: ApiResult::Ok(value)
        }
    }

    pub fn error(status: Status, message: String) -> Self {
        ApiResponse{
            status: status,
            result: ApiResult::Error(ApiError{
                reason: String::from(status.reason),
                message: message
            }),
        }
    }

}

impl<'r,T: Serialize> Responder<'r> for ApiResponse<T, ApiError> {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        match self.result {
            ApiResult::Ok(v) => Response::build_from(Json(v).respond_to(&req).unwrap()),
            ApiResult::Error(e) => Response::build_from(Json(e).respond_to(&req).unwrap())
        }
        .status(self.status)
        .header(ContentType::JSON)
        .ok()
    }
}