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
pub enum CurxmontErrorStatus {
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

impl CurxmontErrorStatus {
    /// Constructs an error status from a numeric code.
    ///
    /// # Arguments
    /// * `code` - The numeric code representing the error status.
    ///
    /// # Returns
    /// * `CurxmontErrorStatus` - The corresponding error status.
    pub fn from_code(code: u16) -> CurxmontErrorStatus {
        match code {
            404 => CurxmontErrorStatus::NotFound,
            403 => CurxmontErrorStatus::Forbidden,
            400 => CurxmontErrorStatus::BadRequest,
            409 => CurxmontErrorStatus::Conflict,
            401 => CurxmontErrorStatus::Unauthorized,
            _ => CurxmontErrorStatus::Unknown,
        }
    }
}

/// The custom error that Actix web automatically converts to a HTTP response.
///
/// # Fields
/// * `message` - The message of the error.
/// * `status` - The status of the error.
#[derive(Serialize, Deserialize, Debug, Error)]
pub struct CruxmontError {
    pub message: String,
    pub status: CurxmontErrorStatus,
}

impl CruxmontError {
    /// Constructs a new error.
    ///
    /// # Arguments
    /// * `message` - The message of the error.
    /// * `status` - The status of the error.
    ///
    /// # Returns
    /// * `CustomError` - The new error.
    pub fn new(message: impl Into<String>, status: CurxmontErrorStatus) -> CruxmontError {
        CruxmontError {
            message: message.into(),
            status,
        }
    }

    /// Constructs a new error with NotFound status.
    ///
    /// # Arguments
    /// * `message` - The message of the error.
    ///
    /// # Returns
    /// * `CruxmontError` - The new error with NotFound status.
    pub fn not_found(message: impl Into<String>) -> CruxmontError {
        CruxmontError {
            message: message.into(),
            status: CurxmontErrorStatus::NotFound,
        }
    }

    /// Constructs a new error with Forbidden status.
    ///
    /// # Arguments
    /// * `message` - The message of the error.
    ///
    /// # Returns
    /// * `CruxmontError` - The new error with Forbidden status.
    pub fn forbidden(message: impl Into<String>) -> CruxmontError {
        CruxmontError {
            message: message.into(),
            status: CurxmontErrorStatus::Forbidden,
        }
    }

    /// Constructs a new error with Unknown status.
    ///
    /// # Arguments
    /// * `message` - The message of the error.
    ///
    /// # Returns
    /// * `CruxmontError` - The new error with Unknown status.
    pub fn unknown(message: impl Into<String>) -> CruxmontError {
        CruxmontError {
            message: message.into(),
            status: CurxmontErrorStatus::Unknown,
        }
    }

    /// Constructs a new error with BadRequest status.
    ///
    /// # Arguments
    /// * `message` - The message of the error.
    ///
    /// # Returns
    /// * `CruxmontError` - The new error with BadRequest status.
    pub fn bad_request(message: impl Into<String>) -> CruxmontError {
        CruxmontError {
            message: message.into(),
            status: CurxmontErrorStatus::BadRequest,
        }
    }

    /// Constructs a new error with Conflict status.
    ///
    /// # Arguments
    /// * `message` - The message of the error.
    ///
    /// # Returns
    /// * `CruxmontError` - The new error with Conflict status.
    pub fn conflict(message: impl Into<String>) -> CruxmontError {
        CruxmontError {
            message: message.into(),
            status: CurxmontErrorStatus::Conflict,
        }
    }

    /// Constructs a new error with Unauthorized status.
    ///
    /// # Arguments
    /// * `message` - The message of the error.
    ///
    /// # Returns
    /// * `CruxmontError` - The new error with Unauthorized status.
    pub fn unauthorized(message: impl Into<String>) -> CruxmontError {
        CruxmontError {
            message: message.into(),
            status: CurxmontErrorStatus::Unauthorized,
        }
    }
}

impl fmt::Display for CruxmontError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl IntoResponse for CruxmontError {
    fn into_response(self) -> AxumResponse {
        let status_code = match self.status {
            CurxmontErrorStatus::NotFound => AxumStatusCode::NOT_FOUND,
            CurxmontErrorStatus::Forbidden => AxumStatusCode::FORBIDDEN,
            CurxmontErrorStatus::Unknown => AxumStatusCode::INTERNAL_SERVER_ERROR,
            CurxmontErrorStatus::BadRequest => AxumStatusCode::BAD_REQUEST,
            CurxmontErrorStatus::Conflict => AxumStatusCode::CONFLICT,
            CurxmontErrorStatus::Unauthorized => AxumStatusCode::UNAUTHORIZED,
        };

        (status_code, Json(self.message)).into_response()
    }
}

impl From<sqlx::Error> for CruxmontError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::RowNotFound => CruxmontError::new(
                "Resource not found".to_string(),
                CurxmontErrorStatus::NotFound,
            ),
            sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23505") => {
                CruxmontError::new(
                    "Duplicate entry".to_string(),
                    CurxmontErrorStatus::Conflict,
                )
            }
            sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23503") => {
                CruxmontError::new(
                    "Foreign key constraint violation".to_string(),
                    CurxmontErrorStatus::BadRequest,
                )
            }
            _ => CruxmontError::new(
                format!("Database error: {}", error),
                CurxmontErrorStatus::Unknown,
            ),
        }
    }
}

impl From<CruxmontError> for u32 {
    fn from(value: CruxmontError) -> Self {
        let outcome = match value.status {
            CurxmontErrorStatus::NotFound => 404,
            CurxmontErrorStatus::Forbidden => 403,
            CurxmontErrorStatus::Unknown => 500,
            CurxmontErrorStatus::BadRequest => 400,
            CurxmontErrorStatus::Conflict => 409,
            CurxmontErrorStatus::Unauthorized => 401,
        };
        outcome as u32
    }
}
