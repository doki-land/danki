use crate::{AppError, AppState};
use chrono::{DateTime, Local};
use poem_openapi::{payload::Json, Object};
use semantic_id::{Semantic64, SemanticKey};
use sqlx::FromRow;
use std::fmt::Display;

pub struct UserKey;

pub type UserId = Semantic64<UserKey>;

impl SemanticKey for UserKey {
    const KEY: &'static str = "user";
}

#[derive(Clone, Debug, PartialEq, Object)]
pub struct UserQueryByLink {
    user_name: String,
}
#[derive(Clone, Debug, PartialEq, Object)]
pub struct LoginByPassword {
    user_name: String,
    password: String,
}

#[derive(Clone, Debug, PartialEq, Object, FromRow)]
pub struct UserInfo {
    pub user_id: UserId,
    pub user_name: String,
    pub nick_name: String,
    pub email: String,
    pub password: String,
    pub create_time: DateTime<Local>,
    pub update_time: DateTime<Local>,
}

#[derive(Clone, Debug, PartialEq, Object)]
pub struct UserCreate {
    pub user_name: String,
    pub nick_name: String,
    pub password: String,
}

#[derive(Clone, Debug, PartialEq, Object)]
pub struct UserEdit {
    pub user_id: UserId,
    pub user_name: Option<String>,
    pub nick_name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

impl UserCreate {
    pub async fn create(self, api: &AppState) -> Result<UserId, AppError> {
        let user_id = UserId::now();
        let _ = sqlx::query("SELECT user_create($1,$2,$3,$4)")
            .bind(user_id)
            .bind(self.user_name)
            .bind(self.nick_name)
            .bind(self.password)
            .execute(&api.pg)
            .await?;
        Ok(user_id)
    }
}


impl UserQueryByLink {
    pub async fn query(self, api: &AppState) -> Result<UserInfo, AppError> {
        let mut user: UserInfo = sqlx::query_as("SELECT * FROM user_info WHERE user_name = $1 LIMIT 1")
            .bind(self.user_name)
            .fetch_one(&api.pg)
            .await?;
        user.password.clear();
        Ok(user)
    }
}

pub async fn login_by_password(api: &AppState, json: Json<LoginByPassword>) -> Result<UserInfo, AppError> {
    let mut user: UserInfo = sqlx::query_as("SELECT * FROM user_info WHERE user_name = $1 LIMIT 1")
        .bind(json.0.user_name)
        .fetch_one(&api.pg)
        .await?;
    if user.password.eq(&json.0.password) {
        user.password.clear();
        Ok(user)
    }
    else {
        Err(AppError::DatabaseError { message: "X".to_string() })
    }
}
