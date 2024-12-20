pub mod raw_video;
use crate::{error::Error, pipeline::GstPipelineFor, traits::*};
use bevy::prelude::*;
use gstreamer::{self as gst, prelude::*};
use gstreamer_app as gst_app;
pub use raw_video::*;
use std::marker::PhantomData;
use std::ops::Deref;
use std::sync::{Arc, Mutex};

pub type GstAppSink<S> = GstAppSinkFor<(), S>;

#[derive(Component)]
pub struct GstAppSinkFor<T, S>
where
    S: Sample,
{
    sink: gst_app::AppSink,
    pub sample: Arc<Mutex<Option<S::Data>>>,
    _phantom: PhantomData<T>,
}

impl<T, S: Sample> Deref for GstAppSinkFor<T, S> {
    type Target = gst_app::AppSink;

    fn deref(&self) -> &Self::Target {
        &self.sink
    }
}

impl<T, S: Sample> From<gst_app::AppSink> for GstAppSinkFor<T, S> {
    fn from(sink: gst_app::AppSink) -> Self {
        debug!("set sink caps");
        sink.set_caps(Some(&S::supported_caps()));

        debug!("set sink callbacks");
        let sample = Arc::new(Mutex::new(None));
        let image = sample.clone();
        sink.set_callbacks(
            gst_app::AppSinkCallbacks::builder()
                .new_sample(move |sink| {
                    debug!("unlock sample");
                    if let Ok(mut image) = image.lock() {
                        debug!("store sample");
                        *image = Some(
                            sink.pull_sample()
                                .map_err(|_| gst::FlowError::Eos)
                                .and_then(S::extract_data)?,
                        );
                        Ok(gst::FlowSuccess::Ok)
                    } else {
                        Err(gst::FlowError::Error)
                    }
                })
                .build(),
        );

        Self {
            sink,
            sample,
            _phantom: PhantomData,
        }
    }
}

impl<T, S: Sample> GstAppSinkFor<T, S> {
    pub fn new(name: &str, pipeline: &GstPipelineFor<T>) -> Result<Self, Error> {
        let sink = pipeline
            .by_name(name)
            .and_downcast::<gst_app::AppSink>()
            .ok_or(Error::CastToAppSink)?
            .into();

        Ok(sink)
    }
}

#[test]
fn test_create_gst_app_sink_for_video() -> Result<(), Error> {
    gst::init().unwrap();

    let pipeline = GstPipeline::new(&format!(
        "videotestsrc ! appsink name={:?} caps={:?}",
        "appsink0", "video/x-raw"
    ))?;

    GstAppSink::<RawVideo>::new("appsink0", &pipeline)?;

    Ok(())
}
