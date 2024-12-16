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

    // Handle any one-time setup like pipelines
    if configuration.media.enabled {
        if let Err(err) = create_stream_pipeline() {
            eprintln!("Failed to create pipeline: {}", err);
        }
    }

    let application = Application::new()
        .with_configuration(&configuration)
        .build();

    let address = format!("{}:{}", configuration.api.local_ip, configuration.api.port);
    let listener = tokio::net::TcpListener::bind(address).await?;
    let local_address = listener.local_addr()?;

    println!("\n~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    println!("API listening on {local_address}");
    println!("API docs are accessible at {local_address}/docs");

    axum::serve(listener, application).await?;

    Ok(())
}
