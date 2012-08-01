#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

use crate::api_getter::HomeStatistics;
pub use crate::errors::AppError;
use poem::{

     Endpoint, EndpointExt, IntoResponse,
};
use poem_openapi::{OpenApi, Tags};
use poem_openapi::payload::Json;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tower_service::Service;
pub use crate::errors::{Result};
mod api_getter;
mod config;
mod database;
mod errors;
mod models;

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


