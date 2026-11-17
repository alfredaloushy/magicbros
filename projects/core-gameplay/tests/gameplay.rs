use bevy_ecs::prelude::*;
use glam::Vec2;
use wild_magic_core::{
    Simulation, Tuning,
    collision::{move_body, segment_box},
    model::*,
};

fn ready() -> Simulation {
    let mut sim = Simulation::default();
    for mut actor in sim.world.query::<&mut Actor>().iter_mut(&mut sim.world) {
        actor.protection = 0.0;
    }
    sim
}

fn ticks(sim: &mut Simulation, count: usize) {
    for _ in 0..count {
        sim.step(PlayerCommand::default());
    }
}

#[test]
fn actors_settle_on_floor_without_sinking() {
    let mut sim = ready();
    ticks(&mut sim, 600);
    for view in sim.snapshot().actors {
        assert!((view.body.position.y + 233.0).abs() < 0.01);
        assert!(view.body.grounded);
    }
}

#[test]
fn movement_accelerates_to_run_speed() {
    let mut sim = ready();
    for _ in 0..30 {
        sim.step(PlayerCommand {
            movement: Vec2::X,
            ..Default::default()
        });
    }
    let body = sim.world.get::<Body>(sim.local_player).unwrap();
    assert!((body.velocity.x - 330.0).abs() < 0.01);
    assert!(body.position.x > -180.0);
}

#[test]
fn double_jump_allows_two_jumps_and_rejects_third() {
    let mut sim = ready();
    let jump = PlayerCommand {
        jump: true,
        ..Default::default()
    };
    sim.step(jump);
    ticks(&mut sim, 5);
    sim.step(jump);
    let second_velocity = sim.world.get::<Body>(sim.local_player).unwrap().velocity.y;
    assert_eq!(sim.world.get::<Actor>(sim.local_player).unwrap().jumps, 2);
    sim.step(jump);
    assert!(sim.world.get::<Body>(sim.local_player).unwrap().velocity.y < second_velocity);
    assert_eq!(sim.world.get::<Actor>(sim.local_player).unwrap().jumps, 2);
}

#[test]
fn landing_restores_both_jumps() {
    let mut sim = ready();
    sim.step(PlayerCommand {
        jump: true,
        ..Default::default()
    });
    ticks(&mut sim, 200);
    assert!(sim.world.get::<Body>(sim.local_player).unwrap().grounded);
    assert_eq!(sim.world.get::<Actor>(sim.local_player).unwrap().jumps, 0);
}

#[test]
fn diagonal_dash_normalizes_direction_and_respects_cooldown() {
    let mut sim = ready();
    sim.step(PlayerCommand {
        movement: Vec2::ONE,
        dash: true,
        ..Default::default()
    });
    let body = sim.world.get::<Body>(sim.local_player).unwrap();
    assert!((body.velocity.length() - 780.0).abs() < 0.01);
    let first = sim
        .world
        .get::<Actor>(sim.local_player)
        .unwrap()
        .dash_direction;
    sim.step(PlayerCommand {
        movement: Vec2::NEG_X,
        dash: true,
        ..Default::default()
    });
    assert_eq!(
        sim.world
            .get::<Actor>(sim.local_player)
            .unwrap()
            .dash_direction,
        first
    );
}

fn airborne() -> Simulation {
    let mut sim = ready();
    sim.world.resource_mut::<Tuning>().gravity = 0.0;
    sim.world.resource_mut::<Arena>().platforms.clear();
    let mut body = sim.world.get_mut::<Body>(sim.local_player).unwrap();
    body.position = Vec2::ZERO;
    body.grounded = false;
    sim
}

#[test]
fn airborne_dash_does_not_recharge_when_cooldown_expires() {
    let mut sim = airborne();
    sim.step(PlayerCommand {
        dash: true,
        movement: Vec2::X,
        ..Default::default()
    });
    ticks(&mut sim, 100);
    let actor = sim.world.get::<Actor>(sim.local_player).unwrap();
    assert_eq!(actor.dash_cooldown, 0.0);
    assert!(!actor.air_dash_available);
    sim.step(PlayerCommand {
        dash: true,
        movement: Vec2::NEG_X,
        ..Default::default()
    });
    let actor = sim.world.get::<Actor>(sim.local_player).unwrap();
    assert_eq!(actor.dash_left, 0.0);
    assert_eq!(actor.dash_cooldown, 0.0);
    assert_eq!(actor.dash_direction, Vec2::X);
}

#[test]
fn landing_on_a_platform_restores_air_dash() {
    let mut sim = ready();
    sim.world
        .get_mut::<Actor>(sim.local_player)
        .unwrap()
        .air_dash_available = false;
    {
        let mut body = sim.world.get_mut::<Body>(sim.local_player).unwrap();
        body.position = Vec2::new(-330.0, -30.0);
        body.grounded = false;
    }
    ticks(&mut sim, 40);
    assert!(sim.world.get::<Body>(sim.local_player).unwrap().grounded);
    assert!((sim.world.get::<Body>(sim.local_player).unwrap().position.y + 61.0).abs() < 0.01);
    assert!(
        sim.world
            .get::<Actor>(sim.local_player)
            .unwrap()
            .air_dash_available
    );
    sim.step(PlayerCommand {
        dash: true,
        movement: Vec2::Y,
        ..Default::default()
    });
    assert!(sim.world.get::<Actor>(sim.local_player).unwrap().dash_left > 0.0);
    assert!(
        !sim.world
            .get::<Actor>(sim.local_player)
            .unwrap()
            .air_dash_available
    );
}

#[test]
fn landing_recharges_air_dash_without_resetting_the_cooldown() {
    let mut sim = ready();
    sim.step(PlayerCommand {
        dash: true,
        movement: Vec2::NEG_Y,
        ..Default::default()
    });
    let actor = sim.world.get::<Actor>(sim.local_player).unwrap();
    assert!(actor.air_dash_available); // Downward dash touches the floor immediately.
    assert!(actor.dash_cooldown > 0.0);
    let old_direction = actor.dash_direction;
    sim.step(PlayerCommand {
        dash: true,
        movement: Vec2::X,
        ..Default::default()
    });
    assert_eq!(
        sim.world
            .get::<Actor>(sim.local_player)
            .unwrap()
            .dash_direction,
        old_direction
    );
}

#[test]
fn ground_dash_can_repeat_after_cooldown() {
    let mut sim = ready();
    sim.step(PlayerCommand {
        dash: true,
        movement: Vec2::X,
        ..Default::default()
    });
    ticks(&mut sim, 100);
    assert!(sim.world.get::<Body>(sim.local_player).unwrap().grounded);
    sim.step(PlayerCommand {
        dash: true,
        movement: Vec2::NEG_X,
        ..Default::default()
    });
    assert_eq!(
        sim.world
            .get::<Actor>(sim.local_player)
            .unwrap()
            .dash_direction,
        Vec2::NEG_X
    );
    assert!(sim.world.get::<Actor>(sim.local_player).unwrap().dash_left > 0.0);
}

#[test]
fn horizontal_ground_dash_keeps_ground_contact_and_air_charge() {
    let mut sim = ready();
    sim.step(PlayerCommand {
        dash: true,
        movement: Vec2::X,
        ..Default::default()
    });
    for _ in 0..8 {
        assert!(sim.world.get::<Body>(sim.local_player).unwrap().grounded);
        assert!(
            sim.world
                .get::<Actor>(sim.local_player)
                .unwrap()
                .air_dash_available
        );
        sim.step(PlayerCommand::default());
    }
}

#[test]
fn an_upward_ground_dash_cannot_chain_into_another_air_dash() {
    let mut sim = ready();
    sim.world.resource_mut::<Tuning>().gravity = 0.0;
    sim.step(PlayerCommand {
        dash: true,
        movement: Vec2::Y,
        ..Default::default()
    });
    ticks(&mut sim, 100);
    assert!(!sim.world.get::<Body>(sim.local_player).unwrap().grounded);
    sim.step(PlayerCommand {
        dash: true,
        movement: Vec2::X,
        ..Default::default()
    });
    assert_eq!(
        sim.world.get::<Actor>(sim.local_player).unwrap().dash_left,
        0.0
    );
    assert_eq!(
        sim.world
            .get::<Actor>(sim.local_player)
            .unwrap()
            .dash_direction,
        Vec2::Y
    );
}

#[test]
fn wall_and_ceiling_contact_do_not_recharge_air_dash() {
    for (center, half, direction) in [
        (Vec2::new(50.0, 0.0), Vec2::new(5.0, 100.0), Vec2::X),
        (Vec2::new(0.0, 50.0), Vec2::new(100.0, 5.0), Vec2::Y),
    ] {
        let mut sim = airborne();
        sim.world
            .resource_mut::<Arena>()
            .platforms
            .push(Platform { center, half });
        sim.step(PlayerCommand {
            dash: true,
            movement: direction,
            ..Default::default()
        });
        ticks(&mut sim, 100);
        assert!(!sim.world.get::<Body>(sim.local_player).unwrap().grounded);
        assert!(
            !sim.world
                .get::<Actor>(sim.local_player)
                .unwrap()
                .air_dash_available
        );
        sim.step(PlayerCommand {
            dash: true,
            movement: -direction,
            ..Default::default()
        });
        assert_eq!(
            sim.world.get::<Actor>(sim.local_player).unwrap().dash_left,
            0.0
        );
    }
}

#[test]
fn respawn_restores_spent_air_dash() {
    let mut sim = ready();
    sim.world
        .get_mut::<Actor>(sim.local_player)
        .unwrap()
        .air_dash_available = false;
    sim.world.resource_mut::<PendingHits>().0.push(Hit {
        target: sim.local_player,
        damage: 100.0,
        impulse: Vec2::ZERO,
        slow_duration: 0.0,
        launch: false,
    });
    sim.step(PlayerCommand::default());
    ticks(&mut sim, 245);
    let actor = sim.world.get::<Actor>(sim.local_player).unwrap();
    assert_eq!(actor.generation, 1);
    assert!(actor.air_dash_available);
}

#[test]
fn swept_body_collision_stops_extreme_speed_on_thin_platform() {
    let mut body = Body {
        position: Vec2::new(0.0, 100.0),
        velocity: Vec2::new(0.0, -50_000.0),
        impulse: Vec2::ZERO,
        half: Vec2::new(16.0, 27.0),
        grounded: false,
    };
    move_body(
        &mut body,
        &[Platform {
            center: Vec2::ZERO,
            half: Vec2::new(100.0, 2.0),
        }],
        1.0 / 60.0,
    );
    assert!((body.position.y - 29.0).abs() < 0.01);
    assert!(body.grounded);
    assert_eq!(body.velocity.y, 0.0);
}

#[test]
fn segment_collision_handles_parallel_and_inside_segments() {
    assert_eq!(
        segment_box(Vec2::ZERO, Vec2::X, Vec2::ZERO, Vec2::ONE),
        Some(0.0)
    );
    assert!(
        segment_box(
            Vec2::new(3.0, 0.0),
            Vec2::new(3.0, 100.0),
            Vec2::ZERO,
            Vec2::ONE
        )
        .is_none()
    );
    assert_eq!(
        segment_box(
            Vec2::new(-10.0, 0.0),
            Vec2::new(10.0, 0.0),
            Vec2::ZERO,
            Vec2::ONE
        ),
        Some(0.45)
    );
}

#[test]
fn fireball_hits_enemy_and_disappears() {
    let mut sim = ready();
    sim.step(PlayerCommand {
        aim_world: Vec2::new(220.0, -233.0),
        cast: true,
        ..Default::default()
    });
    ticks(&mut sim, 90);
    assert_eq!(sim.world.get::<Actor>(sim.dummy).unwrap().health, 75.0);
    assert!(sim.snapshot().projectiles.is_empty());
    assert_eq!(sim.snapshot().diagnostics.hits, 1);
}

#[test]
fn fireball_sweeps_first_surface_before_enemy() {
    let mut sim = ready();
    sim.world.resource_mut::<Arena>().platforms.push(Platform {
        center: Vec2::new(0.0, -220.0),
        half: Vec2::new(2.0, 60.0),
    });
    sim.world.resource_mut::<Tuning>().fire_speed = 100_000.0;
    sim.step(PlayerCommand {
        aim_world: Vec2::new(220.0, -233.0),
        cast: true,
        ..Default::default()
    });
    assert!(sim.snapshot().projectiles.is_empty());
    assert_eq!(sim.world.get::<Actor>(sim.dummy).unwrap().health, 100.0);
    assert!(sim.snapshot().effects.iter().any(|e| e.position.x < 0.0));
}

#[test]
fn own_explosion_deals_reduced_damage() {
    let mut sim = ready();
    sim.step(PlayerCommand {
        aim_world: Vec2::new(-220.0, -400.0),
        cast: true,
        ..Default::default()
    });
    ticks(&mut sim, 5);
    assert_eq!(
        sim.world.get::<Actor>(sim.local_player).unwrap().health,
        91.25
    );
}

#[test]
fn jump_during_dash_does_not_consume_an_air_jump() {
    let mut sim = ready();
    sim.step(PlayerCommand {
        dash: true,
        movement: Vec2::X,
        ..Default::default()
    });
    sim.step(PlayerCommand {
        jump: true,
        ..Default::default()
    });
    assert_eq!(sim.world.get::<Actor>(sim.local_player).unwrap().jumps, 0);
    ticks(&mut sim, 20);
    sim.step(PlayerCommand {
        jump: true,
        ..Default::default()
    });
    assert_eq!(sim.world.get::<Actor>(sim.local_player).unwrap().jumps, 1);
    assert!(sim.world.get::<Body>(sim.local_player).unwrap().velocity.y > 600.0);
}

#[test]
fn fireball_cannot_start_beyond_a_nearby_thin_wall() {
    let mut sim = ready();
    sim.world.resource_mut::<Arena>().platforms.push(Platform {
        center: Vec2::new(-203.5, -233.0),
        half: Vec2::new(0.25, 60.0),
    });
    sim.step(PlayerCommand {
        cast: true,
        aim_world: Vec2::new(220.0, -233.0),
        ..Default::default()
    });
    ticks(&mut sim, 90);
    assert!(sim.snapshot().projectiles.is_empty());
    assert_eq!(sim.world.get::<Actor>(sim.dummy).unwrap().health, 100.0);
    assert_eq!(sim.snapshot().diagnostics.hits, 1); // Nearby blast hits its caster.
}

#[test]
fn original_caster_can_counter_return_without_targeting_themself() {
    let mut sim = ready();
    let shot = incoming(&mut sim, Vec2::new(-80.0, -233.0), Vec2::NEG_X * 650.0);
    sim.step(PlayerCommand {
        select: Some(Spell::Gust),
        cast: true,
        aim_world: Vec2::new(220.0, -233.0),
        ..Default::default()
    });
    sim.world.get_mut::<Projectile>(shot).unwrap().position = Vec2::new(100.0, -233.0);
    sim.world.entity_mut(sim.dummy).remove::<Dummy>();
    *sim.world.get_mut::<PlayerCommand>(sim.dummy).unwrap() = PlayerCommand {
        select: Some(Spell::Gust),
        cast: true,
        aim_world: Vec2::new(-220.0, -233.0),
        ..Default::default()
    };
    sim.step(PlayerCommand::default());
    let projectile = sim.world.get::<Projectile>(shot).unwrap();
    assert!(
        projectile.velocity.x < 0.0,
        "counter-return must fly away from its caster"
    );
    assert_eq!(projectile.owner, sim.dummy);
    assert_eq!(projectile.original_caster, sim.dummy);
    assert_eq!(sim.snapshot().diagnostics.returns, 2);
}

#[test]
fn dash_does_not_erase_vertical_hit_impulse() {
    let mut sim = ready();
    sim.step(PlayerCommand {
        dash: true,
        movement: Vec2::X,
        ..Default::default()
    });
    sim.world.resource_mut::<PendingHits>().0.push(Hit {
        target: sim.local_player,
        damage: 0.0,
        impulse: Vec2::Y * 200.0,
        slow_duration: 0.0,
        launch: false,
    });
    sim.step(PlayerCommand::default());
    let y = sim.world.get::<Body>(sim.local_player).unwrap().position.y;
    sim.step(PlayerCommand::default());
    let body = sim.world.get::<Body>(sim.local_player).unwrap();
    assert!(body.impulse.y > 0.0);
    assert!(body.position.y > y);
}

#[test]
fn invalid_and_oversized_inputs_cannot_poison_the_simulation() {
    let mut sim = ready();
    sim.step(PlayerCommand {
        movement: Vec2::new(f32::NAN, f32::INFINITY),
        aim_world: Vec2::splat(f32::NAN),
        dash: true,
        cast: true,
        ..Default::default()
    });
    ticks(&mut sim, 30);
    sim.step(PlayerCommand {
        movement: Vec2::splat(f32::MAX),
        aim_world: Vec2::splat(f32::MAX),
        dash: true,
        cast: true,
        ..Default::default()
    });
    for view in sim.snapshot().actors {
        assert!(view.body.position.is_finite());
        assert!(view.body.velocity.is_finite());
        assert!((view.actor.aim.length() - 1.0).abs() < 0.0001);
    }
    for projectile in sim.snapshot().projectiles {
        assert!(projectile.velocity.is_finite());
        assert!((projectile.velocity.length() - 650.0).abs() < 0.01);
    }
}

#[test]
fn invalid_tuning_is_rejected_before_it_can_stall_a_client() {
    assert!(
        Tuning {
            tick: 0.0,
            ..Default::default()
        }
        .validate()
        .is_err()
    );
    assert!(
        Tuning {
            health: f32::NAN,
            ..Default::default()
        }
        .validate()
        .is_err()
    );
    assert!(
        Tuning {
            fire_speed: f32::INFINITY,
            ..Default::default()
        }
        .validate()
        .is_err()
    );
    assert!(
        Tuning {
            respawn_time: 0.0,
            ..Default::default()
        }
        .validate()
        .is_err()
    );
}

#[test]
fn spell_switch_does_not_bypass_cast_cooldown() {
    let mut sim = ready();
    sim.step(PlayerCommand {
        cast: true,
        ..Default::default()
    });
    sim.step(PlayerCommand {
        cast: true,
        switch: true,
        ..Default::default()
    });
    assert_eq!(
        sim.world.get::<Actor>(sim.local_player).unwrap().spell,
        Spell::Gust
    );
    assert_eq!(sim.snapshot().diagnostics.casts, 1);
}

fn incoming(sim: &mut Simulation, position: Vec2, velocity: Vec2) -> Entity {
    sim.world
        .spawn(Projectile {
            id: 0,
            position,
            velocity,
            owner: sim.dummy,
            original_caster: sim.dummy,
            lifetime: 2.5,
            reflected: false,
        })
        .id()
}

#[test]
fn gust_returns_incoming_projectile_to_original_caster() {
    let mut sim = ready();
    let entity = incoming(&mut sim, Vec2::new(-80.0, -233.0), Vec2::NEG_X * 650.0);
    sim.step(PlayerCommand {
        select: Some(Spell::Gust),
        cast: true,
        aim_world: Vec2::new(220.0, -233.0),
        ..Default::default()
    });
    let shot = sim.world.get::<Projectile>(entity).unwrap();
    assert!(shot.velocity.x > 0.0);
    assert_eq!(shot.owner, sim.local_player);
    assert_eq!(shot.original_caster, sim.dummy);
    ticks(&mut sim, 90);
    assert_eq!(sim.world.get::<Actor>(sim.dummy).unwrap().health, 75.0);
    assert_eq!(sim.snapshot().diagnostics.returns, 1);
}

#[test]
fn gust_rejects_projectiles_behind_outside_range_and_moving_away() {
    let mut sim = ready();
    let behind = incoming(&mut sim, Vec2::new(-320.0, -233.0), Vec2::X * 650.0);
    let far = incoming(&mut sim, Vec2::new(100.0, -233.0), Vec2::NEG_X * 650.0);
    let away = incoming(&mut sim, Vec2::new(-80.0, -233.0), Vec2::X * 650.0);
    sim.step(PlayerCommand {
        select: Some(Spell::Gust),
        cast: true,
        aim_world: Vec2::new(220.0, -233.0),
        ..Default::default()
    });
    for entity in [behind, far, away] {
        assert_eq!(
            sim.world.get::<Projectile>(entity).unwrap().owner,
            sim.dummy
        );
    }
    assert_eq!(sim.snapshot().diagnostics.returns, 0);
}

#[test]
fn gust_pushes_only_targets_in_forward_range_without_damage() {
    for (offset, affected) in [
        (120.0, true),
        (190.0, true),
        (191.0, false),
        (-120.0, false),
    ] {
        let mut sim = ready();
        sim.world.get_mut::<Body>(sim.dummy).unwrap().position.x = -220.0 + offset;
        sim.step(PlayerCommand {
            select: Some(Spell::Gust),
            cast: true,
            aim_world: Vec2::new(220.0, -233.0),
            ..Default::default()
        });
        let impulse = sim.world.get::<Body>(sim.dummy).unwrap().impulse;
        if affected {
            assert!(
                sim.world.get::<Body>(sim.dummy).unwrap().velocity.x > 800.0,
                "no launch at offset {offset}"
            );
            assert!(!sim.world.get::<Body>(sim.dummy).unwrap().grounded);
            assert!(sim.world.get::<Actor>(sim.dummy).unwrap().slow_left > 0.0);
        } else {
            assert_eq!(
                impulse,
                Vec2::ZERO,
                "pushed outside the cone/range at {offset}"
            );
            assert_eq!(sim.world.get::<Actor>(sim.dummy).unwrap().slow_left, 0.0);
        }
        assert_eq!(sim.world.get::<Actor>(sim.dummy).unwrap().health, 100.0);
        assert_eq!(
            sim.world.get::<Actor>(sim.local_player).unwrap().slow_left,
            0.0
        );
    }
}

fn launch_dummy(
    sign: f32,
    incoming_velocity: Vec2,
    old_impulse: Vec2,
    dashing: bool,
) -> Simulation {
    let mut sim = ready();
    sim.world.entity_mut(sim.dummy).remove::<Dummy>();
    sim.world
        .get_mut::<Body>(sim.local_player)
        .unwrap()
        .position
        .x = 0.0;
    {
        let mut body = sim.world.get_mut::<Body>(sim.dummy).unwrap();
        body.position = Vec2::new(sign * 120.0, -233.0);
        if dashing || incoming_velocity.y < 0.0 {
            body.position.y = -180.0;
        }
        body.velocity = incoming_velocity;
        body.impulse = old_impulse;
        body.grounded = false;
    }
    *sim.world.get_mut::<PlayerCommand>(sim.dummy).unwrap() = PlayerCommand {
        movement: Vec2::new(incoming_velocity.x.signum(), 0.0),
        ..Default::default()
    };
    if dashing {
        let mut actor = sim.world.get_mut::<Actor>(sim.dummy).unwrap();
        actor.dash_left = 0.1;
        actor.dash_cooldown = 0.5;
        actor.dash_direction = incoming_velocity.normalize();
        actor.air_dash_available = false;
    }
    sim.step(PlayerCommand {
        select: Some(Spell::Gust),
        cast: true,
        aim_world: Vec2::new(sign * 300.0, -233.0),
        ..Default::default()
    });
    sim
}

#[test]
fn gust_launch_ignores_incoming_run_dash_fall_and_old_knockback() {
    for sign in [-1.0, 1.0] {
        for incoming in [
            Vec2::new(-330.0, 0.0),
            Vec2::new(330.0, 0.0),
            Vec2::new(0.0, -1100.0),
            Vec2::new(0.0, 650.0),
            Vec2::ZERO,
        ] {
            let sim = launch_dummy(sign, incoming, Vec2::new(-sign * 600.0, -200.0), false);
            let body = sim.world.get::<Body>(sim.dummy).unwrap();
            assert_eq!(body.velocity, Vec2::new(sign * 850.0, 500.0));
            assert_eq!(body.impulse, Vec2::ZERO);
            assert!(!body.grounded);
        }
        for incoming_x in [-780.0, 780.0] {
            let sim = launch_dummy(sign, Vec2::new(incoming_x, 0.0), Vec2::ZERO, true);
            let actor = sim.world.get::<Actor>(sim.dummy).unwrap();
            assert_eq!(
                sim.world.get::<Body>(sim.dummy).unwrap().velocity,
                Vec2::new(sign * 850.0, 500.0)
            );
            assert_eq!(actor.dash_left, 0.0);
            assert!(actor.dash_cooldown > 0.0);
            assert!(!actor.air_dash_available);
        }
    }
}

#[test]
fn gust_launch_is_strong_and_airborne_despite_opposing_controls() {
    for sign in [-1.0, 1.0] {
        let mut sim = launch_dummy(sign, Vec2::new(-sign * 330.0, 0.0), Vec2::ZERO, false);
        let start = sim.world.get::<Body>(sim.dummy).unwrap().position;
        *sim.world.get_mut::<PlayerCommand>(sim.dummy).unwrap() = PlayerCommand {
            movement: Vec2::new(-sign, 0.0),
            jump: true,
            dash: true,
            ..Default::default()
        };
        ticks(&mut sim, 12);
        let body = sim.world.get::<Body>(sim.dummy).unwrap();
        assert!((sign * (body.position.x - start.x) - 85.0).abs() < 0.01);
        assert!(body.position.y - start.y > 35.0);
        assert!(!body.grounded);
        let actor = sim.world.get::<Actor>(sim.dummy).unwrap();
        assert!(actor.gust_launch_left > 0.0);
        assert_eq!(actor.dash_left, 0.0);
        assert_eq!(actor.jumps, 0);
    }
}

#[test]
fn steering_returns_after_gust_launch_while_slow_remains() {
    let mut sim = launch_dummy(1.0, Vec2::NEG_X * 330.0, Vec2::ZERO, false);
    ticks(&mut sim, 30);
    let actor = sim.world.get::<Actor>(sim.dummy).unwrap();
    assert_eq!(actor.gust_launch_left, 0.0);
    assert!(actor.slow_left > 0.0);
    assert!(sim.world.get::<Body>(sim.dummy).unwrap().velocity.x < 800.0);
}

#[test]
fn protected_target_ignores_the_gust_launch() {
    let mut sim = Simulation::default();
    sim.world.get_mut::<Body>(sim.dummy).unwrap().position.x = -100.0;
    sim.step(PlayerCommand {
        select: Some(Spell::Gust),
        cast: true,
        aim_world: Vec2::new(220.0, -233.0),
        ..Default::default()
    });
    let actor = sim.world.get::<Actor>(sim.dummy).unwrap();
    assert_eq!(actor.gust_launch_left, 0.0);
    assert_eq!(actor.slow_left, 0.0);
    assert_eq!(
        sim.world.get::<Body>(sim.dummy).unwrap().velocity,
        Vec2::ZERO
    );
    assert!(sim.world.get::<Body>(sim.dummy).unwrap().grounded);
}

#[test]
fn gust_launch_collides_with_walls_instead_of_passing_through() {
    let mut sim = launch_dummy(1.0, Vec2::NEG_X * 330.0, Vec2::ZERO, false);
    sim.world.resource_mut::<Arena>().platforms.push(Platform {
        center: Vec2::new(165.0, -180.0),
        half: Vec2::new(5.0, 150.0),
    });
    ticks(&mut sim, 12);
    let body = sim.world.get::<Body>(sim.dummy).unwrap();
    assert!(body.position.x <= 144.001);
    assert!(body.position.y > -233.0);
}

#[test]
fn defeat_and_respawn_clear_gust_launch_state() {
    let mut sim = ready();
    sim.world
        .get_mut::<Actor>(sim.local_player)
        .unwrap()
        .gust_launch_left = 10.0;
    sim.world
        .get_mut::<Body>(sim.local_player)
        .unwrap()
        .position
        .y = -500.0;
    sim.step(PlayerCommand::default());
    assert_eq!(
        sim.world
            .get::<Actor>(sim.local_player)
            .unwrap()
            .gust_launch_left,
        0.0
    );
    ticks(&mut sim, 245);
    let actor = sim.world.get::<Actor>(sim.local_player).unwrap();
    assert_eq!(actor.generation, 1);
    assert_eq!(actor.gust_launch_left, 0.0);
}

fn slow_local(sim: &mut Simulation, duration: f32) {
    sim.world.resource_mut::<PendingHits>().0.push(Hit {
        target: sim.local_player,
        damage: 0.0,
        impulse: Vec2::ZERO,
        slow_duration: duration,
        launch: false,
    });
    sim.step(PlayerCommand::default());
}

#[test]
fn gust_slow_reduces_running_speed_and_expires() {
    let mut sim = ready();
    slow_local(&mut sim, 1.0);
    for _ in 0..30 {
        sim.step(PlayerCommand {
            movement: Vec2::X,
            ..Default::default()
        });
    }
    let slowed = sim.world.get::<Body>(sim.local_player).unwrap().velocity.x;
    assert!((slowed - 231.0).abs() < 0.01);
    for _ in 0..100 {
        sim.step(PlayerCommand {
            movement: Vec2::X,
            ..Default::default()
        });
    }
    assert_eq!(
        sim.world.get::<Actor>(sim.local_player).unwrap().slow_left,
        0.0
    );
    assert!((sim.world.get::<Body>(sim.local_player).unwrap().velocity.x - 330.0).abs() < 0.01);
}

#[test]
fn repeated_slows_refresh_duration_without_stacking_strength_or_time() {
    let mut sim = ready();
    slow_local(&mut sim, 1.0);
    ticks(&mut sim, 60);
    slow_local(&mut sim, 1.0);
    assert_eq!(
        sim.world.get::<Actor>(sim.local_player).unwrap().slow_left,
        1.0
    );
    for _ in 0..30 {
        sim.step(PlayerCommand {
            movement: Vec2::X,
            ..Default::default()
        });
    }
    assert!((sim.world.get::<Body>(sim.local_player).unwrap().velocity.x - 231.0).abs() < 0.01);
}

#[test]
fn gust_slow_preserves_jump_but_reduces_dash_speed() {
    let mut sim = ready();
    slow_local(&mut sim, 1.0);
    sim.step(PlayerCommand {
        jump: true,
        ..Default::default()
    });
    assert!(sim.world.get::<Body>(sim.local_player).unwrap().velocity.y > 600.0);
    sim.step(PlayerCommand {
        movement: Vec2::X,
        dash: true,
        ..Default::default()
    });
    assert!((sim.world.get::<Body>(sim.local_player).unwrap().velocity.x - 546.0).abs() < 0.01);
    assert!(sim.world.get::<Actor>(sim.local_player).unwrap().slow_left > 0.0);
}

#[test]
fn gust_slow_shortens_dash_distance_in_every_direction() {
    for direction in [Vec2::X, Vec2::NEG_X, Vec2::Y, Vec2::NEG_Y, Vec2::ONE] {
        let distance = |slowed: bool| {
            let mut sim = airborne();
            if slowed {
                slow_local(&mut sim, 1.0);
            }
            let start = sim.world.get::<Body>(sim.local_player).unwrap().position;
            sim.step(PlayerCommand {
                movement: direction,
                dash: true,
                ..Default::default()
            });
            while sim.world.get::<Actor>(sim.local_player).unwrap().dash_left > 0.0 {
                sim.step(PlayerCommand::default());
            }
            sim.world
                .get::<Body>(sim.local_player)
                .unwrap()
                .position
                .distance(start)
        };
        let normal = distance(false);
        let slowed = distance(true);
        assert!(
            (slowed / normal - 0.70).abs() < 0.001,
            "unexpected dash reach for {direction:?}: {slowed} vs {normal}"
        );
    }
}

#[test]
fn dash_speed_recovers_when_gust_slow_expires() {
    let mut sim = ready();
    slow_local(&mut sim, 1.0);
    sim.step(PlayerCommand {
        movement: Vec2::X,
        dash: true,
        ..Default::default()
    });
    ticks(&mut sim, 130);
    assert_eq!(
        sim.world.get::<Actor>(sim.local_player).unwrap().slow_left,
        0.0
    );
    assert!(sim.world.get::<Body>(sim.local_player).unwrap().grounded);
    sim.step(PlayerCommand {
        movement: Vec2::X,
        dash: true,
        ..Default::default()
    });
    assert!((sim.world.get::<Body>(sim.local_player).unwrap().velocity.x - 780.0).abs() < 0.01);
}

#[test]
fn gust_slow_affects_an_active_dash_and_recovers_mid_dash() {
    let mut sim = airborne();
    sim.step(PlayerCommand {
        movement: Vec2::X,
        dash: true,
        ..Default::default()
    });
    slow_local(&mut sim, 0.05);
    sim.step(PlayerCommand::default());
    assert!((sim.world.get::<Body>(sim.local_player).unwrap().velocity.x - 546.0).abs() < 0.01);
    ticks(&mut sim, 7);
    let actor = sim.world.get::<Actor>(sim.local_player).unwrap();
    assert_eq!(actor.slow_left, 0.0);
    assert!(actor.dash_left > 0.0);
    assert!((sim.world.get::<Body>(sim.local_player).unwrap().velocity.x - 780.0).abs() < 0.01);
}

#[test]
fn fireball_does_not_apply_gust_slow() {
    let mut sim = ready();
    sim.step(PlayerCommand {
        cast: true,
        aim_world: Vec2::new(220.0, -233.0),
        ..Default::default()
    });
    ticks(&mut sim, 90);
    assert_eq!(sim.world.get::<Actor>(sim.dummy).unwrap().health, 75.0);
    assert_eq!(sim.world.get::<Actor>(sim.dummy).unwrap().slow_left, 0.0);
}

#[test]
fn respawn_clears_a_long_slow() {
    let mut sim = ready();
    slow_local(&mut sim, 10.0);
    assert!(sim.world.get::<Actor>(sim.local_player).unwrap().slow_left > 0.0);
    sim.world
        .get_mut::<Body>(sim.local_player)
        .unwrap()
        .position
        .y = -500.0;
    sim.step(PlayerCommand::default());
    ticks(&mut sim, 245);
    let actor = sim.world.get::<Actor>(sim.local_player).unwrap();
    assert_eq!(actor.generation, 1);
    assert_eq!(actor.slow_left, 0.0);
}

#[test]
fn dummy_gust_mode_can_slow_the_local_player_for_playtesting() {
    let mut sim = ready();
    sim.world.get_mut::<Body>(sim.dummy).unwrap().position.x = -100.0;
    sim.toggle_dummy_spell();
    sim.toggle_dummy();
    ticks(&mut sim, 80);
    assert!(sim.world.get::<Actor>(sim.local_player).unwrap().slow_left > 0.0);
    assert_eq!(sim.world.get::<Actor>(sim.dummy).unwrap().slow_left, 0.0);
}

#[test]
fn invalid_slow_tuning_is_rejected() {
    for multiplier in [-0.1, 1.1, f32::NAN] {
        assert!(
            Tuning {
                gust_slow_speed_multiplier: multiplier,
                ..Default::default()
            }
            .validate()
            .is_err()
        );
    }
    assert!(
        Tuning {
            gust_slow_duration: -1.0,
            ..Default::default()
        }
        .validate()
        .is_err()
    );
}

#[test]
fn spawn_protection_blocks_damage_and_cast_ends_protection() {
    let mut sim = Simulation::default();
    sim.world.resource_mut::<PendingHits>().0.push(Hit {
        target: sim.local_player,
        damage: 100.0,
        impulse: Vec2::X * 500.0,
        slow_duration: 1.0,
        launch: false,
    });
    sim.step(PlayerCommand::default());
    assert_eq!(
        sim.world.get::<Actor>(sim.local_player).unwrap().health,
        100.0
    );
    assert_eq!(
        sim.world.get::<Body>(sim.local_player).unwrap().impulse,
        Vec2::ZERO
    );
    assert_eq!(
        sim.world.get::<Actor>(sim.local_player).unwrap().slow_left,
        0.0
    );
    sim.step(PlayerCommand {
        cast: true,
        ..Default::default()
    });
    assert_eq!(
        sim.world.get::<Actor>(sim.local_player).unwrap().protection,
        0.0
    );
}

#[test]
fn defeat_cleans_projectiles_disables_actions_and_respawns_once() {
    let mut sim = ready();
    sim.step(PlayerCommand {
        cast: true,
        ..Default::default()
    });
    sim.world.resource_mut::<PendingHits>().0.push(Hit {
        target: sim.local_player,
        damage: 100.0,
        impulse: Vec2::ZERO,
        slow_duration: 1.0,
        launch: false,
    });
    sim.step(PlayerCommand::default());
    assert!(!sim.world.get::<Actor>(sim.local_player).unwrap().alive());
    assert_eq!(
        sim.world.get::<Actor>(sim.local_player).unwrap().slow_left,
        0.0
    );
    assert!(sim.snapshot().projectiles.is_empty());
    for _ in 0..60 {
        sim.step(PlayerCommand {
            jump: true,
            dash: true,
            cast: true,
            movement: Vec2::X,
            ..Default::default()
        });
    }
    assert!(sim.snapshot().projectiles.is_empty());
    assert!(!sim.world.get::<Actor>(sim.local_player).unwrap().alive());
    ticks(&mut sim, 185);
    let actor = sim.world.get::<Actor>(sim.local_player).unwrap();
    assert_eq!(actor.health, 100.0);
    assert_eq!(actor.generation, 1);
    assert_eq!(actor.spell, Spell::Fireball);
    assert!(actor.protection > 0.0);
    assert_eq!(sim.snapshot().diagnostics.respawns, 1);
}

#[test]
fn falling_outside_arena_causes_defeat() {
    let mut sim = ready();
    sim.world
        .get_mut::<Body>(sim.local_player)
        .unwrap()
        .position
        .y = -500.0;
    sim.step(PlayerCommand::default());
    assert!(!sim.world.get::<Actor>(sim.local_player).unwrap().alive());
    ticks(&mut sim, 245);
    assert_eq!(
        sim.world.get::<Actor>(sim.local_player).unwrap().generation,
        1
    );
}

#[test]
fn repeated_combat_remains_finite_and_entities_are_bounded() {
    let mut sim = ready();
    sim.toggle_dummy();
    for tick in 0..12_000 {
        let target = sim.world.get::<Body>(sim.dummy).unwrap().position;
        sim.step(PlayerCommand {
            movement: Vec2::new(if (tick / 200) % 2 == 0 { 0.3 } else { -0.3 }, 0.0),
            aim_world: target,
            cast: true,
            jump: tick % 150 == 0,
            dash: tick % 230 == 0,
            switch: tick % 110 == 0,
            select: None,
        });
    }
    let snapshot = sim.snapshot();
    assert_eq!(snapshot.actors.len(), 2);
    assert!(snapshot.projectiles.len() < 20);
    assert!(snapshot.effects.len() < 20);
    for view in snapshot.actors {
        assert!(view.body.position.is_finite());
        assert!(view.body.velocity.is_finite());
        assert!((0.0..=100.0).contains(&view.actor.health));
    }
    assert!(snapshot.diagnostics.casts > 100);
    assert!(snapshot.diagnostics.defeats > 0);
    assert!(snapshot.diagnostics.respawns > 0);
}
