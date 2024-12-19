use bevy::app::{App, Plugin, Startup};
use bevy::ecs::result::Result;
use bevy::prelude::*;
use gstreamer as gst;
use gstreamer_video as gst_video;
use std::marker::PhantomData;

use crate::{pipeline::GstPipeline, sink::GstAppSink, traits::*};

pub struct GstPlugin<T = ()> {
    pub pipeline_description: &'static str,
    _phantom: PhantomData<T>,
}

impl<T> GstPlugin<T> {
    pub fn new(pipeline_description: &'static str) -> Self {
        Self {
            pipeline_description,
            _phantom: PhantomData,
        }
    }
}

impl<T> Plugin for GstPlugin<T>
where
    T: Send + Sync + 'static,
{
    fn build(&self, app: &mut App) {
        gst::init().expect("failed to initialize GStreamer");
        let pipeline_description = self.pipeline_description.to_string();

        app.add_systems(Startup, move |mut commands: Commands| -> Result {
            let pipeline = GstPipeline::<T>::new(&pipeline_description)?;
            let video_sinks = pipeline.iterate_sinks_compatible_with(
                gst_video::VideoCapsBuilder::new()
                    .format(RawVideo::FORMAT)
                    .build(),
            );

            commands.spawn(pipeline).with_children(|parent| {
                for sink in video_sinks {
                    parent.spawn(GstAppSink::<RawVideo, T>::from(sink));
                }
            });

            Ok(())
        });
    }
}
