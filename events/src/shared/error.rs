use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use axum_macros::FromRequest;
use serde::Serialize;

#[derive(Debug)]
pub enum RouteError {
    InvalidSignature,
    InvalidMessageType,
    BadRequest,
}

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(RouteError))]
struct AppJson<T>(T);

impl<T> IntoResponse for AppJson<T>
where
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}

impl IntoResponse for RouteError {
    fn into_response(self) -> Response {
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }

        let (status, message) = match &self {
            RouteError::InvalidSignature => (StatusCode::FORBIDDEN, "Unable to Validate Signature"),
            RouteError::InvalidMessageType => (StatusCode::BAD_REQUEST, "Unsupported Message Type"),
            RouteError::BadRequest => (StatusCode::BAD_REQUEST, "Bad Request"),
        };
        (
            status,
            AppJson(ErrorResponse {
                message: message.to_string(),
            }),
        )
            .into_response()
    }
}
