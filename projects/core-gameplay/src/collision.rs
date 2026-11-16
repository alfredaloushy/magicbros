use glam::Vec2;

use crate::model::{Body, Platform};

/// Earliest hit along a segment, including segments starting inside the box.
pub fn segment_box(start: Vec2, end: Vec2, center: Vec2, half: Vec2) -> Option<f32> {
    let delta = end - start;
    let min = center - half;
    let max = center + half;
    let mut enter = 0.0_f32;
    let mut exit = 1.0_f32;
    for axis in 0..2 {
        if delta[axis].abs() < 0.00001 {
            if start[axis] < min[axis] || start[axis] > max[axis] {
                return None;
            }
        } else {
            let a = (min[axis] - start[axis]) / delta[axis];
            let b = (max[axis] - start[axis]) / delta[axis];
            enter = enter.max(a.min(b));
            exit = exit.min(a.max(b));
            if enter > exit {
                return None;
            }
        }
    }
    Some(enter)
}

/// Solid, axis-separated swept AABBs: walking, jumping and dashing cannot tunnel.
pub fn move_body(body: &mut Body, platforms: &[Platform], dt: f32) {
    body.grounded = false;
    for axis in 0..2 {
        let step = (body.velocity[axis] + body.impulse[axis]) * dt;
        let other = 1 - axis;
        let mut allowed = step;
        for p in platforms {
            if (body.position[other] - p.center[other]).abs()
                >= body.half[other] + p.half[other] - 0.001
            {
                continue;
            }
            let sum = body.half[axis] + p.half[axis];
            let offset = body.position[axis] - p.center[axis];
            if step > 0.0 && offset <= -sum + 0.001 {
                let distance = -sum - offset;
                if distance >= -0.001 && distance <= allowed {
                    allowed = distance.max(0.0);
                }
            } else if step < 0.0 && offset >= sum - 0.001 {
                let distance = sum - offset;
                if distance <= 0.001 && distance >= allowed {
                    allowed = distance.min(0.0);
                }
            }
        }
        body.position[axis] += allowed;
        if (allowed - step).abs() > 0.00001 {
            body.velocity[axis] = 0.0;
            body.impulse[axis] = 0.0;
            if axis == 1 && step < 0.0 {
                body.grounded = true;
            }
        }
    }
    // A horizontal dash has no vertical step, so it produces no landing collision.
    // Preserve ground contact while the feet remain supported by a platform.
    if !body.grounded && body.velocity.y + body.impulse.y <= 0.0 {
        let feet = body.position.y - body.half.y;
        body.grounded = platforms.iter().any(|p| {
            (feet - (p.center.y + p.half.y)).abs() <= 0.001
                && (body.position.x - p.center.x).abs() < body.half.x + p.half.x - 0.001
        });
    }
}
