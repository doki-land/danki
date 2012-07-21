use dioxus::prelude::*;


#[server(endpoint = "static_routes")]
pub async fn static_routes() -> Result<Vec<String>, ServerFnError> {
    // The `Routable` trait has a `static_routes` method that returns all static routes in the enum
    Ok(DoraRouter::static_routes().iter().map(ToString::to_string).collect())
}
