use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tracing::error;

#[derive(Debug, thiserror::Error)]
pub enum ResponseError {
    #[error("internal server error")]
    AxumHttpError(#[from] axum::http::Error),

    #[error("internal server error")]
    DbError(#[from] sqlx::Error),

    #[error("not found")]
    NotFound(),

    #[error("unauthorized")]
    TypedHeaderRejection(#[from] axum_extra::typed_header::TypedHeaderRejection),

    #[error("unauthorized")]
    Unauthorized(),
}

impl IntoResponse for ResponseError {
    fn into_response(self) -> Response {
        if let ResponseError::DbError(err) = &self {
            error!("unexpected database error: {:?}", err);
        }

        let status_code = match self {
            ResponseError::Unauthorized() | ResponseError::TypedHeaderRejection(_) => {
                StatusCode::UNAUTHORIZED
            }
            ResponseError::NotFound() => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        Response::builder()
            .status(status_code)
            .body(Body::from(self.to_string()))
            .expect("Response Builder with known setup should not fail")
            .into_response()
    }
}
