use std::error::Error;
use std::fmt::Display;
use warp::reject::Reject;

/// Error type that converts to a warp::Rejection
#[derive(Debug)]
pub enum SessionError<E> {
    /// Represents an error which occurred while loading a session from
    /// the backing session store.
    LoadError { source: E },

    /// Represents an error that occurred while saving a session to
    /// the backing session store.
    StoreError { source: E },

    /// Represents an error that occurred while destroying a session
    /// record from the backing session store.
    DestroyError { source: E },
}

impl<E: Error + 'static> Error for SessionError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            SessionError::LoadError { ref source } => Some(source),
            SessionError::StoreError { ref source } => Some(source),
            SessionError::DestroyError { ref source } => Some(source),
        }
    }
}

impl<E: Error> Display for SessionError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            SessionError::LoadError { .. } => {
                write!(f, "Failed to load session")
            }
            SessionError::StoreError { .. } => {
                write!(f, "Failed to store session")
            }
            SessionError::DestroyError { .. } => {
                write!(f, "Failed to destroy session")
            }
        }
    }
}

impl<E: Error + Send + Sync + 'static> Reject for SessionError<E> {}
