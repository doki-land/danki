#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

use crate::api_getter::HomeStatistics;
pub use crate::errors::AppError;
use poem::{middleware::Cors, web::Json, Addr, Endpoint, EndpointExt, IntoResponse, Route};
use poem::web::{LocalAddr, RemoteAddr};
use poem_openapi::{OpenApi, OpenApiService, Tags};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tower_service::Service;
use worker::{console_log, event, Context, Env, HttpRequest, HttpResponse};

mod api_getter;
mod config;
mod database;
mod errors;
mod models;

pub type Result<T> = std::result::Result<Json<T>, AppError>;

pub struct AppState {
    pg: Pool<Postgres>,
}

impl AppState {
    pub async fn connect() -> AppState {
        let db = match std::env::var("DATABASE_URL") {
            Ok(o) => PgPoolOptions::new().max_connections(5).connect(&o).await.expect("无法连接数据库"),
            Err(_) => panic!("找不到 `PGSQL_URL`"),
        };
        Self { pg: db }
    }
}

#[event(start)]
fn start() {
    console_log!("Example docs are accessible at http://127.0.0.1:8080");
}

#[event(fetch)]
async fn fetch(req: HttpRequest, env: Env, _ctx: Context) -> worker::Result<HttpResponse> {
    console_error_panic_hook::set_once();
    // PrintTracing::enable();
    env.var("")?;
    let db = AppState::connect().await;
    let time = chrono::Utc::now();
    let api_service = OpenApiService::new(db, "ApiEndpoint", time.to_string()).server("http://localhost:8080");

    let json = api_service.spec_endpoint();

    let app = Route::new()
        .nest("/digital-human.json", json)
        .nest("/", api_service)
        .with(
            Cors::new(), /* .allow_origin("*")
                          * .allow_credentials(true)
                          * .max_age(3600)
                          * .allow_methods(vec!["POST", "OPTIONS"])
                          * .allow_headers(vec!["Origin", "Methods", "Content-Type"]) */
        )
        // .with(RequestTracing {})
        ;

    match app.call(poem::Request::from((req, LocalAddr(Addr::Custom()), RemoteAddr(Addr::Custom()), c))).await {
        Ok(o) => Ok(o.into_response()),
        Err(e) => Err(worker::Error::Infallible),
    }
}

#[derive(Tags)]
pub enum AppTags {
    Auth,
}

#[OpenApi]
impl AppState {
    /// 刷新 jwt
    ///
    /// 注意! 要用 refresh_token!!!
    #[oai(path = "/auth/refresh", method = "get", tag = "AppTags::Auth")]
    pub async fn auth_refresh(&self) -> Result<HomeStatistics> {
        Ok(Json(crate::api_getter::HomeStatistics { all_users: 0, all_packages: 0, all_downloads: 0 }))
    }
}

#[test]
fn test() {}
