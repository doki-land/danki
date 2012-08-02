#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

pub use crate::errors::{AppError, Result};
use crate::models::{UserCreate, UserId};
use poem::{Endpoint, EndpointExt, IntoResponse};
use poem_openapi::{payload::Json, OpenApi, Tags};
use sqlx::{Pool, Postgres};

mod api_getter;
mod database;
mod errors;
mod models;

pub struct AppState {
    pub pg: Pool<Postgres>,
}

#[derive(Tags)]
pub enum AppTags {
    Auth,
}

#[OpenApi]
impl AppState {
    /// Register a new user
    #[oai(path = "/user/create", method = "get", tag = "AppTags::Auth")]
    pub async fn user_create(&self, json: Json<UserCreate>) -> Result<UserId> {
        Ok(Json(json.0.create(self).await?))
    }
}
