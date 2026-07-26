use super::command::Command;
use super::event::GameEvent;
use super::snapshot::FrameSnapshot;

/// シミュレーション状態の単一所有者。
#[derive(Debug)]
pub struct World {
    tick: u64,
    paused: bool,
    castle_hp: f32,
    resources: u32,
    wave: u32,
    commands: Vec<Command>,
    events: Vec<GameEvent>,
}

impl World {
    pub fn new() -> Self {
        Self {
            tick: 0,
            paused: false,
            castle_hp: 100.0,
            resources: 100,
            wave: 0,
            commands: Vec::new(),
            events: Vec::new(),
        }
    }

    pub fn push_command(&mut self, command: Command) {
        self.commands.push(command);
    }

    pub fn tick(&mut self, _dt: f32) {
        self.apply_commands();
        if self.paused {
            return;
        }
        self.tick = self.tick.saturating_add(1);
    }

    pub fn take_snapshot(&mut self) -> FrameSnapshot {
        let events = std::mem::take(&mut self.events);
        FrameSnapshot {
            tick: self.tick,
            paused: self.paused,
            castle_hp: self.castle_hp,
            resources: self.resources,
            wave: self.wave,
            towers: Vec::new(),
            enemies: Vec::new(),
            events,
        }
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn current_tick(&self) -> u64 {
        self.tick
    }

    fn apply_commands(&mut self) {
        let commands = std::mem::take(&mut self.commands);
        for command in commands {
            match command {
                Command::TogglePause => {
                    self.paused = !self.paused;
                    self.events
                        .push(GameEvent::PauseChanged { paused: self.paused });
                }
                Command::SetPaused { paused } => {
                    if self.paused != paused {
                        self.paused = paused;
                        self.events.push(GameEvent::PauseChanged { paused });
                    }
                }
            }
        }
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_not_paused_tick_advances() {
        let mut world = World::new();
        world.tick(1.0 / 60.0);
        world.tick(1.0 / 60.0);
        assert_eq!(world.current_tick(), 2);
    }

    #[test]
    fn when_paused_via_command_tick_does_not_advance() {
        let mut world = World::new();
        world.push_command(Command::SetPaused { paused: true });
        world.tick(1.0 / 60.0);
        world.tick(1.0 / 60.0);
        assert!(world.is_paused());
        assert_eq!(world.current_tick(), 0);
    }
}
