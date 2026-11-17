use glam::Vec2;
use serde::{Deserialize, Serialize};
use wild_magic_core::{Tuning, model::*};

pub const VERSION: u32 = 1;
pub const DEFAULT_PORT: u16 = 24560;

#[derive(Debug, Serialize, Deserialize)]
pub enum Packet {
    Hello {
        version: u32,
    },
    Welcome {
        version: u32,
        slot: u8,
    },
    Input {
        sequence: u64,
        generation: u32,
        command: PlayerCommand,
    },
    State(Box<Frame>),
    End {
        reason: String,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerState {
    pub actor: Actor,
    pub body: Body,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ShotState {
    pub id: u64,
    pub position: Vec2,
    pub velocity: Vec2,
    pub owner_slot: u8,
    pub original_slot: u8,
    pub lifetime: f32,
    pub reflected: bool,
}

/// Explicit wire DTO: ECS Entity values never leave the authoritative host.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Frame {
    pub tick: u64,
    pub acknowledged_input: u64,
    pub tuning: Tuning,
    pub players: Vec<PlayerState>,
    pub shots: Vec<ShotState>,
    pub platforms: Vec<Platform>,
    pub effects: Vec<Effect>,
    pub diagnostics: Diagnostics,
}

impl Frame {
    pub fn from_snapshot(snapshot: &Snapshot, acknowledged_input: u64) -> Self {
        let slot = |entity| {
            snapshot
                .actors
                .iter()
                .find(|a| a.entity == entity)
                .expect("projectile owner exists")
                .actor
                .slot
        };
        Self {
            tick: snapshot.diagnostics.tick,
            acknowledged_input,
            tuning: snapshot.tuning.clone(),
            players: snapshot
                .actors
                .iter()
                .map(|v| PlayerState {
                    actor: v.actor.clone(),
                    body: v.body.clone(),
                })
                .collect(),
            shots: snapshot
                .projectiles
                .iter()
                .map(|p| ShotState {
                    id: p.id,
                    position: p.position,
                    velocity: p.velocity,
                    owner_slot: slot(p.owner),
                    original_slot: slot(p.original_caster),
                    lifetime: p.lifetime,
                    reflected: p.reflected,
                })
                .collect(),
            platforms: snapshot.platforms.clone(),
            effects: snapshot.effects.clone(),
            diagnostics: snapshot.diagnostics.clone(),
        }
    }

    pub fn view(&self) -> Snapshot {
        // These are presentation placeholders, not reconstructed ECS entities.
        let placeholder = wild_magic_core::Entity::PLACEHOLDER;
        Snapshot {
            tuning: self.tuning.clone(),
            actors: self
                .players
                .iter()
                .map(|p| ActorView {
                    entity: placeholder,
                    actor: p.actor.clone(),
                    body: p.body.clone(),
                    dummy_firing: None,
                })
                .collect(),
            projectiles: self
                .shots
                .iter()
                .map(|p| Projectile {
                    id: p.id,
                    position: p.position,
                    velocity: p.velocity,
                    owner: placeholder,
                    original_caster: placeholder,
                    lifetime: p.lifetime,
                    reflected: p.reflected,
                })
                .collect(),
            platforms: self.platforms.clone(),
            effects: self.effects.clone(),
            diagnostics: self.diagnostics.clone(),
        }
    }
}
