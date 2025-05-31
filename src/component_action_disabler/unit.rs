use bevy::prelude::*;

/// A unit component that will cause any ActionStates on the same entity to behave as though they
/// are disabled. This will override a manually set [`ActionState::disabled`].
#[derive(Component, Clone, Debug, PartialEq, Serialize, Deserialize, Reflect)]
pub struct AllActionStatesDisabled;

/// A unit component that will disable a specific ActionState when added to an entity, and reenable
/// it when removed. This will override a manually set [`ActionState::disabled`].
#[derive(Component, Clone, Debug, PartialEq, Serialize, Deserialize, Reflect)]
pub struct ActionStateDisabled<A: Actionlike>;

/// A unit component that will disable a specific Action on a specific ActionState when added to an
/// entity, and reenable it when removed. This will override a manually set [`ActionState::disabled`].
#[derive(Component, Clone, Debug, PartialEq, Serialize, Deserialize, Reflect)]
pub struct ActionsDisabled<A: Actionlike>(Vec<A>);

fn shared_unit_insert_handler<A: Actionlike>(world: &mut DeferredWorld, ctx: HookContext) {
    let (maybe_action_state, maybe_all_action_states_disabled, maybe_action_state_disabled, maybe_actions_disabled)
        = world.query::<(&mut ActionState::<A>, Option<&AllActionStatesDisabled>, Option<&ActionStateDisabled<A>>, Option<&ActionsDisabled::<A>>), With<ActionState::<A>>>().get(ctx.entity);

    // If there's no action state, there's nothing to do
    let Some(action_state) = maybe_action_state else { return };

    if maybe_all_action_states_disabled.is_some() || maybe_action_state_disabled.is_some() {
        action_state.disable();
    }

    if let Some(actions_disabled) = maybe_actions_disabled {
        for action in actions_disabled {
            action_state.disable_action(action);
        }
    }
}

fn shared_unit_remove_handler<A: Actionlike>(world: &mut DeferredWorld, ctx: HookContext) {
    let (maybe_action_state, maybe_all_action_states_disabled, maybe_action_state_disabled, maybe_actions_disabled)
        = world.query::<(&mut ActionState::<A>, Option<&AllActionStatesDisabled>, Option<&ActionStateDisabled<A>>, Option<&ActionsDisabled::<A>>), With<ActionState::<A>>>().get(ctx.entity);

    // If there's no action state, there's nothing to do
    let Some(action_state) = maybe_action_state else { return };

    // Re-enabling logic has to check component IDs because the remove hook triggers before a
    // comopnent is removed, so the Option query will always return Some for the removed component
    if (maybe_all_action_states_disabled.is_none() && world.component_id::<ActionStateDisabled::<A>>() == ctx.component_id)
        || (maybe_action_state_disabled.is_none() && world.component_id::<AllActionStatesDisabled>() == ctx.component_id) {
        action_state.enable();
    }

    if let Some(actions_disabled) = maybe_actions_disabled {
        for action in actions_disabled {
            action_state.enable_action(action);
        }
    }
}

pub fn setup_unit<A: Actionlike>(world: &mut World) {
    world.register_component_hooks::<AllActionStatesDisabled>()
        .on_insert(shared_unit_add_handler)
        .on_remove(shared_unit_remove_handler);

    world.register_component_hooks::<ActionStateDisabled<A>>()
        .on_insert(shared_unit_add_handler)
        .on_remove(shared_unit_remove_handler);

    world.register_component_hooks::<ActionsDisabled<A>>()
        .on_insert(shared_unit_add_handler)
        .on_remove(shared_unit_remove_handler);

    // We also have to register these component hooks for the input manager so that if an input
    // manager is added after a disabling component, the disabling will occur correctly
    world.register_component_hooks::<ActionState<A>>()
        .on_insert(shared_unit_add_handler);
}
