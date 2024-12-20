# Yet Another Bevy Plugin for GStreamer (Under Development)

This project aims to provide a simple but general interface for Bevy to integrate with GStreamer pipelines.


## Design Principles
- Simple and general API for both Bevy and GStreamer developers.
- Focus on creating a good-enough MVP while keeping extensibility in mind.
- Leverage existing libraries to avoid reinventing the wheel.
- Make minimal assumptions and impose few restrictions on user input.
- Aim for reasonable performance.


## Features
- **[Unstable]** Bevy plugin for initialization and setup orchestration.
- **[Unstable]** GStreamer Pipeline as a Bevy Component.
- **[Unstable]** GStreamer AppSink as a Bevy Component.
- **[TODO]** GStreamer AppSrc as a Bevy Component.
- **[WIP]** Raw video and audio support.


## Use Cases
- **Streaming into Bevy:**
  - Cutscenes.
  - In-game displays, such as billboards or background TVs.
- **Streaming out of Bevy:**
  - Recording gameplay.
  - Live streaming, multi-monitor setups, remote play, and more.


## Examples

Run the example:

```shell
cargo run --example simple-sink
```


## Development

This section is under development. Guidelines and contributions will be added soon.


## Credits

This project was inspired by the following:

- [bevy_gstreamer](https://github.com/foxzool/bevy_gstreamer)
- [bevy_gst_video](https://github.com/schizobulia/bevy_gst_video)
