use bevy::prelude::*;
use rhythem_game_bevy::{play::note::HIT_TIME, *};

fn main() {
    println!("HIT_TIME: {:?}", HIT_TIME);
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_startup_system(Camera_Setup)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: ui::window::GAME_WIDTH,
                height: ui::window::GAME_HEIGHT,
                title: "Bevy Rhythem".to_string(),
                ..Default::default()
            },
            ..default()
        }))
        .add_plugin(play::note::NotePlugin)
        .run();
}

fn Camera_Setup(mut command: Commands) {
    let config = play::map::test_map();
    command.spawn(Camera2dBundle::default());
    command.insert_resource(config);
}
