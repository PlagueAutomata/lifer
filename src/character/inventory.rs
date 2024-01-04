use bevy::prelude::*;

#[derive(Component, Reflect)]
pub struct Inventory {
    pub money: u32,
    pub items: f32,
}

impl Inventory {
    pub const MAX_ITEMS: f32 = 20.0;
}
