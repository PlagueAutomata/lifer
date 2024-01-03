use crate::{
    character::{CharacterController, FindAndMove, Inventory, DEFAULT_COLOR, FARM_COLOR},
    game_state::GameState,
};
use bevy::prelude::*;
use big_brain::prelude::*;

#[derive(Component, Clone)]
pub struct Field;

pub struct FieldPlugin;

impl Plugin for FieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            (
                (farm_action, FindAndMove::<Field>::system).in_set(BigBrainSet::Actions),
                work_need_scorer.in_set(BigBrainSet::Scorers),
            )
                .run_if(in_state(GameState::Playing)),
        );
    }
}

/// Farming 🚜
#[derive(Component, Clone, ActionSpawn)]
pub struct Farm {
    pub until: f32,
    pub per_second: f32,
}

impl Farm {
    pub fn new(until: f32, per_second: f32) -> Self {
        Self { until, per_second }
    }
}

pub fn farm_action(
    time: Res<Time>,
    mut actors: Query<(&mut Inventory, &mut CharacterController)>,
    mut query: Query<(ActionQuery, &Farm)>,
) {
    for (mut action, farm) in &mut query {
        let (mut inventory, mut ctrl) = actors.get_mut(action.actor()).unwrap();

        if action.is_executing() {
            //debug!("Time to farm!");

            trace!("Farming...");
            inventory.items += farm.per_second * time.delta_seconds();
            ctrl.color = FARM_COLOR;

            if inventory.items >= Inventory::MAX_ITEMS {
                debug!("Inventory full!");
                ctrl.color = DEFAULT_COLOR;
                action.success();
            }
        }

        if action.is_cancelled() {
            debug!("Farming was interrupted. Still need to work.");
            ctrl.color = DEFAULT_COLOR;
            action.failure();
        }
    }
}

#[derive(Component, Clone, Default, ScorerSpawn)]
pub struct WorkNeedScorer;

pub fn work_need_scorer(
    actors: Query<&Inventory>,
    mut query: Query<ScorerQuery, With<WorkNeedScorer>>,
) {
    for mut score in &mut query {
        let inventory = actors.get(score.actor());
        let need_more = inventory.map_or(false, |inv| inv.items < Inventory::MAX_ITEMS);
        score.set(if need_more { 0.6 } else { 0.0 });
    }
}
