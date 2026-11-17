use bevy_ecs::prelude::*;
use glam::Vec2;

use crate::{
    collision::{move_body, segment_box},
    model::*,
    tuning::Tuning,
};

pub fn tick_lifetimes(
    tuning: Res<Tuning>,
    mut actors: Query<&mut Actor>,
    mut effects: ResMut<Effects>,
    mut diagnostics: ResMut<Diagnostics>,
) {
    diagnostics.tick += 1;
    for mut actor in &mut actors {
        actor.protection = (actor.protection - tuning.tick).max(0.0);
        actor.hit_flash = (actor.hit_flash - tuning.tick).max(0.0);
        actor.health_reveal = (actor.health_reveal - tuning.tick).max(0.0);
        actor.slow_left = (actor.slow_left - tuning.tick).max(0.0);
        actor.gust_launch_left = (actor.gust_launch_left - tuning.tick).max(0.0);
        actor.cooldown = (actor.cooldown - tuning.tick).max(0.0);
        actor.dash_cooldown = (actor.dash_cooldown - tuning.tick).max(0.0);
    }
    for effect in &mut effects.0 {
        effect.left -= tuning.tick;
    }
    effects.0.retain(|e| e.left > 0.0);
}

pub fn respawn(
    tuning: Res<Tuning>,
    mut actors: Query<(&mut Actor, &mut Body, &mut PlayerCommand)>,
    mut diagnostics: ResMut<Diagnostics>,
) {
    for (mut actor, mut body, mut input) in &mut actors {
        if actor.alive() {
            continue;
        }
        actor.respawn_left = (actor.respawn_left - tuning.tick).max(0.0);
        if actor.respawn_left > 0.0 {
            continue;
        }
        actor.health = tuning.health;
        actor.protection = tuning.spawn_protection;
        actor.hit_flash = 0.0;
        actor.health_reveal = 0.0;
        actor.slow_left = 0.0;
        actor.gust_launch_left = 0.0;
        actor.cooldown = 0.0;
        actor.dash_cooldown = 0.0;
        actor.air_dash_available = true;
        actor.dash_left = 0.0;
        actor.jumps = 0;
        actor.spell = Spell::Fireball;
        actor.generation += 1;
        body.position = actor.spawn;
        body.velocity = Vec2::ZERO;
        body.impulse = Vec2::ZERO;
        body.grounded = false;
        *input = PlayerCommand::default();
        diagnostics.respawns += 1;
    }
}

pub fn dummy_input(
    tuning: Res<Tuning>,
    local: Query<&Body, (With<Actor>, Without<Dummy>)>,
    mut dummies: Query<(&mut Dummy, &Actor, &mut PlayerCommand)>,
) {
    let Some(target) = local.iter().next() else {
        return;
    };
    for (mut dummy, actor, mut input) in &mut dummies {
        *input = PlayerCommand {
            aim_world: target.position,
            ..Default::default()
        };
        if !dummy.firing || !actor.alive() {
            continue;
        }
        dummy.timer -= tuning.tick;
        if dummy.timer <= 0.0 {
            input.cast = true;
            dummy.timer = tuning.dummy_fire_interval;
        }
    }
}

pub fn apply_input(mut actors: Query<(&mut Actor, &Body, &PlayerCommand)>) {
    for (mut actor, body, input) in &mut actors {
        if !actor.alive() {
            continue;
        }
        let aim = input.aim_world - body.position;
        if aim.is_finite() && aim.length_squared() > 0.01 {
            actor.aim = aim.normalize();
        }
        if input.switch {
            actor.spell = match actor.spell {
                Spell::Fireball => Spell::Gust,
                Spell::Gust => Spell::Fireball,
            };
        }
        if let Some(spell) = input.select {
            actor.spell = spell;
        }
    }
}

pub fn movement(
    tuning: Res<Tuning>,
    arena: Res<Arena>,
    mut actors: Query<(&mut Actor, &mut Body, &PlayerCommand)>,
) {
    for (mut actor, mut body, input) in &mut actors {
        if !actor.alive() {
            continue;
        }
        if body.grounded {
            actor.jumps = 0;
            actor.air_dash_available = true;
        }
        let launched = actor.gust_launch_left > 0.0;
        if !launched
            && input.dash
            && actor.dash_cooldown <= 0.0
            && (body.grounded || actor.air_dash_available)
        {
            let wish = input.movement;
            actor.dash_direction = if wish.length_squared() > 0.01 {
                wish.normalize()
            } else {
                actor.aim
            };
            actor.dash_left = tuning.dash_time;
            actor.dash_cooldown = tuning.dash_cooldown;
            // A dash that launches from the ground also spends the flight's charge.
            // Ground contact below restores it; cooldown alone never does.
            actor.air_dash_available = false;
        }
        // Dash takes priority; an ignored jump must not spend an airborne jump.
        if !launched && input.jump && actor.jumps < 2 && actor.dash_left <= 0.0 {
            body.velocity.y = tuning.jump_speed;
            body.grounded = false;
            actor.jumps += 1;
        }
        let speed_multiplier = if actor.slow_left > 0.0 {
            tuning.gust_slow_speed_multiplier
        } else {
            1.0
        };
        if launched {
            // Brief ballistic launch: steering, jump and dash cannot cancel the hit.
            body.velocity.y -= tuning.gravity * tuning.tick;
        } else if actor.dash_left > 0.0 {
            // Preserve dash timing/cooldown; slower travel also shortens its reach.
            // Re-evaluate each tick so Gust affects an already-active dash too.
            body.velocity = actor.dash_direction * tuning.dash_speed * speed_multiplier;
            actor.dash_left = (actor.dash_left - tuning.tick).max(0.0);
        } else {
            let desired = input.movement.x.clamp(-1.0, 1.0) * tuning.run_speed * speed_multiplier;
            let change = (desired - body.velocity.x).clamp(
                -tuning.acceleration * tuning.tick,
                tuning.acceleration * tuning.tick,
            );
            body.velocity.x += change;
            body.velocity.y = (body.velocity.y - tuning.gravity * tuning.tick).max(-1100.0);
        }
        move_body(&mut body, &arena.platforms, tuning.tick);
        if body.grounded {
            actor.air_dash_available = true;
        }
        body.impulse *= (-5.0 * tuning.tick).exp();
    }
}

pub fn cast(
    mut commands: Commands,
    tuning: Res<Tuning>,
    mut actors: Query<(Entity, &mut Actor, &Body, &PlayerCommand)>,
    mut projectiles: Query<&mut Projectile>,
    mut effects: ResMut<Effects>,
    state: (
        ResMut<PendingHits>,
        ResMut<Diagnostics>,
        ResMut<ProjectileIds>,
    ),
) {
    let (mut hits, mut diagnostics, mut ids) = state;
    let mut gusts = Vec::new();
    for (entity, mut actor, body, input) in &mut actors {
        if !actor.alive() || !input.cast || actor.cooldown > 0.0 {
            continue;
        }
        actor.protection = 0.0;
        diagnostics.casts += 1;
        match actor.spell {
            Spell::Fireball => {
                actor.cooldown = tuning.fire_cooldown;
                ids.0 += 1;
                commands.spawn(Projectile {
                    id: ids.0,
                    // Begin at the body center and sweep the whole path. A muzzle
                    // offset can place a shot beyond a nearby thin wall or enemy.
                    position: body.position,
                    velocity: actor.aim * tuning.fire_speed,
                    owner: entity,
                    original_caster: entity,
                    lifetime: tuning.fire_lifetime,
                    reflected: false,
                });
            }
            Spell::Gust => {
                actor.cooldown = tuning.gust_cooldown;
                gusts.push((entity, body.position, actor.aim));
                effects.0.push(Effect {
                    position: body.position,
                    radius: tuning.gust_radius,
                    left: 0.22,
                    duration: 0.22,
                    kind: EffectKind::Gust {
                        direction: actor.aim,
                    },
                });
            }
        }
    }
    // Separate collection/application avoids borrowing an actor twice in an ECS query.
    for (caster, origin, direction) in gusts {
        for (target, actor, body, _) in &actors {
            if target == caster || !actor.alive() {
                continue;
            }
            if in_gust(body.position - origin, direction, tuning.gust_radius) {
                hits.0.push(Hit {
                    target,
                    damage: 0.0,
                    impulse: Vec2::new(
                        direction.x * tuning.gust_push,
                        (direction.y * tuning.gust_push).max(tuning.gust_lift),
                    ),
                    slow_duration: tuning.gust_slow_duration,
                    launch: true,
                });
            }
        }
        for mut projectile in &mut projectiles {
            if projectile.owner == caster {
                continue;
            }
            let offset = projectile.position - origin;
            if !in_gust(offset, direction, tuning.gust_radius)
                || projectile.velocity.dot(offset) >= 0.0
            {
                continue;
            }
            // A counter-return must not target its own caster. First returns
            // target the original caster; that caster can return to the last owner.
            let return_target = if projectile.original_caster == caster {
                projectile.owner
            } else {
                projectile.original_caster
            };
            let Ok((_, target, body, _)) = actors.get(return_target) else {
                continue;
            };
            if !target.alive() {
                continue;
            }
            let return_aim = (body.position - projectile.position).normalize_or_zero();
            if return_aim == Vec2::ZERO {
                continue;
            }
            projectile.velocity = return_aim * tuning.fire_speed;
            projectile.owner = caster;
            projectile.reflected = true;
            diagnostics.returns += 1;
        }
    }
}

pub fn in_gust(offset: Vec2, direction: Vec2, radius: f32) -> bool {
    offset.length_squared() <= radius * radius
        && (offset.length_squared() < 0.001 || offset.normalize().dot(direction) >= 0.35)
}

pub fn projectiles(
    mut commands: Commands,
    tuning: Res<Tuning>,
    arena: Res<Arena>,
    mut projectiles: Query<(Entity, &mut Projectile)>,
    actors: Query<(Entity, &Actor, &Body)>,
    mut hits: ResMut<PendingHits>,
    mut effects: ResMut<Effects>,
) {
    for (entity, mut projectile) in &mut projectiles {
        projectile.lifetime -= tuning.tick;
        if projectile.lifetime <= 0.0 {
            commands.entity(entity).despawn();
            continue;
        }
        let start = projectile.position;
        let end = start + projectile.velocity * tuning.tick;
        let mut first: Option<f32> = None;
        for platform in &arena.platforms {
            if let Some(t) = segment_box(
                start,
                end,
                platform.center,
                platform.half + Vec2::splat(7.0),
            ) {
                first = Some(first.map_or(t, |a| a.min(t)));
            }
        }
        for (target, actor, body) in &actors {
            if target == projectile.owner || !actor.alive() {
                continue;
            }
            if let Some(t) = segment_box(start, end, body.position, body.half + Vec2::splat(7.0)) {
                first = Some(first.map_or(t, |a| a.min(t)));
            }
        }
        if let Some(t) = first {
            let impact = start.lerp(end, t);
            effects.0.push(Effect {
                position: impact,
                radius: tuning.fire_radius,
                left: 0.28,
                duration: 0.28,
                kind: EffectKind::Explosion,
            });
            for (target, actor, body) in &actors {
                if !actor.alive() {
                    continue;
                }
                let nearest = impact.clamp(body.position - body.half, body.position + body.half);
                if nearest.distance_squared(impact) > tuning.fire_radius.powi(2) {
                    continue;
                }
                let direction = (body.position - impact).normalize_or_zero();
                let scale = if target == projectile.owner {
                    tuning.self_damage_scale
                } else {
                    1.0
                };
                hits.0.push(Hit {
                    target,
                    damage: tuning.fire_damage * scale,
                    impulse: direction * tuning.fire_push + Vec2::Y * 65.0,
                    slow_duration: 0.0,
                    launch: false,
                });
            }
            commands.entity(entity).despawn();
        } else {
            projectile.position = end;
        }
    }
}

pub fn damage(
    tuning: Res<Tuning>,
    mut hits: ResMut<PendingHits>,
    mut actors: Query<(&mut Actor, &mut Body)>,
    mut diagnostics: ResMut<Diagnostics>,
) {
    for hit in hits.0.drain(..) {
        let Ok((mut actor, mut body)) = actors.get_mut(hit.target) else {
            continue;
        };
        if !actor.alive() || actor.protection > 0.0 {
            continue;
        }
        actor.health = (actor.health - hit.damage).max(0.0);
        if hit.launch {
            // Incoming run, dash, fall and previous knockback never subtract
            // from Gust's launch. Terrain collision still resolves normally.
            body.velocity = hit.impulse;
            body.impulse = Vec2::ZERO;
            body.grounded = false;
            actor.dash_left = 0.0;
            actor.gust_launch_left = tuning.gust_launch_time;
        } else {
            body.impulse += hit.impulse;
        }
        // Repeated hits refresh the timer; they never multiply the slow strength.
        actor.slow_left = actor.slow_left.max(hit.slow_duration);
        if hit.damage > 0.0 {
            actor.hit_flash = 0.14;
            actor.health_reveal = 2.0;
            diagnostics.hits += 1;
        }
    }
}

pub fn defeat(
    mut commands: Commands,
    tuning: Res<Tuning>,
    mut actors: Query<(Entity, &mut Actor, &mut Body)>,
    projectiles: Query<(Entity, &Projectile)>,
    mut diagnostics: ResMut<Diagnostics>,
) {
    let mut defeated = Vec::new();
    for (entity, mut actor, mut body) in &mut actors {
        if body.position.y < -430.0 || body.position.x.abs() > 900.0 {
            actor.health = 0.0;
        }
        if actor.alive() || actor.respawn_left > 0.0 {
            continue;
        }
        actor.respawn_left = tuning.respawn_time;
        actor.dash_left = 0.0;
        actor.slow_left = 0.0;
        actor.gust_launch_left = 0.0;
        body.velocity = Vec2::ZERO;
        body.impulse = Vec2::ZERO;
        defeated.push(entity);
        diagnostics.defeats += 1;
    }
    for (entity, projectile) in &projectiles {
        if defeated.contains(&projectile.owner) || defeated.contains(&projectile.original_caster) {
            commands.entity(entity).despawn();
        }
    }
}
