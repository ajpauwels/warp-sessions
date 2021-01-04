use thiserror::Error;
use warp::{
    reject::{self, Reject},
    Rejection,
};

/// Error type that converts to a warp::Rejection
#[derive(Error, Debug)]
pub enum SessionError {
    #[error("Failed to load session")]
    LoadError { source: async_session::Error },

    #[error("Failed to store session")]
    StoreError { source: async_session::Error },

    #[error("Failed to destroy session")]
    DestroyError { source: async_session::Error },
}

impl Reject for SessionError {}

impl std::convert::From<SessionError> for Rejection {
    fn from(error: SessionError) -> Self {
        reject::custom(error)
    }
}
