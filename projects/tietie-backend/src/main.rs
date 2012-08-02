use clap::Parser;
use tie_tie_space::{AppError, AppState};
use poem::{middleware::Cors, EndpointExt, Route};
use poem::listener::TcpListener;
use poem_openapi::OpenApiService;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};


#[derive(Parser)]
pub struct App {
    #[clap(short, long)]
    database_url: Option<String>,
}

impl App {
    pub async fn startup(self) -> Result<AppState, AppError> {
        Ok(AppState { pg: self.database().await? })
    }
    async fn database(&self) -> Result<Pool<Postgres>, AppError> {
        let url = match self.database_url.as_ref() {
            Some(o) => o.to_string(),
            None => std::env::var("DATABASE_URL")?,
        };
        Ok(PgPoolOptions::new().max_connections(5).connect(&url).await?)
    }
}

impl App {
    pub async fn run(self) -> Result<(), AppError> {
        // PrintTracing::enable();
        let db = self.startup().await?;
        let time = chrono::Utc::now();
        let api_service = OpenApiService::new(db, "ApiEndpoint", time.to_string()).server("http://localhost:8080");

        let json = api_service.spec_endpoint();

        let app = Route::new()
            .nest("/tie-tie.json", json)
            .nest("/", api_service)
            .with(
                Cors::new().allow_origin("*")
                           .allow_credentials(true)
                          .max_age(3600)
                           .allow_methods(vec!["POST", "OPTIONS"])
                          .allow_headers(vec!["Origin", "Methods", "Content-Type"])
            )
            // .with(RequestTracing {})
            ;
        poem::Server::new(TcpListener::bind("0.0.0.0:8080")).run(app).await?;
        Ok(())
    }
}

#[tokio::main]
pub async fn main() -> Result<(), AppError> {
    App::parse().run().await
}
