use bevy::{asset::RenderAssetUsages, prelude::*};
use bevy_gst_plugin::plugin;
use gstreamer as gst;

fn main() {
    gst::init().unwrap();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(plugin::GStreamerAppPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
    commands.spawn((
        ImageNode::default(),
        plugin::GStreamerApp::new("videotestsrc").unwrap(),
    ));
}

fn update(
    mut query: Query<(&plugin::GStreamerApp, &mut ImageNode)>,
    mut images: ResMut<Assets<Image>>,
) {
    if let Ok((gstreamer_app, mut image_node)) = query.get_single_mut() {
        if let Ok(image) = gstreamer_app.image.lock() {
            if let Some(ref image) = *image {
                warn!("receive image {}x{}", image.width(), image.height());
                let old_image = image_node.image.clone_weak();
                image_node.image = images.add(Image::from_dynamic(
                    image.clone(),
                    true,
                    RenderAssetUsages::RENDER_WORLD,
                ));
                images.remove(&old_image);
            }
        }
    }
}
