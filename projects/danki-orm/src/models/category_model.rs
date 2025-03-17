use super::*;

#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CategoryInfo {
    pub category_id: Uuid,
    pub name: String,
    pub description: String,
    pub create_time: DateTime<Local>,
    pub create_user: Uuid,
    pub update_time: DateTime<Local>,
    pub update_user: Uuid,
    pub link: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CategoryEdit {
    pub category_id: Uuid,
    pub name: Option<String>,
    pub description: Option<String>,
    pub create_time: Option<DateTime<Local>>,
    pub create_user: Option<Uuid>,
    pub update_time: Option<DateTime<Local>>,
    pub update_user: Option<Uuid>,
    pub link: Option<String>,
}

#[server(endpoint = "category/find")]
pub async fn get_category(category_link: String) -> Result<CategoryInfo, ServerFnError> {
    let category: CategoryInfo = sqlx::query_as("SELECT * FROM category_info WHERE link = $1 LIMIT 1")
        .bind(category_link)
        .fetch_one(DankiSQL::instance())
        .await?;
    Ok(category)
}
