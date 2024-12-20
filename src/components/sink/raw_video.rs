use crate::traits::*;
use bevy::prelude::*;
use gstreamer as gst;
use gstreamer_video::{self as gst_video, VideoFrameExt};
use image::{DynamicImage, RgbaImage};

pub struct RawVideo;

impl IsVideo for RawVideo {
    const FORMAT: gst_video::VideoFormat = gst_video::VideoFormat::Rgbx;
}

impl Sample for RawVideo {
    type Data = DynamicImage;

    fn supported_caps() -> gst::Caps {
        gst_video::VideoCapsBuilder::new()
            .format(Self::FORMAT)
            .build()
    }

    fn extract_data(sample: gst::Sample) -> Result<Self::Data, gst::FlowError> {
        debug!("get buffer");
        let buffer = sample.buffer().ok_or(gst::FlowError::Error)?;
        debug!("get frame");
        let frame = sample
            .caps()
            .and_then(|caps| gst_video::VideoInfo::from_caps(caps).ok())
            .and_then(|ref info| {
                debug!("{:?}", info);
                gst_video::VideoFrameRef::from_buffer_ref_readable(buffer, info).ok()
            })
            .ok_or(gst::FlowError::Error)?;
        let image = {
            debug!("get raw data");
            let raw_data = frame
                .plane_data(0)
                .map(|data| data.to_vec())
                .map_err(|_| gst::FlowError::Error)?;
            debug!("get image buffer");
            let image_buffer = RgbaImage::from_raw(frame.width(), frame.height(), raw_data)
                .ok_or(gst::FlowError::Error)?;
            debug!("create image");
            DynamicImage::ImageRgba8(image_buffer)
        };
        Ok(image)
    }
}
