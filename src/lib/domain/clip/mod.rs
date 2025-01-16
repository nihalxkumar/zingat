use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod field;
#[derive(Debug, Error)]
pub enum ClipError {
    #[error("Clip not found")]
    NotFound,

    #[error("Clip already exists")]
    AlreadyExists,

    #[error("Clip has expired")]
    Expired,

    #[error("invalid password: {0}")]
    InvalidPassword(String),

    #[error("invalid title: {0}")]
    InvalidTitle(String),

    #[error("empty content")]
    EmptyContent,

    #[error("invalid date: {0}")]
    InvalidDate(String),

    #[error("date parse error: {0}")]
    DateParse(#[from] chrono::ParseError),

    #[error("id parse error: {0}")]
    Id(#[from] uuid::Error),

    #[error("hits parse error: {0}")]
    Hits(#[from] std::num::TryFromIntError),

}

/// Clip stores all the data about Clips posted to the service.
///
/// Each field in the Clip uses a newtype that encapsulates the requirements
/// for that particular field. If one of the fields cannot be created, then
/// a Clip cannot be created. This enforcement of field creation ensures
/// that a Clip will always be valid whenever it is utilized at any point
/// in the program.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Clip {
    #[serde(skip)]
    pub clip_id: field::ClipId,
    pub shortcode: field::ShortCode,
    pub content: field::Content,
    pub title: field::Title,
    pub posted: field::Posted,
    pub expires: field::Expires,
    pub password: field::Password,
    pub hits: field::Hits,
}
