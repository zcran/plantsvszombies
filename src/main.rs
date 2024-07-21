use bevy::render::texture::ImageSamplerDescriptor;
use bevy::{prelude::*, window::WindowResolution};
// use bevy_ecs_ldtk::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(ImagePlugin {
                // 像素画放大后仍保证清晰
                default_sampler: ImageSamplerDescriptor::nearest(),
            })
            .set(WindowPlugin {
                //设置窗口大小 1100*750
                primary_window: Some(Window {
                    #[cfg(target_os = "windows")]
                    position: WindowPosition::Centered(MonitorSelection::Primary), //窗口居中
                    resolution: WindowResolution::new(1200.0, 800.0),
                    ..default()
                }),
                ..default()
            }),
    )
    // .add_plugins(LdtkPlugin)
    ;

    // #[cfg(not(target_arch = "wasm32"))]
    // {
    //     app.add_plugins(WeatherPlugin);
    // }

    app
    // .add_state::<AppState>()
    //     .insert_resource(ClearColor(Color::BLACK))
    //     .insert_resource(LevelSelection::index(0))
    //     .insert_resource(CameraState::Following)
    //     .add_event::<CameraShakeEvent>()
    //     .add_systems(Startup, (setup_camera,))
    //     // Start Menu
    //     .add_systems(OnEnter(AppState::StartMenu), (setup_start_menu,))
    //     .add_systems(
    //         Update,
    //         (enter_gaming,).run_if(in_state(AppState::StartMenu)),
    //     )
    //     .add_systems(OnExit(AppState::StartMenu), (cleanup_start_menu,))
    //     // Gaming
    //     .add_systems(OnEnter(AppState::Gaming), (setup_ldtk_world,))
    //     .add_systems(
    //         PreUpdate,
    //         (spawn_ldtk_entity,).run_if(in_state(AppState::Gaming)),
    //     )
    //     .add_systems(
    //         PostUpdate,
    //         (player_state_machine,).run_if(in_state(AppState::Gaming)),
    //     )
        .run();
}
