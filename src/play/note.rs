use crate::play::lane::*;
use crate::play::map::*;
use crate::ui::window::*;
use bevy::prelude::*;

pub const NOTE_WIDTH: f32 = 80.;
pub const NOTE_HEIGHT: f32 = 30.;
pub const SPAWN_POSITION: f32 = GAME_HEIGHT / 2.;
pub const HIT_POSITION: f32 = -GAME_HEIGHT / 2.;
pub const DROP_SPEED: f32 = 200.;
pub const HIT_TIME: f32 = (SPAWN_POSITION - HIT_POSITION) / DROP_SPEED;
const GREAT_JUDGE: f32 = 0.025;
const GOOD_JUDGE: f32 = 0.100;
const BAD_JUDGE: f32 = 0.250;

#[derive(Default, Resource)]
pub struct NoteMaterialRes {
    pub Key1: Handle<Image>,
    pub Key2: Handle<Image>,
    pub Key3: Handle<Image>,
    pub Hold: Handle<Image>,
    pub Tail1: Handle<Image>,
    pub Tail2: Handle<Image>,
    pub Tail3: Handle<Image>,
}

#[derive(Resource)]
struct SpawnTimer(Timer);

#[derive(Component)]
struct Note {
    lane: Lane4K,
    last_time: f32,
    time_alive: f32
}

pub struct NotePlugin;
impl Plugin for NotePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NoteMaterialRes>()
            .add_startup_system(NoteMaterialSetup)
            .add_system(spawn_note)
            .add_system(move_note)
            .add_system(setupJudgeLine)
            .add_system(despawn_note);
    }
}

fn NoteMaterialSetup(mut command: Commands, asset_server: Res<AssetServer>) {
    command.insert_resource(NoteMaterialRes {
        Key1: asset_server.load("skin/Default/mania-note1.png"),
        Key2: asset_server.load("skin/Default/mania-note2.png"),
        Key3: asset_server.load("skin/Default/mania-noteS.png"),
        Hold: asset_server.load("skin/Default/NoteHoldBody.png"),
        Tail1: asset_server.load("skin/Default/mania-note1.png"),
        Tail2: asset_server.load("skin/Default/mania-note2.png"),
        Tail3: asset_server.load("skin/Default/mania-noteS.png"),
    });
}

fn spawn_note(
    mut command: Commands,
    materials: Res<NoteMaterialRes>,
    time: Res<Time>,
    mut map_config: ResMut<MapConfig>,
) {
    let secs = time.elapsed_seconds()-3.;
    let mut remove_counter = 0;

    for note in &map_config.notes {
        if note.spawn_time < secs {
            remove_counter += 1;
            //Generate Note
            let transform =
                Transform::from_translation(Vec3::new(note.lane.x(), SPAWN_POSITION, 1.));
            command
                .spawn(SpriteBundle {
                    texture: match note.lane {
                        Lane4K::Lane1 => materials.Key1.clone(),
                        Lane4K::Lane2 => materials.Key2.clone(),
                        Lane4K::Lane3 => materials.Key2.clone(),
                        Lane4K::Lane4 => materials.Key1.clone(),
                    },
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(NOTE_WIDTH, NOTE_HEIGHT)),
                        ..Default::default()
                    },
                    transform,
                    ..Default::default()
                })
                .insert(Note { lane: note.lane, last_time: 0., time_alive: 0. });


                //Generate Long Note Body
                if note.last_time > 0. {
                    let transform =
                    Transform::from_translation(Vec3::new(note.lane.x(), SPAWN_POSITION + note.last_time * DROP_SPEED / 2. , 0.));
                    command
                    .spawn(SpriteBundle {
                        texture: materials.Hold.clone(),
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(NOTE_WIDTH, note.last_time * DROP_SPEED)),
                            ..Default::default()
                        },
                        transform,
                        ..Default::default()
                    })
                    .insert(Note { lane: note.lane, last_time: note.last_time, time_alive: 0. });


                    //Generate Long Note Tail
                    let transform =
                    Transform::from_translation(Vec3::new(note.lane.x(), SPAWN_POSITION + note.last_time * DROP_SPEED, 1.));
                command
                    .spawn(SpriteBundle {
                        texture: match note.lane {
                            Lane4K::Lane1 => materials.Tail1.clone(),
                            Lane4K::Lane2 => materials.Tail2.clone(),
                            Lane4K::Lane3 => materials.Tail2.clone(),
                            Lane4K::Lane4 => materials.Tail1.clone(),
                        },
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(NOTE_WIDTH, NOTE_HEIGHT)),
                            ..Default::default()
                        },
                        transform,
                        ..Default::default()
                    })
                    .insert(Note { lane: note.lane, last_time: 0., time_alive: 0. });
                }

                


        } else {
            break;
        }
    }

    for _ in 0..remove_counter {
        map_config.notes.remove(0);
    }
}

fn move_note(time: Res<Time>, mut query: Query<(&mut Transform, &mut Note)>) {
    for (mut transform, mut note) in query.iter_mut() {
        transform.translation.y -= time.delta_seconds() * DROP_SPEED;
        note.time_alive += time.delta_seconds();
        // println!("{:?}", note.time_alive);
    }
}

fn despawn_note(mut commands: Commands, query: Query<(Entity, &Note)>, key_input: Res<Input<KeyCode>>) {
    for (entity, note) in query.iter() {
        let timing = note.time_alive - HIT_TIME;
        if (timing < 0.400) && (timing > - 0.400) {
            if (timing < GREAT_JUDGE) && (timing > - GREAT_JUDGE) && note.lane.key_just_pressed(&key_input) {
                println!("Great! Hit Time: {:?}, timing: {:?}", note.time_alive, timing);
                commands.entity(entity).despawn();
            } else if (timing < GOOD_JUDGE) && (timing > - GOOD_JUDGE) && note.lane.key_just_pressed(&key_input) {
                println!("Good! Hit Time: {:?}, timing: {:?}", note.time_alive, timing);
                commands.entity(entity).despawn();
            } else if (timing < BAD_JUDGE) && (timing > - BAD_JUDGE) && note.lane.key_just_pressed(&key_input) {
                println!("BAD! Hit Time: {:?}, timing: {:?}", note.time_alive, timing);
                commands.entity(entity).despawn();
            }
            else if (timing > 0.250) {
                println!("Poor");
                commands.entity(entity).despawn();
            }
        }
        
    }
}