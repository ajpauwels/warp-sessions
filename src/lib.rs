pub mod cookie;
pub mod reply;
pub mod request;

pub use async_session::{MemoryStore, Session, SessionStore};
use cookie::CookieOptions;
use thiserror::Error;
use warp::{
    reject::{self, Reject},
    Rejection, Reply,
};

// Error type that converts to a warp::Rejection
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

// SessionWithStore binds a session object with its backing store and some cookie options
// This is passed around by routes wanting to do things with a session
pub struct SessionWithStore<S: SessionStore> {
    pub session: Session,
    pub session_store: S,
    pub cookie_options: CookieOptions,
}

// WithSession is a warp::Reply that attaches a session ID in the form of
// a Set-Cookie header to an existing warp::Reply
pub struct WithSession<T: Reply> {
    reply: T,
    cookie_options: CookieOptions,
}

impl<T> WithSession<T>
where
    T: Reply,
{
    pub async fn new<S: SessionStore>(
        reply: T,
        session_with_store: SessionWithStore<S>,
    ) -> Result<WithSession<T>, Rejection> {
        let mut cookie_options = session_with_store.cookie_options;

        if session_with_store.session.is_destroyed() {
            cookie_options.cookie_value = Some("".to_string());
            cookie_options.max_age = Some(0);

            session_with_store
                .session_store
                .destroy_session(session_with_store.session)
                .await
                .map_err(|source| SessionError::DestroyError { source })?;
        } else {
            if session_with_store.session.data_changed() {
                match session_with_store
                    .session_store
                    .store_session(session_with_store.session)
                    .await
                    .map_err(|source| SessionError::StoreError { source })?
                {
                    Some(sid) => cookie_options.cookie_value = Some(sid),
                    None => (),
                }
            }
        }

        Ok(WithSession {
            reply,
            cookie_options,
        })
    }
}

impl<T> Reply for WithSession<T>
where
    T: Reply,
{
    fn into_response(self) -> warp::reply::Response {
        if let Some(_) = self.cookie_options.cookie_value {
            warp::reply::with_header(self.reply, "Set-Cookie", self.cookie_options.to_string())
                .into_response()
        } else {
            self.reply.into_response()
        }
    }
}

#[cfg(test)]
mod test {
    use super::WithSession;
    use crate::{cookie::CookieOptions, SessionWithStore};
    use async_session::{MemoryStore, Session};

    #[tokio::test]
    async fn test_session_reply_with_no_data_changed() {
        let html_reply = warp::reply::html("".to_string());
        let session = Session::new();
        let session_store = MemoryStore::new();
        let cookie_options = CookieOptions::default();
        let session_with_store = SessionWithStore {
            session,
            session_store,
            cookie_options,
        };

        assert_eq!(session_with_store.session.data_changed(), false);
        WithSession::new(html_reply, session_with_store)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_session_reply_with_data_changed() {
        let html_reply = warp::reply::html("".to_string());
        let mut session = Session::new();
        session.insert("key", "value").unwrap();
        let session_store = MemoryStore::new();
        let cookie_options = CookieOptions::default();
        let session_with_store = SessionWithStore {
            session,
            session_store,
            cookie_options,
        };

        assert_eq!(session_with_store.session.data_changed(), true);
        WithSession::new(html_reply, session_with_store)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_session_reply_with_session_destroyed() {
        let html_reply = warp::reply::html("".to_string());
        let mut session = Session::new();
        session.destroy();
        let session_store = MemoryStore::new();
        let cookie_options = CookieOptions::default();
        let session_with_store = SessionWithStore {
            session,
            session_store,
            cookie_options,
        };

        assert_eq!(session_with_store.session.is_destroyed(), true);
        WithSession::new(html_reply, session_with_store)
            .await
            .unwrap();
    }
}
