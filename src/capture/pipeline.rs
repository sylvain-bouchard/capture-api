use gstreamer as gst;
use gstreamer::prelude::Cast;
use gstreamer::prelude::ElementExt;
use gstreamer::prelude::ElementExtManual;
use gstreamer::prelude::GstObjectExt;

pub fn create_stream_pipeline() -> Result<(), gst::glib::error::Error> {
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
