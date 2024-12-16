use std::error::Error;

use application::Application;
use configuration::load_config;
use features::streams::pipeline::create_stream_pipeline;

mod application;
mod configuration;
mod errors;
mod features;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let configuration = load_config()?;

    let application = Application::new()
        .with_configuration(&configuration);

    // Handle any one-time setup like pipelines
    if configuration.media.enabled {
        if let Err(err) = create_stream_pipeline() {
            eprintln!("Failed to create pipeline: {}", err);
        }
    }

    let application_name = &application.name;
    let address = format!("{}:{}", configuration.api.local_ip, configuration.api.port);
    let listener = tokio::net::TcpListener::bind(address).await?;
    let local_address = listener.local_addr()?;

    println!("\n~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    println!("{application_name} listening on {local_address}");
    println!("API docs are accessible at {local_address}/docs");

    axum::serve(listener, application.build_router()).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    use crate::{application::Application, configuration::load_config};

    #[tokio::test]
    async fn it_should_create_user() -> Result<(), Box<dyn std::error::Error>> {
        let configuration = load_config()?;

        let router = Application::new()
            .with_configuration(&configuration)
            .build_router();

        // GIVEN
        let new_user_json = r#"{
            "id": "123",
            "username": "jane"
        }"#;

        // WHEN
        let response = router
            .oneshot(
                Request::builder()
                    .uri("/api/users")
                    .method("POST")
                    .header("Content-Type", "application/json")
                    .body(Body::from(new_user_json))
                    .unwrap(),
            )
            .await
            .unwrap();

        // THEN
        assert_eq!(response.status(), StatusCode::CREATED);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(body.as_ref(), b"jane");

        Ok(())
    }
}
