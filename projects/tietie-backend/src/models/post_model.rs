use super::*;

#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostInfo {
    pub post_id: Uuid,
    pub category_id: Uuid,
    pub link: String,
    pub title: String,
    pub content: String,
    pub create_time: DateTime<Local>,
    pub create_user: Uuid,
    pub update_time: DateTime<Local>,
    pub update_user: Uuid,
    pub is_locked: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostEdit {
    pub post_id: Uuid,
    pub category_id: Option<Uuid>,
    pub link: Option<String>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub create_time: Option<DateTime<Local>>,
    pub create_user: Option<Uuid>,
    pub update_time: Option<DateTime<Local>>,
    pub update_user: Option<Uuid>,
    pub is_locked: Option<bool>,
}

#[server(endpoint = "post/find")]
pub async fn get_post(post_link: String) -> Result<PostInfo, ServerFnError> {
    let post: PostInfo = sqlx::query_as("SELECT * FROM post_info WHERE link = $1 LIMIT 1")
        .bind(post_link)
        .fetch_one(DankiSQL::instance())
        .await?;
    Ok(post)
}