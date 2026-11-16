pub mod client;
pub mod collision;
pub mod model;
pub mod systems;
pub mod tuning;

use bevy_ecs::prelude::*;
use glam::Vec2;
use model::*;
pub use tuning::Tuning;

/// Fixed-step Bevy ECS simulation. No dependency on a window, renderer or transport.
pub struct Simulation {
    pub world: World,
    schedule: Schedule,
    pub local_player: Entity,
    pub dummy: Entity,
}

impl Default for Simulation {
    fn default() -> Self {
        Self::new(Tuning::default())
    }
}

impl Simulation {
    pub fn new(tuning: Tuning) -> Self {
        tuning.validate().expect("invalid gameplay tuning");
        let mut world = World::new();
        world.insert_resource(tuning.clone());
        world.insert_resource(Arena::default());
        world.insert_resource(Effects::default());
        world.insert_resource(PendingHits::default());
        world.insert_resource(Diagnostics::default());
        let local_player = spawn_actor(&mut world, 0, Vec2::new(-220.0, -233.0), &tuning);
        let dummy = spawn_actor(&mut world, 1, Vec2::new(220.0, -233.0), &tuning);
        world.entity_mut(dummy).insert(Dummy {
            firing: false,
            timer: 0.8,
        });
        let mut schedule = Schedule::default();
        schedule.add_systems(
            (
                systems::tick_lifetimes,
                systems::respawn,
                systems::dummy_input,
                systems::apply_input,
                systems::movement,
                systems::cast,
                systems::projectiles,
                systems::damage,
                systems::defeat,
            )
                .chain(),
        );
        Self {
            world,
            schedule,
            local_player,
            dummy,
        }
    }

    pub fn step(&mut self, command: PlayerCommand) {
        let body = self.world.get::<Body>(self.local_player).unwrap();
        let actor = self.world.get::<Actor>(self.local_player).unwrap();
        let command = command.sanitized(body.position + actor.aim);
        *self
            .world
            .get_mut::<PlayerCommand>(self.local_player)
            .unwrap() = command;
        self.schedule.run(&mut self.world);
    }

    pub fn toggle_dummy(&mut self) {
        let mut dummy = self.world.get_mut::<Dummy>(self.dummy).unwrap();
        dummy.firing = !dummy.firing;
        dummy.timer = 0.6;
    }

    pub fn toggle_dummy_spell(&mut self) {
        let mut actor = self.world.get_mut::<Actor>(self.dummy).unwrap();
        actor.spell = match actor.spell {
            Spell::Fireball => Spell::Gust,
            Spell::Gust => Spell::Fireball,
        };
    }

    pub fn snapshot(&mut self) -> Snapshot {
        let mut actors: Vec<_> = self
            .world
            .query::<(Entity, &Actor, &Body, Option<&Dummy>)>()
            .iter(&self.world)
            .map(|(entity, actor, body, dummy)| ActorView {
                entity,
                actor: actor.clone(),
                body: body.clone(),
                dummy_firing: dummy.map(|d| d.firing),
            })
            .collect();
        actors.sort_by_key(|a| a.actor.slot);
        Snapshot {
            tuning: self.world.resource::<Tuning>().clone(),
            actors,
            projectiles: self
                .world
                .query::<&Projectile>()
                .iter(&self.world)
                .cloned()
                .collect(),
            platforms: self.world.resource::<Arena>().platforms.clone(),
            effects: self.world.resource::<Effects>().0.clone(),
            diagnostics: self.world.resource::<Diagnostics>().clone(),
        }
    }
}

fn spawn_actor(world: &mut World, slot: u8, position: Vec2, tuning: &Tuning) -> Entity {
    world
        .spawn((
            Actor {
                slot,
                spawn: position,
                health: tuning.health,
                respawn_left: 0.0,
                protection: tuning.spawn_protection,
                hit_flash: 0.0,
                health_reveal: 0.0,
                slow_left: 0.0,
                gust_launch_left: 0.0,
                spell: Spell::Fireball,
                cooldown: 0.0,
                aim: if slot == 0 { Vec2::X } else { Vec2::NEG_X },
                jumps: 0,
                dash_left: 0.0,
                dash_cooldown: 0.0,
                air_dash_available: true,
                dash_direction: Vec2::X,
                generation: 0,
            },
            Body {
                position,
                velocity: Vec2::ZERO,
                impulse: Vec2::ZERO,
                half: Vec2::new(16.0, 27.0),
                grounded: true,
            },
            PlayerCommand::default(),
        ))
        .id()
}
