use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};

/// Provisional values inherited from the Godot experiment, in world units/seconds.
#[derive(Resource, Clone, Debug, Serialize, Deserialize)]
pub struct Tuning {
    pub tick: f32,
    pub run_speed: f32,
    pub acceleration: f32,
    pub gravity: f32,
    pub jump_speed: f32,
    pub dash_speed: f32,
    pub dash_time: f32,
    pub dash_cooldown: f32,
    pub health: f32,
    pub fire_damage: f32,
    pub fire_speed: f32,
    pub fire_lifetime: f32,
    pub fire_radius: f32,
    pub fire_cooldown: f32,
    pub fire_push: f32,
    pub self_damage_scale: f32,
    pub gust_radius: f32,
    pub gust_push: f32,
    pub gust_lift: f32,
    pub gust_launch_time: f32,
    pub gust_cooldown: f32,
    pub gust_slow_duration: f32,
    pub gust_slow_speed_multiplier: f32,
    pub respawn_time: f32,
    pub spawn_protection: f32,
    pub dummy_fire_interval: f32,
}

impl Tuning {
    pub fn validate(&self) -> Result<(), &'static str> {
        if !self.tick.is_finite() || self.tick <= 0.0 {
            return Err("tick must be finite and positive");
        }
        if !self.health.is_finite() || self.health <= 0.0 {
            return Err("health must be finite and positive");
        }
        if !self.respawn_time.is_finite()
            || self.respawn_time <= 0.0
            || !self.dummy_fire_interval.is_finite()
            || self.dummy_fire_interval <= 0.0
        {
            return Err("respawn and dummy fire intervals must be finite and positive");
        }
        if !self.self_damage_scale.is_finite() || !(0.0..=1.0).contains(&self.self_damage_scale) {
            return Err("self damage scale must be between zero and one");
        }
        if !self.gust_slow_speed_multiplier.is_finite()
            || !(0.0..=1.0).contains(&self.gust_slow_speed_multiplier)
        {
            return Err("Gust slow speed multiplier must be between zero and one");
        }
        let nonnegative = [
            self.run_speed,
            self.acceleration,
            self.gravity,
            self.jump_speed,
            self.dash_speed,
            self.dash_time,
            self.dash_cooldown,
            self.fire_damage,
            self.fire_speed,
            self.fire_lifetime,
            self.fire_radius,
            self.fire_cooldown,
            self.fire_push,
            self.gust_radius,
            self.gust_push,
            self.gust_lift,
            self.gust_launch_time,
            self.gust_cooldown,
            self.gust_slow_duration,
            self.spawn_protection,
        ];
        if nonnegative.iter().any(|v| !v.is_finite() || *v < 0.0) {
            return Err("gameplay values must be finite and nonnegative");
        }
        Ok(())
    }
}

impl Default for Tuning {
    fn default() -> Self {
        Self {
            tick: 1.0 / 120.0,
            run_speed: 330.0,
            acceleration: 2300.0,
            gravity: 1700.0,
            jump_speed: 650.0,
            dash_speed: 780.0,
            dash_time: 0.14,
            dash_cooldown: 0.70,
            health: 100.0,
            fire_damage: 25.0,
            fire_speed: 650.0,
            fire_lifetime: 2.5,
            fire_radius: 82.0,
            fire_cooldown: 0.42,
            fire_push: 185.0,
            self_damage_scale: 0.35,
            gust_radius: 190.0,
            gust_push: 850.0,
            gust_lift: 500.0,
            gust_launch_time: 0.20,
            gust_cooldown: 0.65,
            gust_slow_duration: 1.0,
            gust_slow_speed_multiplier: 0.70,
            respawn_time: 2.0,
            spawn_protection: 1.2,
            dummy_fire_interval: 1.5,
        }
    }
}
