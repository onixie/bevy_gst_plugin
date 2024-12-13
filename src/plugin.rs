use bevy::app::{App, Plugin};
use bevy::prelude::*;
use gstreamer::prelude::*;
use gstreamer::{self as gst, FlowSuccess};
use gstreamer_app as gst_app;
use gstreamer_video::{self as gst_video, VideoFrameExt};
use image::{DynamicImage, RgbaImage};
use std::sync::{Arc, Mutex};

pub struct GStreamerAppPlugin;
impl Plugin for GStreamerAppPlugin {
    fn build(&self, _app: &mut App) {}
}

#[derive(Component)]
pub struct GStreamerApp {
    pipeline: gst::Pipeline,
    sink: gst_app::AppSink,
    pub image: Arc<Mutex<Option<DynamicImage>>>,
}

#[derive(Debug)]
pub enum Error {
    Glib(gst::glib::Error),
    CastPipeline,
    CastAppSink,
    SetPipelineState,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

const APP_SINK_NAME: &str = "app_sink";

impl GStreamerApp {
    pub fn new(pipeline_description: &str) -> Result<Self, Error> {
        let pipeline = gst::parse::launch(&format!(
            "{pipeline_description} ! appsink name={APP_SINK_NAME}"
        ))
        .map_err(Error::Glib)
        .and_then(|pipeline| {
            pipeline
                .downcast::<gst::Pipeline>()
                .or(Err(Error::CastPipeline))
        })?;

        let sink = pipeline
            .by_name(APP_SINK_NAME)
            .and_downcast::<gst_app::AppSink>()
            .ok_or(Error::CastAppSink)?;

        sink.set_caps(Some(
            &gst_video::VideoCapsBuilder::new()
                .format(gst_video::VideoFormat::Rgbx)
                .build(),
        ));

        debug!("set sink callbacks");
        let image = Arc::new(Mutex::new(None));
        let image_sample = image.clone();
        sink.set_callbacks(
            gst_app::AppSinkCallbacks::builder()
                .new_sample(move |sink| {
                    debug!("get sample");
                    let sample = sink.pull_sample().map_err(|_| gst::FlowError::Eos)?;
                    debug!("get buffer");
                    let buffer = sample.buffer().ok_or_else(|| gst::FlowError::Error)?;
                    debug!("get frame");
                    let frame = sample
                        .caps()
                        .and_then(|caps| gst_video::VideoInfo::from_caps(caps).ok())
                        .and_then(|ref info| {
                            gst_video::VideoFrameRef::from_buffer_ref_readable(buffer, info).ok()
                        })
                        .ok_or_else(|| gst::FlowError::Error)?;
                    debug!("get image");
                    if let Ok(mut image_sample) = image_sample.lock() {
                        debug!("create image {}", frame.format());
                        let image = {
                            let image_buffer = RgbaImage::from_raw(
                                frame.width(),
                                frame.height(),
                                frame.plane_data(0).map(|data| data.to_vec()).unwrap(),
                            )
                            .ok_or_else(|| gst::FlowError::Error)?;

                            DynamicImage::ImageRgba8(image_buffer)
                        };

                        debug!("store image");
                        *image_sample = Some(image);

                        Ok(gst::FlowSuccess::Ok)
                    } else {
                        Err(gst::FlowError::Error)
                    }
                })
                .build(),
        );

        pipeline
            .set_state(gst::State::Playing)
            .map_err(|_| Error::SetPipelineState)?;

        Ok(GStreamerApp {
            pipeline,
            sink,
            image,
        })
    }
}

#[test]
fn test_gstreamer_app_new() -> Result<(), Error> {
    gst::init().unwrap();

    GStreamerApp::new("videotestsrc").map(|_| ())
}
