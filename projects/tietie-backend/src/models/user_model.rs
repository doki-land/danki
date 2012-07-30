use crate::{AppError, AppState};
use aide::{axum::IntoApiResponse, OperationOutput};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use chrono::{DateTime, Local};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow};
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Deserialize, JsonSchema)]
pub struct UserQueryByLink {
    link: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, JsonSchema, FromRow)]
pub struct UserInfo {
    pub user_id: Uuid,
    pub user_name: String,
    pub nick_name: String,
    pub email: String,
    pub password: String,
    pub create_time: DateTime<Local>,
    pub update_time: DateTime<Local>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, JsonSchema, FromRow)]
pub struct UserEdit {
    #[serde(default)]
    pub user_id: Uuid,
    pub user_name: Option<String>,
    pub nick_name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

pub async fn new_user(state: State<AppState>, json: Json<UserEdit>) -> Result<Json<UserInfo>, AppError> {
    let user_id = Uuid::now_v7();
    let user_name = user_id.to_string();
    let user: UserInfo = sqlx::query_as(
        r#"
    INSERT INTO user_info (user_id, user_name, nick_name, email, password, create_time, update_time)
    VALUES ($1,$2,$3,coalesce($4,''),$5,current_timestamp, current_timestamp)
    RETURNING *
    "#,
    )
    .bind(user_id)
    .bind(json.0.user_name.unwrap_or(user_name.to_string()))
    .bind(json.0.nick_name.unwrap_or(user_name.to_string()))
    .bind(json.0.email)
    .bind(json.0.password)
    .fetch_one(&state.db)
    .await?;
    Ok(Json(user))
}

pub async fn get_user(state: State<AppState>, json: Json<UserQueryByLink>) -> Result<Json<UserInfo>, AppError> {
    let mut user: UserInfo =
        sqlx::query_as("SELECT * FROM user_info WHERE user_link = $1 LIMIT 1").bind(json.0.link).fetch_one(&state.db).await?;
    user.password = "".to_string();
    Ok(Json(user))
}
