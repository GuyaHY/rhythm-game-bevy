use crate::play::note::*;
use crate::ui::window::*;
use bevy::prelude::*;

const LAND_POS_7K: f32 = 0. - 3.5 * NOTE_WIDTH;
const LAND_POS_4K: f32 = 0. - 2.5 * NOTE_WIDTH;
const LANE_LENGTH: f32 = SPAWN_POSITION - HIT_POSITION;

pub enum Lane7K {
    Lane1,
    Lane2,
    Lane3,
    Lane4,
    Lane5,
    Lane6,
    Lane7,
}
impl Lane7K {
    pub fn key_just_pressed(&self, input: &Input<KeyCode>) -> bool {
        match self {
            Lane7K::Lane1 => input.just_pressed(KeyCode::Z),
            Lane7K::Lane2 => input.just_pressed(KeyCode::X),
            Lane7K::Lane3 => input.just_pressed(KeyCode::C),
            Lane7K::Lane4 => input.just_pressed(KeyCode::Space),
            Lane7K::Lane5 => input.just_pressed(KeyCode::M),
            Lane7K::Lane6 => input.just_pressed(KeyCode::Comma),
            Lane7K::Lane7 => input.just_pressed(KeyCode::Period),
        }
    }

    pub fn y(&self) -> f32 {
        match self {
            Lane7K::Lane1 => LAND_POS_7K + NOTE_WIDTH * 1.,
            Lane7K::Lane2 => LAND_POS_7K + NOTE_WIDTH * 2.,
            Lane7K::Lane3 => LAND_POS_7K + NOTE_WIDTH * 3.,
            Lane7K::Lane4 => LAND_POS_7K + NOTE_WIDTH * 4.,
            Lane7K::Lane5 => LAND_POS_7K + NOTE_WIDTH * 5.,
            Lane7K::Lane6 => LAND_POS_7K + NOTE_WIDTH * 6.,
            Lane7K::Lane7 => LAND_POS_7K + NOTE_WIDTH * 7.,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Lane4K {
    Lane1,
    Lane2,
    Lane3,
    Lane4,
}
impl Lane4K {
    pub fn key_just_pressed(&self, input: &Input<KeyCode>) -> bool {
        match self {
            Lane4K::Lane1 => input.just_pressed(KeyCode::Z),
            Lane4K::Lane2 => input.just_pressed(KeyCode::X),
            Lane4K::Lane3 => input.just_pressed(KeyCode::Comma),
            Lane4K::Lane4 => input.just_pressed(KeyCode::Period),
        }
    }

    pub fn x(&self) -> f32 {
        match self {
            Lane4K::Lane1 => LAND_POS_4K + NOTE_WIDTH * 1.,
            Lane4K::Lane2 => LAND_POS_4K + NOTE_WIDTH * 2.,
            Lane4K::Lane3 => LAND_POS_4K + NOTE_WIDTH * 3.,
            Lane4K::Lane4 => LAND_POS_4K + NOTE_WIDTH * 4.,
        }
    }
}

#[derive(Component)]
struct JudgeLine;

pub fn setupJudgeLine(mut commands: Commands, materials: Res<crate::play::note::NoteMaterialRes>) {
    commands
        .spawn(SpriteBundle {
            texture: materials.Key1.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(NOTE_WIDTH, NOTE_HEIGHT)),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(
                LAND_POS_4K + NOTE_WIDTH * 1.,
                HIT_POSITION + NOTE_HEIGHT,
                1.,
            )),
            ..Default::default()
        })
        .insert(JudgeLine);
    commands
        .spawn(SpriteBundle {
            texture: materials.Key2.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(NOTE_WIDTH, NOTE_HEIGHT)),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(
                LAND_POS_4K + NOTE_WIDTH * 2.,
                HIT_POSITION + NOTE_HEIGHT,
                1.,
            )),
            ..Default::default()
        })
        .insert(JudgeLine);
    commands
        .spawn(SpriteBundle {
            texture: materials.Key2.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(NOTE_WIDTH, NOTE_HEIGHT)),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(
                LAND_POS_4K + NOTE_WIDTH * 3.,
                HIT_POSITION + NOTE_HEIGHT,
                1.,
            )),
            ..Default::default()
        })
        .insert(JudgeLine);
    commands
        .spawn(SpriteBundle {
            texture: materials.Key1.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(NOTE_WIDTH, NOTE_HEIGHT)),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(
                LAND_POS_4K + NOTE_WIDTH * 4.,
                HIT_POSITION + NOTE_HEIGHT,
                1.,
            )),
            ..Default::default()
        })
        .insert(JudgeLine);
}
