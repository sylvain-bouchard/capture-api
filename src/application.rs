use std::sync::Arc;

use axum::middleware;
use axum::{response::Response, routing::get_service, Router};
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use tower_http::services::ServeDir;

use crate::configuration::AppConfiguration;
use crate::features::users::user_routes;
use crate::features::users::user_service::UserService;
use crate::service::{ServiceProvider, ServiceType};

#[derive(Clone)]
pub struct ApplicationState {
    connection: Option<Arc<DatabaseConnection>>,
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
        self.state
            .service_provider
            .add_service(ServiceType::UserService(UserService::new(
                self.state.connection.clone(),
            )));

        let router = Router::new()
            .nest_service("/api/users", user_routes::routes(self.state.clone()))
            .layer(middleware::map_response(main_response_mapper))
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
