use axum::{routing::get_service, Router};
use configuration::load_config;
use gstreamer as gst;
use gstreamer::prelude::Cast;
use gstreamer::prelude::ElementExt;
use gstreamer::prelude::ElementExtManual;
use gstreamer::prelude::GstObjectExt;
use tower_http::services::ServeDir;

pub use self::error::Error;

mod configuration;
mod error;
mod web;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let app_configuration = load_config().unwrap();

    match create_stream_pipeline() {
        Ok(()) => println!("Pipeline created successfully"),
        Err(err) => eprintln!("Failed to create pipeline: {}", err),
    }

    let app = Router::new()
        .merge(web::users::routes())
        .fallback_service(routes_static());

    let address = format!(
        "{}:{}",
        app_configuration.api.local_ip, app_configuration.api.port
    );
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

fn create_stream_pipeline() -> Result<(), gst::glib::error::Error> {
    gst::init().unwrap();

    let pipeline = gst::parse::launch(&format!(
        "avfvideosrc ! videoconvert ! queue ! x264enc ! mp4mux ! filesink location=output/recording.mp4"
    ))?
    .downcast::<gst::Pipeline>()
    .expect("type error");

    pipeline
        .set_state(gst::State::Playing)
        .expect("Unable to set the pipeline to the `Playing` state");

    // Wait for a short while to simulate some processing (e.g., 10 seconds)
    std::thread::sleep(std::time::Duration::from_secs(10));

    pipeline.send_event(gst::event::Eos::new());

    for msg in pipeline.bus().unwrap().iter_timed(gst::ClockTime::NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                panic!(
                    "Error from {:?}: {} ({:?})",
                    err.src().map(|s| s.path_string()),
                    err.error(),
                    err.debug()
                );
            }
            _ => (),
        }
    }

    pipeline
        .set_state(gst::State::Null)
        .expect("Unable to set the pipeline to the `Null` state");

    Ok(())
}
