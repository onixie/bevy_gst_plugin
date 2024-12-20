use bevy::prelude::*;
use gstreamer as gst;
use gstreamer_video as gst_video;

pub trait Sample {
    type Data: Sync + Send + 'static;

    fn supported_caps() -> gst::Caps;
    fn extract_data(sample: gst::Sample) -> Result<Self::Data, gst::FlowError>;
}

pub trait IsVideo: Sample {
    const FORMAT: gst_video::VideoFormat;
}
