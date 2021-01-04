# warp-sessions
This repo provides session middleware for the warp framework that:
- Supports async
- Provides plug-and-play functionality for multiple session backends
- Integrates into existing Warp-style filter chains

## how
```rust
use warp_sessions::MemoryStore;

#[tokio::main]
async fn main() {
	let session_store = MemoryStore::new();

	let route = warp::get()
		.and(warp::path!("test"))
		.and(warp_sessions::request::with_session(                                      // 1.
			session_store,
			None,
		))
		.and_then(move |session_with_store: SessionWithStore<MemoryStore>| async move { // 2.
			Ok::<_, Rejection>((
				warp::reply::html("<html></html>".to_string()),
				session_with_store,
			))
		})
		.untuple_one()
		.and_then(warp_sessions::reply::with_session);                                  // 3.
}
```

1. The `warp_sessions::request::with_session(...)` filter is used to bring a session
   into scope of the route handler. This function accepts a type that implements the
   `SessionStore` trait and an optional `CookieOptions` struct to customize the session's
   cookie parameters.
   
2. The `warp_sessions::request::with_session(...)` filter returns a `SessionWithStore<SessionStore>`
   struct. This struct contains the Session struct, a type that implements `SessionStore`, and a
   `CookieOptions` struct. Modifications can be made to the `Session` object within this struct.
   It is returned as part as a tuple at the end of the handler.
   
3. The `warp_sessions::reply::with_session(...)` handler accepts a type that implements a
   `warp::Reply` and a `SessionWithStore` struct. It then handles saving any changes made
   to the session and updating the session ID cookie if necessary.
