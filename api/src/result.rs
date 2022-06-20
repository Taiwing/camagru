use rocket::http::{ContentType, Method, Status};
use rocket::serde::{json::Json, Serialize};
use rocket::{
    response::{self, Responder},
    Request, Response,
};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ApiError {
    status: u16,
    error: String,
    message: String,
    method: Method,
    path: String,
}

impl ApiError {
    fn new(status: Status, message: &str, method: Method, path: &str) -> Self {
        ApiError {
            status: status.code,
            error: String::from(status.reason_lossy()),
            message: String::from(message),
            method,
            path: String::from(path),
        }
    }
}

pub enum ApiResult<T: Serialize> {
    Success { status: Status, payload: T },
    Failure { status: Status, message: String },
}

#[catch(default)]
pub fn default(status: Status, req: &Request) -> Json<ApiError> {
    Json(ApiError::new(
        status,
        "unexepected error",
        req.method(),
        &req.uri().path().to_string(),
    ))
}

#[catch(400)]
pub fn bad_request(req: &Request) -> Json<ApiError> {
    Json(ApiError::new(
        Status::BadRequest,
        "invalid request",
        req.method(),
        &req.uri().path().to_string(),
    ))
}

#[catch(401)]
pub fn unauthorized(req: &Request) -> Json<ApiError> {
    Json(ApiError::new(
        Status::Unauthorized,
        "user must be logged in to execute this request",
        req.method(),
        &req.uri().path().to_string(),
    ))
}

#[catch(403)]
pub fn forbidden(req: &Request) -> Json<ApiError> {
    Json(ApiError::new(
        Status::Forbidden,
        "this request is not allowed",
        req.method(),
        &req.uri().path().to_string(),
    ))
}

#[catch(404)]
pub fn not_found(req: &Request) -> Json<ApiError> {
    Json(ApiError::new(
        Status::NotFound,
        "requested resource does not exist",
        req.method(),
        &req.uri().path().to_string(),
    ))
}

#[catch(422)]
pub fn unprocessable_entity(req: &Request) -> Json<ApiError> {
    Json(ApiError::new(
        Status::UnprocessableEntity,
        "the request cannot be processed because the payload is ill-formed",
        req.method(),
        &req.uri().path().to_string(),
    ))
}

#[catch(500)]
pub fn internal_error(req: &Request) -> Json<ApiError> {
    Json(ApiError::new(
        Status::InternalServerError,
        "Ooooooops.... Looks like we messed up, sorry :)",
        req.method(),
        &req.uri().path().to_string(),
    ))
}

fn build_success_response<'r, T: Serialize>(
    status: Status,
    payload: T,
    request: &Request,
) -> Response<'r> {
    let body = Json(payload);
    Response::build_from(body.respond_to(request).unwrap())
        .header(ContentType::JSON)
        .status(status)
        .finalize()
}

fn build_failure_response<'r, 's, 't>(
    status: Status,
    message: &'r str,
    request: &'s Request,
) -> Response<'t> {
    let body = Json(ApiError::new(
        status,
        message,
        request.method(),
        &request.uri().path().to_string(),
    ));
    Response::build_from(body.respond_to(request).unwrap())
        .header(ContentType::JSON)
        .status(status)
        .finalize()
}

impl<'r, T: Serialize> Responder<'r, 'static> for ApiResult<T> {
    fn respond_to(self, request: &'r Request<'_>) -> response::Result<'static> {
        match self {
            ApiResult::Success { status, payload } => {
                Ok(build_success_response(status, payload, request))
            }
            ApiResult::Failure { status, message } => {
                Ok(build_failure_response(status, &message, request))
            }
        }
    }
}
