use serde::Deserialize;

#[derive(Deserialize)]
pub struct TieTieConfig {
    database_url: String,
}
