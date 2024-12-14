use std::sync::Arc;

use aide::{
    axum::ApiRouter,
    openapi::{OpenApi, Tag},
    transform::TransformOpenApi,
};
use axum::{
    http::{Error, StatusCode},
    middleware,
    response::Response,
    routing::get_service,
    Extension, Json, Router,
};
use configuration::load_config;
use errors::ApplicationError;
use tower_http::services::ServeDir;
use uuid::Uuid;

use crate::features::users::{user_routes, user_service::UserService};

mod configuration;
mod errors;
mod features;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // initialize tracing
    tracing_subscriber::fmt::init();

    aide::gen::on_error(|error| {
        println!("{error}");
    });
    aide::gen::extract_schemas(true);

    let mut api = OpenApi::default();

    let app_configuration = load_config().unwrap();

    // match create_stream_pipeline() {
    //     Ok(()) => println!("Pipeline created successfully"),
    //     Err(err) => eprintln!("Failed to create pipeline: {}", err),
    // }

    let user_service = UserService::new().await.unwrap();

    let app = ApiRouter::new()
        .nest_api_service("/users", user_routes::routes(user_service.clone()))
        .finish_api_with(&mut api, api_docs)
        .layer(middleware::map_response(main_response_mapper))
        .layer(Extension(Arc::new(api)))
        .fallback_service(routes_static());

    let address = format!(
        "{}:{}",
        app_configuration.api.local_ip, app_configuration.api.port
    );
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    let local_address = listener.local_addr().unwrap();

    println!("\n~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    println!("API listening on {local_address}");
    println!("API docs are accessible at {local_address}/docs");

    axum::serve(listener, application).await.unwrap();

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

fn api_docs(api: TransformOpenApi) -> TransformOpenApi {
    api.title("Aide axum Open API")
        .summary("An example Todo application")
        .description(include_str!("README.md"))
        .tag(Tag {
            name: "todo".into(),
            description: Some("Todo Management".into()),
            ..Default::default()
        })
        .security_scheme(
            "ApiKey",
            aide::openapi::SecurityScheme::ApiKey {
                location: aide::openapi::ApiKeyLocation::Header,
                name: "X-Auth-Key".into(),
                description: Some("A key that is ignored.".into()),
                extensions: Default::default(),
            },
        )
        .default_response_with::<Json<ApplicationError>, _>(|res| {
            res.example(ApplicationError {
                message: "some error happened".to_string(),
                details: None,
                id: Uuid::nil(),
                // This is not visible.
                status: StatusCode::IM_A_TEAPOT,
            })
        })
}
