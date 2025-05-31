use bevy::prelude::*;

use crate::{Serialize, Deserialize, Actionlike, ActionState};

/// While this entity and relationship exists, it will disable any actions on the target entity.
/// This will override a manually set [`ActionState::disabled`] on the target entity.
#[derive(Component, Clone, Debug, PartialEq, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
#[relationship(relationship_target = AllActionStatesDisabledBy)]
pub struct DisablingAllActionStates(Entity);

/// This entity is having its actions disabled by these entities.
/// This will override a manually set [`ActionState::disabled`].
#[derive(Component, Clone, Debug, PartialEq, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
#[relationship_target(relationship = DisablingAllActionStates)]
pub struct AllActionStatesDisabledBy(Vec<Entity>);

/// While this entity and relationship exists, it will disable any actions on the target entity.
/// This will override a manually set [`ActionState::disabled`] on the target entity.
#[derive(Component, Clone, Debug, PartialEq, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
#[relationship(relationship_target = ActionStateDisabledBy)]
pub struct DisablingActionState<A: Actionlike>(Entity);

/// This entity is having its actions disabled by these entities.
/// This will override a manually set [`ActionState::disabled`].
#[derive(Component, Clone, Debug, PartialEq, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
#[relationship_target(relationship = DisablingActionState)]
pub struct ActionStateDisabledBy<A: Actionlike>(Vec<Entity>);

/// While this entity and relationship exists, it will disable any actions on the target entity.
/// This will override a manually set [`ActionState::disabled`] on the target entity.
#[derive(Component, Clone, Debug, PartialEq, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
#[relationship(relationship_target = ActionsDisabledBy)]
pub struct DisablingActions<A: Actionlike> {
    #[relationship]
    to_disable: Entity,
    actions_to_disable: Vec<A>,
}

/// This entity is having its actions disabled by these entities.
/// This will override a manually set [`ActionState::disabled`].
#[derive(Component, Clone, Debug, PartialEq, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
#[relationship_target(relationship = DisablingAction)]
pub struct ActionsDisabledBy<A: Actionlike> {
    disablers: Vec<Entity>,
}
pub fn setup_relationships<A: Actionlike>(world: &mut World) {
    world.register_component_hooks::<AllActionsDisabled>()
}

pub mod prelude {
    pub use super::DisablingAllActionStates;
    pub use super::AllActionStatesDisabledBy;

    pub use super::DisablingActionState;
    pub use super::ActionStateDisabledBy;

    pub use super::DisablingActions;
    pub use super::ActionsDisabledBy;
}
