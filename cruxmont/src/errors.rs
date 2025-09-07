//! Defines a generic error.
//!
//! # Notes
//! This is currently a placeholder but happy to talk about the error handling more
use axum::{
    Json,
    http::StatusCode as AxumStatusCode,
    response::{IntoResponse, Response as AxumResponse},
};
use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// The response error status for usually a HTTP request.
#[derive(Error, Debug, Serialize, Deserialize, PartialEq)]
pub enum NanoServiceErrorStatus {
    #[error("Requested resource was not found")]
    NotFound,
    #[error("You are forbidden to access requested resource.")]
    Forbidden,
    #[error("Unknown Internal Error")]
    Unknown,
    #[error("Bad Request")]
    BadRequest,
    #[error("Conflict")]
    Conflict,
    #[error("Unauthorized")]
    Unauthorized,
}

impl NanoServiceErrorStatus {
    /// Constructs an error status from a numeric code.
    ///
    /// # Arguments
    /// * `code` - The numeric code representing the error status.
    ///
    /// # Returns
    /// * `NanoServiceErrorStatus` - The corresponding error status.
    pub fn from_code(code: u16) -> NanoServiceErrorStatus {
        match code {
            404 => NanoServiceErrorStatus::NotFound,
            403 => NanoServiceErrorStatus::Forbidden,
            400 => NanoServiceErrorStatus::BadRequest,
            409 => NanoServiceErrorStatus::Conflict,
            401 => NanoServiceErrorStatus::Unauthorized,
            _ => NanoServiceErrorStatus::Unknown,
        }
    }
}

/// The custom error that Actix web automatically converts to a HTTP response.
///
/// # Fields
/// * `message` - The message of the error.
/// * `status` - The status of the error.
#[derive(Serialize, Deserialize, Debug, Error)]
pub struct NanoServiceError {
    pub message: String,
    pub status: NanoServiceErrorStatus,
}

impl NanoServiceError {
    /// Constructs a new error.
    ///
    /// # Arguments
    /// * `message` - The message of the error.
    /// * `status` - The status of the error.
    ///
    /// # Returns
    /// * `CustomError` - The new error.
    pub fn new(message: impl Into<String>, status: NanoServiceErrorStatus) -> NanoServiceError {
        NanoServiceError {
            message: message.into(),
            status,
        }
    }
}

impl fmt::Display for NanoServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl IntoResponse for NanoServiceError {
    fn into_response(self) -> AxumResponse {
        let status_code = match self.status {
            NanoServiceErrorStatus::NotFound => AxumStatusCode::NOT_FOUND,
            NanoServiceErrorStatus::Forbidden => AxumStatusCode::FORBIDDEN,
            NanoServiceErrorStatus::Unknown => AxumStatusCode::INTERNAL_SERVER_ERROR,
            NanoServiceErrorStatus::BadRequest => AxumStatusCode::BAD_REQUEST,
            NanoServiceErrorStatus::Conflict => AxumStatusCode::CONFLICT,
            NanoServiceErrorStatus::Unauthorized => AxumStatusCode::UNAUTHORIZED,
        };

        (status_code, Json(self.message)).into_response()
    }
}

impl From<sqlx::Error> for NanoServiceError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::RowNotFound => NanoServiceError::new(
                "Resource not found".to_string(),
                NanoServiceErrorStatus::NotFound,
            ),
            sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23505") => {
                NanoServiceError::new(
                    "Duplicate entry".to_string(),
                    NanoServiceErrorStatus::Conflict,
                )
            }
            sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23503") => {
                NanoServiceError::new(
                    "Foreign key constraint violation".to_string(),
                    NanoServiceErrorStatus::BadRequest,
                )
            }
            _ => NanoServiceError::new(
                format!("Database error: {}", error),
                NanoServiceErrorStatus::Unknown,
            ),
        }
    }
}

impl From<NanoServiceError> for u32 {
    fn from(value: NanoServiceError) -> Self {
        let outcome = match value.status {
            NanoServiceErrorStatus::NotFound => 404,
            NanoServiceErrorStatus::Forbidden => 403,
            NanoServiceErrorStatus::Unknown => 500,
            NanoServiceErrorStatus::BadRequest => 400,
            NanoServiceErrorStatus::Conflict => 409,
            NanoServiceErrorStatus::Unauthorized => 401,
        };
        outcome as u32
    }
}
