use crate::error;
use bevy::prelude::*;
use gstreamer as gst;
use gstreamer::prelude::*;
use gstreamer_app as gst_app;
use std::marker::PhantomData;
use std::ops::Deref;

pub type GstPipeline = GstPipelineFor<()>;

#[derive(Component)]
pub struct GstPipelineFor<T> {
    pipeline: gst::Pipeline,
    _phantom: PhantomData<T>,
}

impl<T> Deref for GstPipelineFor<T> {
    type Target = gst::Pipeline;

    fn deref(&self) -> &Self::Target {
        &self.pipeline
    }
}

impl<T> GstPipelineFor<T> {
    pub fn new(pipeline_description: &str) -> Result<Self, error::Error> {
        gst::parse::launch(pipeline_description)
            .map_err(error::Error::Glib)
            .and_then(|pipeline| {
                pipeline
                    .downcast::<gst::Pipeline>()
                    .or(Err(error::Error::CastToPipeline))
            })
            .map(|pipeline| Self {
                pipeline,
                _phantom: PhantomData,
            })
    }

    pub fn resume(&self) -> Result<(), error::Error> {
        self.set_state(gst::State::Playing)
            .map(|_| ())
            .map_err(|_| error::Error::SetPipelineState)
    }

    pub fn pause(&self) -> Result<(), error::Error> {
        self.set_state(gst::State::Paused)
            .map(|_| ())
            .map_err(|_| error::Error::SetPipelineState)
    }

    pub fn restart(&self) -> Result<(), error::Error> {
        self.set_state(gst::State::Null)
            .and(self.set_state(gst::State::Ready))
            .and(self.set_state(gst::State::Playing))
            .map(|_| ())
            .map_err(|_| error::Error::SetPipelineState)
    }

    pub fn iterate_sinks_compatible_with(
        &self,
        supported_caps: gst::Caps,
    ) -> impl Iterator<Item = gst_app::AppSink> {
        self.iterate_sinks()
            .into_iter()
            .flatten()
            .filter_map(move |element| {
                let element_type = element.type_();
                let allowed_caps = element
                    .iterate_sink_pads()
                    .next()
                    .ok()
                    .flatten()
                    .and_then(|pad| pad.allowed_caps());

                if element_type.name() == "GstAppSink"
                    && allowed_caps.is_some_and(|ref caps| supported_caps.can_intersect(caps))
                {
                    element.downcast::<gst_app::AppSink>().ok()
                } else {
                    warn!("skip sink element: {:?}", element.name());
                    None
                }
            })
    }
}

#[test]
fn test_create_gst_pipeline() -> Result<(), error::Error> {
    gst::init().unwrap();

    GstPipeline::new("videotestsrc ! appsink")?;
    GstPipeline::new("audiotestsrc ! appsink")?;
    GstPipeline::new("appsrc ! appsink")?;
    Ok(())
}

#[test]
fn test_deref_gst_pipeline() -> Result<(), error::Error> {
    gst::init().unwrap();

    &GstPipeline::new("videotestsrc ! appsink")? as &gst::Pipeline;
    Ok(())
}
