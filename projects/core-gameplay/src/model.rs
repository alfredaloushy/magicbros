use bevy_ecs::prelude::*;
use glam::Vec2;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Spell {
    #[default]
    Fireball,
    Gust,
}

/// Input contract shared by desktop, future touch, and future network adapters.
/// Aim is an absolute world position. Edge actions must be submitted for one tick.
#[derive(Component, Clone, Copy, Debug)]
pub struct PlayerCommand {
    pub movement: Vec2,
    pub aim_world: Vec2,
    pub jump: bool,
    pub dash: bool,
    pub cast: bool,
    pub switch: bool,
    pub select: Option<Spell>,
}

impl Default for PlayerCommand {
    fn default() -> Self {
        Self {
            movement: Vec2::ZERO,
            aim_world: Vec2::new(600.0, -200.0),
            jump: false,
            dash: false,
            cast: false,
            switch: false,
            select: None,
        }
    }
}

impl PlayerCommand {
    pub fn sanitized(mut self, fallback_aim: Vec2) -> Self {
        for axis in 0..2 {
            self.movement[axis] = if self.movement[axis].is_finite() {
                self.movement[axis].clamp(-1.0, 1.0)
            } else {
                0.0
            };
        }
        self.aim_world = if self.aim_world.is_finite() {
            self.aim_world
                .clamp(Vec2::splat(-1_000_000.0), Vec2::splat(1_000_000.0))
        } else {
            fallback_aim
        };
        self
    }

    pub fn clear_edges(&mut self) {
        self.jump = false;
        self.dash = false;
        self.switch = false;
        self.select = None;
    }
}

#[derive(Component, Clone, Debug)]
pub struct Actor {
    pub slot: u8,
    pub spawn: Vec2,
    pub health: f32,
    pub respawn_left: f32,
    pub protection: f32,
    pub hit_flash: f32,
    pub health_reveal: f32,
    pub slow_left: f32,
    pub gust_launch_left: f32,
    pub spell: Spell,
    pub cooldown: f32,
    pub aim: Vec2,
    pub jumps: u8,
    pub dash_left: f32,
    pub dash_cooldown: f32,
    pub air_dash_available: bool,
    pub dash_direction: Vec2,
    pub generation: u32,
}

impl Actor {
    pub fn alive(&self) -> bool {
        self.health > 0.0
    }
}

#[derive(Component, Clone, Debug)]
pub struct Body {
    pub position: Vec2,
    pub velocity: Vec2,
    pub impulse: Vec2,
    pub half: Vec2,
    pub grounded: bool,
}

#[derive(Component)]
pub struct Dummy {
    pub firing: bool,
    pub timer: f32,
}

#[derive(Component, Clone, Debug)]
pub struct Projectile {
    pub position: Vec2,
    pub velocity: Vec2,
    pub owner: Entity,
    pub original_caster: Entity,
    pub lifetime: f32,
    pub reflected: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct Platform {
    pub center: Vec2,
    pub half: Vec2,
}

#[derive(Resource, Clone, Debug)]
pub struct Arena {
    pub platforms: Vec<Platform>,
}

impl Default for Arena {
    fn default() -> Self {
        Self {
            platforms: vec![
                Platform {
                    center: Vec2::new(0.0, -280.0),
                    half: Vec2::new(610.0, 20.0),
                },
                Platform {
                    center: Vec2::new(-330.0, -100.0),
                    half: Vec2::new(130.0, 12.0),
                },
                Platform {
                    center: Vec2::new(330.0, -100.0),
                    half: Vec2::new(130.0, 12.0),
                },
                Platform {
                    center: Vec2::new(0.0, 65.0),
                    half: Vec2::new(120.0, 12.0),
                },
            ],
        }
    }
}

#[derive(Clone, Debug)]
pub enum EffectKind {
    Explosion,
    Gust { direction: Vec2 },
}

#[derive(Clone, Debug)]
pub struct Effect {
    pub position: Vec2,
    pub radius: f32,
    pub left: f32,
    pub duration: f32,
    pub kind: EffectKind,
}

#[derive(Resource, Default)]
pub struct Effects(pub Vec<Effect>);

#[derive(Clone, Debug)]
pub struct Hit {
    pub target: Entity,
    pub damage: f32,
    pub impulse: Vec2,
    pub slow_duration: f32,
    /// Replace incoming motion with a forced Gust launch instead of adding force.
    pub launch: bool,
}

#[derive(Resource, Default)]
pub struct PendingHits(pub Vec<Hit>);

/// Bounded diagnostics for playtesting. Not a scoring/attribution system.
#[derive(Resource, Default, Clone, Debug)]
pub struct Diagnostics {
    pub tick: u64,
    pub casts: u64,
    pub hits: u64,
    pub returns: u64,
    pub defeats: u64,
    pub respawns: u64,
}

#[derive(Clone, Debug)]
pub struct ActorView {
    pub entity: Entity,
    pub actor: Actor,
    pub body: Body,
    pub dummy_firing: Option<bool>,
}

#[derive(Clone, Debug)]
pub struct Snapshot {
    pub tuning: crate::tuning::Tuning,
    pub actors: Vec<ActorView>,
    pub projectiles: Vec<Projectile>,
    pub platforms: Vec<Platform>,
    pub effects: Vec<Effect>,
    pub diagnostics: Diagnostics,
}
