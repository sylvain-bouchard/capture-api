use std::sync::Arc;

use aide::{
    axum::ApiRouter,
    openapi::{OpenApi, Tag},
    transform::TransformOpenApi,
};
use axum::{
    http::StatusCode, middleware, response::Response, routing::get_service, Extension, Json, Router,
};
use tower_http::services::ServeDir;
use uuid::Uuid;

use crate::configuration::AppConfiguration;
use crate::{
    errors::ApplicationError,
    features::users::{user_routes, user_service::UserService},
};

pub struct Application {
    pub name: String,
    pub configuration: Option<AppConfiguration>,
}

impl Application {
    /// Creates a new `Application` instance.
    pub fn new() -> Self {
        Self {
            name: String::from("Capture API"),
            configuration: None,
        }
    }

    pub fn with_configuration(mut self, configuration: &AppConfiguration) -> Self {
        self.configuration = Some(configuration.clone());
        self
    }

    /// Builds the application.
    pub fn build_router(&self) -> Router {
        // Setup error handling for aide (OpenAPI generation)
        aide::gen::on_error(|error| {
            println!("{error}");
        });
        aide::gen::extract_schemas(true);
        let mut api = OpenApi::default();

        let user_service = UserService::new();

        let router = ApiRouter::new()
            .nest_api_service("/api/users", user_routes::routes(user_service.clone()))
            .finish_api_with(&mut api, api_docs)
            .layer(middleware::map_response(main_response_mapper))
            .layer(Extension(Arc::new(api)))
            .fallback_service(routes_static());

        router
    }
}

async fn main_response_mapper(response: Response) -> Response {
    println!("->> {:<12} - main response mapper", "RESPONSE_MAPPER");
    println!();

    response
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
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
