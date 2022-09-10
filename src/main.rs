use std::collections::HashMap;

type PlayerId = u64;

#[derive(Default)]
pub struct GameState {
    pub players: HashMap<PlayerId, String>,
    history: Vec<GameEvent>,
}

#[derive(Clone)]
pub enum GameEvent {
    PlayerJoined { player_id: PlayerId, name: String },
}

impl GameState {
    /// Aggregates an event into the GameState.
    /// Note that the event is assumed to be valid when passed to reduce
    pub fn reduce(&mut self, event: &GameEvent) {
        use GameEvent::*;
        match event {
            PlayerJoined { player_id, name } => {
                self.players.insert(*player_id, name.to_string());
            }
        }

        self.history.push(event.clone());
    }

    /// Determines if the event is valid in the current GameState
    pub fn validate(&self, event: &GameEvent) -> bool {
        use GameEvent::*;
        // In this match statement we try our best to invalidate the event
        match event {
            PlayerJoined { player_id, name: _ } => {
                if self.players.contains_key(player_id) {
                    return false;
                }
            }
        }

        // If we can't find something that's wrong
        // with the event then it must be ok
        true
    }

    /// Tries to consume an event by first validating it
    pub fn dispatch(&mut self, event: &GameEvent) -> Result<(), ()> {
        // Its very common to have a "dispatching" function
        // like this do things like validating and logging
        if !self.validate(&event) {
            return Err(());
        }

        self.reduce(event);
        Ok(())
    }
}

fn main() {
    let mut game_state = GameState::default();
    let event = GameEvent::PlayerJoined {
        player_id: 1234,
        name: "Joseph L.".to_string(),
    };

    // This is accepted like before
    game_state.dispatch(&event).unwrap();
}
