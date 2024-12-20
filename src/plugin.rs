use bevy::app::{App, Plugin, Startup};
use bevy::ecs::result::Result;
use bevy::prelude::*;
use gstreamer as gst;
use std::marker::PhantomData;

use crate::{
    pipeline::GstPipelineFor,
    sink::{GstAppSinkFor, RawVideo},
    traits::*,
};

pub type GstPlugin = GstPluginFor<()>;

pub struct GstPluginFor<T> {
    pub pipeline_description: &'static str,
    _phantom: PhantomData<T>,
}

impl<T> GstPluginFor<T> {
    pub fn new(pipeline_description: &'static str) -> Self {
        Self {
            pipeline_description,
            _phantom: PhantomData,
        }
    }
}

impl<T> Plugin for GstPluginFor<T>
where
    T: Send + Sync + 'static,
{
    fn build(&self, app: &mut App) {
        gst::init().expect("failed to initialize GStreamer");
        let pipeline_description = self.pipeline_description.to_string();

        app.add_systems(Startup, move |mut commands: Commands| -> Result {
            let pipeline = GstPipelineFor::<T>::new(&pipeline_description)?;
            let video_sinks = pipeline.iterate_sinks_compatible_with(RawVideo::supported_caps());

            commands.spawn(pipeline).with_children(|parent| {
                for sink in video_sinks {
                    parent.spawn(GstAppSinkFor::<T, RawVideo>::from(sink));
                }
            });

            Ok(())
        });
    }
}
