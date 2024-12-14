use axum::{http::Error, middleware, response::Response, routing::get_service, Router};
use configuration::load_config;
use tower_http::services::ServeDir;

use crate::features::users::{user_routes, user_service::UserService};

mod configuration;
mod features;
mod stream;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let app_configuration = load_config().unwrap();

    // match create_stream_pipeline() {
    //     Ok(()) => println!("Pipeline created successfully"),
    //     Err(err) => eprintln!("Failed to create pipeline: {}", err),
    // }

    let user_service = UserService::new().await.unwrap();

    let app = Router::new()
        .nest("/api", user_routes::routes(user_service.clone()))
        .layer(middleware::map_response(main_response_mapper))
        .fallback_service(routes_static());

    let address = format!(
        "{}:{}",
        app_configuration.api.local_ip, app_configuration.api.port
    );
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

async fn main_response_mapper(response: Response) -> Response {
    println!("->> {:<12} - main response mapper", "RESPONSE_MAPPER");
    println!();

    response
}
