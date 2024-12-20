# Yet another GStreamer plugin for Bevy game engine (under development)

This project aims to provide a simple but general interface for Bevy to integrate with arbitrary GStreamer pipelines.

## Design Principles
  * Simple and general API for both BEVY and GStreamer developers
  * Good enough MVP with extensibility in mind
  * Leverage existing libraries and avoid reinventing the wheel
  * Minimum assumption and restriction to the user input
  * Reasonable performance

## Features
  * [Unstable] BEVY plugin to orchestrate initialization and setup
  * [Unstable] GStreamer Pipeline as BEVY Component
  * [Unstable] GStreamer AppSink as BEVY Component
  * [TODO] GStreamer AppSrc as BEVY Component
  * [WIP] Raw video/audio support

### Use cases
  * Stream in BEVY
    * Cut scenes
    * In game displays, such as billboards and TVs in the background
  * Stream out BEVY
    * Recording gameplay
    * Live streaming gameplay
    * Multi-monitor, remote play, etc.

## Examples

``` shell
cargo run --example simple-sink
```

## Development

T.B.D

## Credits

Inspired by the following projects:

- [bevy_gstreamer](https://github.com/foxzool/bevy_gstreamer)
- [bevy_gst_video](https://github.com/schizobulia/bevy_gst_video)
