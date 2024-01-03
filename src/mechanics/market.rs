use crate::{
    character::{FindAndMove, Inventory},
    game_state::GameState,
};
use bevy::prelude::*;
use big_brain::prelude::*;

#[derive(Component, Clone)]
pub struct Market;

pub struct MarketPlugin;

impl Plugin for MarketPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            (
                (sell_action, FindAndMove::<Market>::system).in_set(BigBrainSet::Actions),
                sell_need_scorer.in_set(BigBrainSet::Scorers),
            )
                .run_if(in_state(GameState::Playing)),
        );
    }
}

/// Selling 💰
#[derive(Component, Clone, ActionSpawn)]
pub struct Sell;

pub fn sell_action(mut actors: Query<&mut Inventory>, mut query: Query<ActionQuery, With<Sell>>) {
    for mut action in &mut query {
        let mut inventory = actors.get_mut(action.actor()).unwrap();

        if action.is_executing() {
            inventory.money += inventory.items as u32;
            inventory.items = 0.0;
            debug!("Sold! Money: {}", inventory.money);
            action.success();
        }

        if action.is_cancelled() {
            debug!("Selling was interrupted. Still need to work.");
            action.failure();
        }
    }
}

#[derive(Component, Clone, Default, ScorerSpawn)]
pub struct SellNeedScorer;

pub fn sell_need_scorer(
    actors: Query<&Inventory>,
    mut query: Query<ScorerQuery, With<SellNeedScorer>>,
) {
    for mut score in &mut query {
        let inventory = actors.get(score.actor());
        let has_enough = inventory.map_or(false, |inv| inv.items >= Inventory::MAX_ITEMS);
        score.set(if has_enough { 0.6 } else { 0.0 });
    }
}
