use bevy::prelude::*;

#[derive(Component)]
pub struct Selectable {}

#[derive(Resource)]
pub struct CurrentlySelected {
    pub sel: Entity,
}

impl Default for CurrentlySelected {
    fn default() -> Self {
        CurrentlySelected {
            sel: Entity::from_raw(u32::MAX),
        }
    }
}
