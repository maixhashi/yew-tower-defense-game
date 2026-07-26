use std::collections::HashMap;

use super::command::Command;
use super::entity::{Enemy, EntityId, Tower};
use super::event::GameEvent;
use super::snapshot::{EnemySnap, FrameSnapshot, TowerSnap};

/// シミュレーション状態の単一所有者。
#[derive(Debug)]
pub struct World {
    tick: u64,
    paused: bool,
    castle_hp: f32,
    resources: u32,
    wave: u32,
    next_id: EntityId,
    towers: HashMap<EntityId, Tower>,
    enemies: HashMap<EntityId, Enemy>,
    commands: Vec<Command>,
    events: Vec<GameEvent>,
    debug_spawned: bool,
}

impl World {
    pub fn new() -> Self {
        Self {
            tick: 0,
            paused: false,
            castle_hp: 100.0,
            resources: 100,
            wave: 0,
            next_id: 1,
            towers: HashMap::new(),
            enemies: HashMap::new(),
            commands: Vec::new(),
            events: Vec::new(),
            debug_spawned: false,
        }
    }

    pub fn push_command(&mut self, command: Command) {
        self.commands.push(command);
    }

    pub fn tick(&mut self, dt: f32) {
        self.apply_commands();
        if !self.debug_spawned {
            self.spawn_debug_enemies();
            self.debug_spawned = true;
        }
        if self.paused {
            return;
        }
        self.update_enemies(dt);
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
            towers: self
                .towers
                .values()
                .map(|t| TowerSnap {
                    id: t.id,
                    type_id: t.type_id.clone(),
                    visual_key: t.visual_key.clone(),
                    x: t.x,
                    y: t.y,
                    z: t.z,
                })
                .collect(),
            enemies: self
                .enemies
                .values()
                .map(|e| EnemySnap {
                    id: e.id,
                    type_id: e.type_id.clone(),
                    visual_key: e.visual_key.clone(),
                    x: e.x,
                    y: e.y,
                    z: e.z,
                    hp: e.hp,
                })
                .collect(),
            events,
        }
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn current_tick(&self) -> u64 {
        self.tick
    }

    fn alloc_id(&mut self) -> EntityId {
        let id = self.next_id;
        self.next_id = self.next_id.saturating_add(1);
        id
    }

    fn spawn_debug_enemies(&mut self) {
        for i in 0..3 {
            let id = self.alloc_id();
            self.enemies.insert(
                id,
                Enemy {
                    id,
                    type_id: "debug_grunt".into(),
                    visual_key: "enemy_box".into(),
                    x: -10.0 + i as f32 * 2.0,
                    y: 0.6,
                    z: -12.0,
                    hp: 10.0,
                    speed: 1.2 + i as f32 * 0.15,
                    phase: i as f32,
                },
            );
        }
    }

    /// Update Method: 敵ごとの簡易移動（円弧デモ）。
    fn update_enemies(&mut self, dt: f32) {
        for enemy in self.enemies.values_mut() {
            enemy.phase += dt * enemy.speed;
            let radius = 11.0;
            enemy.x = radius * enemy.phase.cos();
            enemy.z = radius * enemy.phase.sin();
            enemy.y = 0.6;
        }
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

    #[test]
    fn when_ticked_debug_enemies_are_spawned_and_move() {
        let mut world = World::new();
        world.tick(0.1);
        assert_eq!(world.enemies.len(), 3);
        let before = world.enemies.values().next().unwrap().x;
        world.tick(0.5);
        let after = world.enemies.values().next().unwrap().x;
        assert_ne!(before, after);
    }
}
