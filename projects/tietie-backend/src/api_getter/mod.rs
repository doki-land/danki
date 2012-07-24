use aide::axum::IntoApiResponse;
use axum::Json;
use schemars::JsonSchema;
use serde::Serialize;

#[derive(Serialize, JsonSchema)]
pub struct HomeStatistics {
    all_users: u64,
    all_packages: u64,
    all_downloads: u64,
}

pub async fn home_statistics() -> impl IntoApiResponse {
    Json(HomeStatistics { all_users: 1, all_packages: 2, all_downloads: 3 })
}
