use super::*;

#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserInfo {
    pub user_id: Uuid,
    pub user_name: String,
    pub nick_name: String,
    pub email: String,
    pub password: String,
    pub create_time: DateTime<Local>,
    pub update_time: DateTime<Local>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserEdit {
    pub user_id: Uuid,
    pub user_name: Option<String>,
    pub nick_name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub create_time: Option<DateTime<Local>>,
    pub update_time: Option<DateTime<Local>>,
}

#[server(endpoint = "user/find")]
pub async fn get_user(user_link: String) -> Result<UserInfo, ServerFnError> {
    let mut user: UserInfo = sqlx::query_as("SELECT * FROM user_info WHERE user_link = $1 LIMIT 1")
        .bind(user_link)
        .fetch_one(DankiSQL::instance())
        .await?;
    user.password = "".to_string();
    Ok(user)
}
