use std::convert::Infallible;

use axum::response::{sse::Event, IntoResponse, Response, Sse};
use futures::stream;
use serde::Serialize;

pub fn once_sse<D: Serialize>(data: &D) -> Response {
    let json = serde_json::to_string(data).unwrap_or_else(|_| "null".to_string());
    let stream = stream::once(async move {
        let event = Event::default().data(json);
        Ok::<Event, Infallible>(event)
    });
    Sse::new(stream).into_response()
}
