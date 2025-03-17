#[cfg(feature = "server")]
use sqlx::{postgres::PgPoolOptions, Executor, Pool, Postgres};
#[cfg(feature = "server")]
use dioxus::prelude::ServerFnError;
#[cfg(feature = "server")]
static Danki_DB: std::sync::OnceLock<Pool<Postgres>> = std::sync::OnceLock::new();

#[derive(Default)]
pub struct DankiSQL {
    _private: (),
}
#[cfg(feature = "server")]
impl DankiSQL {
    pub fn instance() -> impl Executor<'static, Database = Postgres> {
        Danki_DB.get().unwrap()
    }

    pub async fn connect(url: &str) -> Result<&'static Self, ServerFnError> {
        let config = PgPoolOptions::new();
        let db = config.connect(url).await?;
        let _ = Danki_DB.set(db);
        Ok(&DankiSQL { _private: () })
    }
}
