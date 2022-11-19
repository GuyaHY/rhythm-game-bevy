use super::lane::*;
use super::note::*;
use bevy::prelude::*;

#[derive(Resource)]
pub struct NoteProperty {
    pub spawn_time: f32,
    pub last_time: f32,
    pub lane: Lane4K,
}
impl NoteProperty {
    fn new(click_time: f32, last_time: f32, lane: Lane4K) -> Self {
        Self {
            spawn_time: click_time - (HIT_POSITION / DROP_SPEED),
            last_time,
            lane,
        }
    }
}
#[derive(Resource)]
pub struct MapConfig {
    pub notes: Vec<NoteProperty>,
}

pub fn test_map() -> MapConfig {
    MapConfig {
        notes: vec![
            NoteProperty::new(2., 0., Lane4K::Lane1),
            NoteProperty::new(2., 0.5, Lane4K::Lane2),
            NoteProperty::new(2.5, 0., Lane4K::Lane3),
            NoteProperty::new(2.5, 0., Lane4K::Lane4),
            NoteProperty::new(3., 0., Lane4K::Lane4),
            NoteProperty::new(3., 0., Lane4K::Lane3),
        ],
    }
}
