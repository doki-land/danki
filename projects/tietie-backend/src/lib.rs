#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

use axum::{routing::get, Router};
use axum::http::Response;
use tower_service::Service;
use worker::{Context, Env, HttpRequest};
use worker_macros::event;

pub async fn root() -> &'static str {
    "Hello Axum!"
}

fn router() -> Router {
    Router::new().route("/", get(root))
}

#[event(fetch)]
async fn fetch(req: HttpRequest, _env: Env, _ctx: Context) -> Result<Response<axum::body::Body>, worker::Error> {
    Ok(router().call(req).await?)
}
