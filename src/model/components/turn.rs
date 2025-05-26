use bevy::prelude::*;
use std::collections::VecDeque;

use crate::model::types::GameAction;

// Define components for the turn system
#[derive(Component, Debug)]
pub struct TurnActor {
    pub speed: u64,
    pub alive: bool,
    pub next_turn_time: u64,
    pub actions: VecDeque<Box<dyn GameAction>>,
}

impl TurnActor {
    pub fn new(speed: u64) -> Self {
        Self { speed, alive: true, next_turn_time: 0, actions: VecDeque::new() }
    }

    // Add this method
    pub fn queue_action(&mut self, action: impl GameAction) -> &mut Self {
        self.actions.push_back(Box::new(action));
        self
    }

    pub fn add_action(&mut self, action: impl GameAction) {
        self.actions.push_back(Box::new(action));
    }

    pub fn next_action(&mut self) -> Option<Box<dyn GameAction>> { self.actions.pop_front() }

    pub fn peak_next_action(&self) -> Option<&dyn GameAction> {
        self.actions.front().map(Box::as_ref)
    }

    pub fn is_alive(&self) -> bool { self.alive }
}

#[derive(Component)]
pub struct WaitingForInput;
