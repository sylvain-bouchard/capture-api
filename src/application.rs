use std::sync::Arc;

use aide::axum::ApiRouter;
use aide::openapi::OpenApi;
use aide::{openapi::Tag, transform::TransformOpenApi};
use axum::{http::StatusCode, response::Response, routing::get_service, Json, Router};
use axum::{middleware, Extension};
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use tower_http::services::ServeDir;
use uuid::Uuid;

use crate::configuration::AppConfiguration;
use crate::errors::ApplicationError;
use crate::features::users::user_routes;
use crate::features::users::user_service::UserService;
use crate::service::{ServiceProvider, ServiceType};

#[derive(Clone)]
pub struct ApplicationState {
    pub connection: Option<Arc<DatabaseConnection>>,
    pub service_provider: Arc<ServiceProvider>,
}

pub struct Application {
    pub name: String,
    configuration: AppConfiguration,
    state: ApplicationState,
}

impl Application {
    pub fn new(configuration: &AppConfiguration) -> Self {
        Self {
            name: String::from("Capture API"),
            configuration: configuration.clone(),
            state: ApplicationState {
                connection: None,
                service_provider: Arc::new(ServiceProvider::new()),
            },
        }
    }

    /// Initializes the database connection and populates the application state
    pub async fn initialize_state(mut self) -> Result<Self, Box<dyn std::error::Error>> {
        if self.configuration.datasource.enabled {
            let database_uri = self.configuration.datasource.get_connection_string();
            let connection = Database::connect(&database_uri).await?;

            // Apply pending migrations
            Migrator::up(&connection, None).await?;

            self.state = ApplicationState {
                connection: Some(Arc::new(connection)),
                service_provider: Arc::new(ServiceProvider::new()),
            };
        }

        Ok(self)
    }

    /// Builds the application.
    pub fn build_router(&self) -> Router {
        // Setup error handling for aide (OpenAPI generation)
        aide::gen::on_error(|error| {
            println!("{error}");
        });
        aide::gen::extract_schemas(true);
        let mut api = OpenApi::default();

        self.state
            .service_provider
            .add_service(ServiceType::UserService(UserService::new()));

        let router = ApiRouter::new()
            .nest_api_service("/api/users", user_routes::routes(self.state.clone()))
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
