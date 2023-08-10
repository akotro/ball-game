use bevy::prelude::*;

use super::SimulationState;

// pub fn pause_simulation(mut simulation_state_next_state: ResMut<NextState<SimulationState>>) {
//     simulation_state_next_state.set(SimulationState::Paused);
//     println!("Simulation paused.");
// }

pub fn resume_simulation(mut simulation_state_next_state: ResMut<NextState<SimulationState>>) {
    simulation_state_next_state.set(SimulationState::Running);
    println!("Simulation running.");
}

pub fn toggle_simulation(
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.any_just_pressed([KeyCode::Space, KeyCode::Escape]) {
        match simulation_state.0 {
            SimulationState::Running => {
                next_simulation_state.set(SimulationState::Paused);
                println!("Simulation paused.");
            }
            SimulationState::Paused => {
                next_simulation_state.set(SimulationState::Running);
                println!("Simulation running.");
            }
        }
    }
}
