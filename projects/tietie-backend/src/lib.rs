#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

pub use crate::errors::AppError;
use aide::{
    axum::{
        routing::{get, post},
        ApiRouter, IntoApiResponse,
    },
    openapi::OpenApi,
};
use axum::{http, Extension, Json};
use sqlx_d1::D1Connection;
use tower_service::Service;
use worker::{console_log, event, Context, Env, HttpRequest};
mod api_getter;
mod errors;
mod models;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: D1Connection,
}

#[event(start)]
fn start() {
    console_log!("Example docs are accessible at http://127.0.0.1:3000/docs");
}

#[event(fetch)]
async fn fetch(req: HttpRequest, env: Env, _ctx: Context) -> worker::Result<http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();
    aide::generate::on_error(|error| {
        println!("{error}");
    });

    aide::generate::extract_schemas(true);

    let d1 = env.d1("TTDB")?;
    let conn = D1Connection::new(d1);

    let state = AppState { db: conn };

    let mut api = OpenApi::default();

    let mut app = ApiRouter::new()
        .api_route("/test", post(api_getter::home_statistics))
        .api_route("/user/new", post(models::new_user))
        .api_route("/user/get", post(models::get_user))
        .route("/tietie.json", get(open_api))
        .finish_api(&mut api)
        .layer(Extension(api))
        .with_state(state);

    Ok(app.call(req).await?)
}

/// Export the generated OpenAPI schema.
pub async fn open_api(Extension(api): Extension<OpenApi>) -> impl IntoApiResponse {
    Json(api)
}

#[test]
fn test() {}
