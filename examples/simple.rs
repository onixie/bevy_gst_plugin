use bevy::{
    asset::RenderAssetUsages,
    log::{Level, LogPlugin},
    prelude::*,
};
use bevy_gst_plugin::*;

struct Example1;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            level: Level::DEBUG,
            filter: "info,bevy_gst_plugin=debug".into(),
            ..default()
        }))
        .add_plugins(GstPlugin::<Example1>::new("videotestsrc ! appsink"))
        .add_systems(Startup, setup)
        .add_systems(Update, (update, control))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
    commands
        .spawn(Node::default())
        .with_child(ImageNode::default());
}

fn control(gst_pipeline: Query<&GstPipeline<Example1>>, keys: Res<ButtonInput<KeyCode>>) -> Result {
    let gst_pipeline = gst_pipeline.get_single()?;
    if keys.just_pressed(KeyCode::Space) {
        gst_pipeline.pause()?;
    }
    if keys.just_pressed(KeyCode::Enter) {
        gst_pipeline.resume()?;
    }
    Ok(())
}

fn update(
    mut image_node: Query<&mut ImageNode>,
    video_sink: Query<&GstAppSink<RawVideo, Example1>>,
    mut images: ResMut<Assets<Image>>,
) -> Result {
    let mut image_node = image_node.get_single_mut()?;
    let video_sink = video_sink.get_single()?;

    if let Ok(image) = video_sink.sample.lock() {
        if let Some(ref image) = *image {
            debug!("received image {:?}", image);
            let old_image = image_node.image.clone_weak();
            image_node.image = images.add(Image::from_dynamic(
                image.clone(),
                true,
                RenderAssetUsages::RENDER_WORLD,
            ));
            images.remove(&old_image);
        }
    }

    Ok(())
}
