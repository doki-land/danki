use super::*;

#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TagInfo {
    pub tag_id: Uuid,
    pub link: String,
    pub name: String,
    pub description: String,
    pub create_time: DateTime<Local>,
    pub create_user: Uuid,
    pub update_time: DateTime<Local>,
    pub update_user: Uuid,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TagEdit {
    pub tag_id: Uuid,
    pub link: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub create_time: Option<DateTime<Local>>,
    pub create_user: Option<Uuid>,
    pub update_time: Option<DateTime<Local>>,
    pub update_user: Option<Uuid>,
}

#[server(endpoint = "tag/find")]
pub async fn get_tag(tag_link: String) -> Result<TagInfo, ServerFnError> {
    let tag: TagInfo = sqlx::query_as("SELECT * FROM tag_info WHERE link = $1 LIMIT 1")
        .bind(tag_link)
        .fetch_one(DankiSQL::instance())
        .await?;
    Ok(tag)
}