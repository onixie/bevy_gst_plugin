use gstreamer::glib;

#[derive(Debug)]
pub enum Error {
    Glib(glib::Error),
    CastToPipeline,
    CastToAppSink,
    SetPipelineState,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}
