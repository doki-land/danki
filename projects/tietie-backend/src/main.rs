use clap::Parser;
use danki_orm::{AppError, AppState};
use poem::{middleware::Cors, EndpointExt, Route};
use poem_openapi::OpenApiService;
use tokio::net::TcpListener;

#[derive(Parser)]
pub struct App {}

impl App {
    pub async fn run(self) -> Result<(), AppError> {
        // PrintTracing::enable();

        let db = AppState::connect().await;
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
        poem::Server::new(TcpListener::bind("0.0.0.0")).run(app).await?;
        Ok(())
    }
}

#[tokio::main]
pub async fn main() -> Result<(), AppError> {
    App::parse().run().await
}
