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
		.and(warp_sessions::request::with_session(
			session_store,
			None,
		))
		.and_then(move |sessions_with_store: SessionWithStore<MemoryStore>| async move {
			Ok::<_, Rejection>((
				warp::reply::html("<html></html>".to_string()),
				session_with_store,
			))
		})
		.untuple_one()
		.and_then(warp_sessions::reply::with_session);
}
```
