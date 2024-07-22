use bevy::{prelude::*, window::WindowResolution};
use bevy_ecs_ldtk::prelude::*;

const WIDTH: f32 = 1200.0;
const HEIGHT: f32 = 800.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())
                                   .set(WindowPlugin {
                                    primary_window: Some(Window {
                                        #[cfg(target_os = "windows")]
                                        position: WindowPosition::Centered(MonitorSelection::Primary), //窗口居中
                                        resolution: WindowResolution::new(WIDTH, HEIGHT),
                                        ..default()
                                    }),
                                    ..default()
                                }))
        .add_plugins(LdtkPlugin)
        .add_systems(Startup, setup)
        .insert_resource(LevelSelection::index(0))  // 初始进入关卡0
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.5;  // 缩放投影。随着尺度的增大，物体的表观尺寸减小，反之亦然。
    camera.transform.translation.x += WIDTH / 5.0; // 相机位置
    camera.transform.translation.y += HEIGHT / 5.0;
    commands.spawn(camera); // 生成相机

    commands.spawn(LdtkWorldBundle {        // 导入ldtk
        ldtk_handle: asset_server.load("pvz1.ldtk.ldtk"),
        ..Default::default()
    });
}
