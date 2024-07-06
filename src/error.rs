use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub enum Error {
    LoginFail,
    UserNotFound { username: String },
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("--> {:12} - {self:?}", "INTO_RES");

        (StatusCode::INTERNAL_SERVER_ERROR, "Unhandled client").into_response()
    }
}
