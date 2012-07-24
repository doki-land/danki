use super::*;

#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SectionInfo {
    pub section_id: Uuid,
    pub name: String,
    pub description: String,
    pub create_time: DateTime<Local>,
    pub create_user: Uuid,
    pub update_time: DateTime<Local>,
    pub update_user: Uuid,
    pub categories: Vec<Uuid>,
    pub order: Option<i32>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SectionEdit {
    pub section_id: Uuid,
    pub name: Option<String>,
    pub description: Option<String>,
    pub create_time: Option<DateTime<Local>>,
    pub create_user: Option<Uuid>,
    pub update_time: Option<DateTime<Local>>,
    pub update_user: Option<Uuid>,
    pub categories: Option<Vec<Uuid>>,
    pub order: Option<i32>,
}

#[server(endpoint = "section/list")]
pub async fn get_sections() -> Result<Vec<SectionInfo>, ServerFnError> {
    let section =
        sqlx::query_as("SELECT * FROM section_info ORDER BY 'order' LIMIT 100").fetch_all(DankiSQL::instance()).await?;
    Ok(section)
}
