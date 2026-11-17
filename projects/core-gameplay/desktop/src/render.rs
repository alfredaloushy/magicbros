use glam::Vec2;
use std::{
    fs::File,
    io::{self, Write},
};
use wild_magic_core::model::{EffectKind, Snapshot, Spell};

pub const WIDTH: usize = 1280;
pub const HEIGHT: usize = 720;
const INK: u32 = 0xe9e5fa;
const MUTED: u32 = 0x9694b4;
const FIRE: u32 = 0xffb45e;
const GUST: u32 = 0x7ee8d8;
const BLUE: u32 = 0xada1ff;

pub fn screen_to_world(p: Vec2) -> Vec2 {
    Vec2::new(p.x - 640.0, 360.0 - p.y)
}
fn screen(p: Vec2) -> Vec2 {
    Vec2::new(p.x + 640.0, 360.0 - p.y)
}

struct Viewport {
    scale: f32,
    width: usize,
    height: usize,
    left: usize,
    top: usize,
}

impl Viewport {
    fn new(width: usize, height: usize) -> Self {
        let scale = (width as f32 / WIDTH as f32).min(height as f32 / HEIGHT as f32);
        let draw_width = ((WIDTH as f32 * scale) as usize).min(width);
        let draw_height = ((HEIGHT as f32 * scale) as usize).min(height);
        Self {
            scale,
            width: draw_width,
            height: draw_height,
            left: (width - draw_width) / 2,
            top: (height - draw_height) / 2,
        }
    }
}

pub fn unfit(p: Vec2, width: u32, height: u32) -> Vec2 {
    let v = Viewport::new(width as usize, height as usize);
    if v.scale == 0.0 {
        return Vec2::ZERO;
    }
    (p - Vec2::new(v.left as f32, v.top as f32)) / v.scale
}

pub fn fit(frame: &[u32], buffer: &mut [u32], width: usize, height: usize) {
    let v = Viewport::new(width, height);
    buffer.fill(0x0d0c17);
    for y in 0..v.height {
        for x in 0..v.width {
            let sx = ((x as f32 / v.scale) as usize).min(WIDTH - 1);
            let sy = ((y as f32 / v.scale) as usize).min(HEIGHT - 1);
            buffer[(y + v.top) * width + x + v.left] = frame[sy * WIDTH + sx];
        }
    }
}

pub fn save_ppm(frame: &[u32], path: &str) -> io::Result<()> {
    let mut file = io::BufWriter::new(File::create(path)?);
    write!(file, "P6\n{WIDTH} {HEIGHT}\n255\n")?;
    for &c in frame {
        file.write_all(&[(c >> 16) as u8, (c >> 8) as u8, c as u8])?;
    }
    Ok(())
}

struct Canvas {
    pixels: Vec<u32>,
}
impl Canvas {
    fn new() -> Self {
        Self {
            pixels: vec![0x161523; WIDTH * HEIGHT],
        }
    }
    fn dot(&mut self, x: i32, y: i32, color: u32) {
        if x >= 0 && y >= 0 && x < WIDTH as i32 && y < HEIGHT as i32 {
            self.pixels[y as usize * WIDTH + x as usize] = color;
        }
    }
    fn rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: u32) {
        for py in y.max(0)..(y + h).min(HEIGHT as i32) {
            for px in x.max(0)..(x + w).min(WIDTH as i32) {
                self.dot(px, py, color);
            }
        }
    }
    fn circle(&mut self, p: Vec2, r: f32, color: u32) {
        let radius = r.ceil() as i32;
        for y in -radius..=radius {
            for x in -radius..=radius {
                if (x * x + y * y) as f32 <= r * r {
                    self.dot(p.x as i32 + x, p.y as i32 + y, color);
                }
            }
        }
    }
    fn line(&mut self, a: Vec2, b: Vec2, width: f32, color: u32) {
        let steps = a.distance(b).ceil().max(1.0) as i32;
        for i in 0..=steps {
            self.circle(a.lerp(b, i as f32 / steps as f32), width / 2.0, color);
        }
    }
    fn ring(&mut self, center: Vec2, r: f32, width: f32, color: u32) {
        for i in 0..96 {
            let a = i as f32 * std::f32::consts::TAU / 96.0;
            let b = (i + 1) as f32 * std::f32::consts::TAU / 96.0;
            self.line(
                center + Vec2::new(a.cos(), a.sin()) * r,
                center + Vec2::new(b.cos(), b.sin()) * r,
                width,
                color,
            );
        }
    }
    fn text(&mut self, x: i32, y: i32, text: &str, scale: i32, color: u32) {
        for (index, letter) in text.to_uppercase().chars().enumerate() {
            let rows = glyph(letter);
            for (row, bits) in rows.iter().enumerate() {
                for col in 0..5 {
                    if bits & (1 << (4 - col)) != 0 {
                        self.rect(
                            x + index as i32 * 6 * scale + col * scale,
                            y + row as i32 * scale,
                            scale,
                            scale,
                            color,
                        );
                    }
                }
            }
        }
    }
}

pub fn draw(state: &Snapshot, cursor: Vec2, focused: bool) -> Vec<u32> {
    draw_with_backend(state, cursor, focused, "BEVY ECS / OFFLINE TEST WINDOW")
}

pub fn draw_with_backend(state: &Snapshot, cursor: Vec2, focused: bool, backend: &str) -> Vec<u32> {
    draw_session(state, cursor, focused, backend, 0, true, "SOLO PRACTICE")
}

#[allow(clippy::too_many_arguments)]
pub fn draw_session(
    state: &Snapshot,
    cursor: Vec2,
    focused: bool,
    backend: &str,
    local_slot: u8,
    practice: bool,
    status: &str,
) -> Vec<u32> {
    let mut c = Canvas::new();
    for y in (140..650).step_by(40) {
        c.rect(24, y, 1232, 1, 0x211f33);
    }
    for x in (40..1260).step_by(40) {
        c.rect(x, 128, 1, 530, 0x211f33);
    }
    c.text(44, 154, "01 / MOVEMENT + QUICK CASTS", 2, 0x54506c);
    c.text(44, 178, "DESKTOP TEST ARENA", 1, 0x54506c);

    // All geometry below is disposable placeholder art, independent of collision.
    for p in &state.platforms {
        let top = screen(p.center + Vec2::new(-p.half.x, p.half.y));
        c.rect(
            top.x as i32,
            top.y as i32,
            (p.half.x * 2.0) as i32,
            (p.half.y * 2.0) as i32,
            0x37334e,
        );
        c.rect(
            top.x as i32,
            top.y as i32,
            (p.half.x * 2.0) as i32,
            4,
            0x8e83b2,
        );
        c.rect(
            top.x as i32 + 5,
            top.y as i32 + 8,
            (p.half.x * 2.0) as i32 - 10,
            2,
            0x49435f,
        );
    }
    for effect in &state.effects {
        let p = screen(effect.position);
        let progress = 1.0 - effect.left / effect.duration;
        match effect.kind {
            EffectKind::Explosion => {
                c.ring(p, effect.radius * (0.45 + progress * 0.55), 3.0, FIRE);
                c.ring(p, effect.radius * 0.5 * (1.0 - progress), 2.0, 0xffe4a8);
            }
            EffectKind::Gust { direction } => {
                let facing = -direction.y.atan2(direction.x);
                for radius in [0.35, 0.65, 1.0] {
                    let r = effect.radius * radius * (0.5 + progress * 0.5);
                    for i in 0..32 {
                        let a = facing - 1.2 + i as f32 / 32.0 * 2.4;
                        let b = facing - 1.2 + (i + 1) as f32 / 32.0 * 2.4;
                        c.line(
                            p + Vec2::new(a.cos(), a.sin()) * r,
                            p + Vec2::new(b.cos(), b.sin()) * r,
                            2.0,
                            GUST,
                        );
                    }
                }
            }
        }
    }
    for projectile in &state.projectiles {
        let p = screen(projectile.position);
        let trail = Vec2::new(-projectile.velocity.x, projectile.velocity.y).normalize_or_zero();
        let color = if projectile.reflected { GUST } else { FIRE };
        c.line(p + trail * 22.0, p, 5.0, color);
        c.circle(p, 9.0, color);
        c.circle(p, 4.0, 0xffefcd);
    }
    for view in &state.actors {
        let actor = &view.actor;
        let p = screen(view.body.position);
        let color = if actor.slot == 0 { BLUE } else { 0xf0918b };
        if !actor.alive() {
            c.rect(p.x as i32 - 22, p.y as i32 + 12, 44, 12, 0x514661);
            c.text(
                p.x as i32 - 54,
                p.y as i32 - 50,
                &format!("RESPAWN {:.1}", actor.respawn_left),
                1,
                INK,
            );
            continue;
        }
        let color = if actor.hit_flash > 0.0 {
            0xffffff
        } else {
            color
        };
        if actor.protection > 0.0 {
            c.ring(p, 39.0, 1.0, 0x727d9c);
        }
        if actor.dash_left > 0.0 {
            let trail = Vec2::new(-actor.dash_direction.x, actor.dash_direction.y);
            c.line(p + trail * 62.0, p, 15.0, 0x4c436e);
        }
        c.rect(p.x as i32 - 16, p.y as i32 - 10, 32, 37, color);
        c.rect(p.x as i32 - 12, p.y as i32 - 27, 24, 18, 0xf0c9a8);
        // A small hat stays inside the collision silhouette.
        for row in 0..12 {
            c.rect(
                p.x as i32 - row,
                p.y as i32 - 27 + row / 2,
                row * 2 + 1,
                1,
                color,
            );
        }
        c.rect(p.x as i32 - 18, p.y as i32 - 19, 36, 4, color);
        c.rect(p.x as i32 - 16, p.y as i32 + 11, 32, 4, 0x3d304f);
        c.rect(p.x as i32 - 3, p.y as i32 + 11, 6, 4, FIRE);
        let eye_x = if actor.aim.x >= 0.0 { 5 } else { -9 };
        c.rect(p.x as i32 + eye_x, p.y as i32 - 12, 4, 4, 0x242032);
        let hand = p + Vec2::new(actor.aim.x, -actor.aim.y) * 23.0;
        c.circle(hand, 4.0, 0xf0c9a8);
        let label = if actor.slot == local_slot {
            "YOU"
        } else if practice {
            "TRAINING DUMMY"
        } else if actor.slot == 0 {
            "PLAYER 1"
        } else {
            "PLAYER 2"
        };
        c.text(
            p.x as i32 - label.len() as i32 * 3,
            p.y as i32 - 58,
            label,
            1,
            color,
        );
        if actor.gust_launch_left > 0.0 {
            c.text(p.x as i32 - 24, p.y as i32 - 72, "LAUNCHED", 1, GUST);
        } else if actor.slow_left > 0.0 {
            c.text(
                p.x as i32 - 33,
                p.y as i32 - 72,
                &format!("SLOWED {:.1}S", actor.slow_left),
                1,
                GUST,
            );
        }
        if actor.slot == local_slot || actor.health_reveal > 0.0 {
            c.rect(p.x as i32 - 25, p.y as i32 - 43, 50, 4, 0x3d354f);
            c.rect(
                p.x as i32 - 25,
                p.y as i32 - 43,
                (50.0 * (actor.health / state.tuning.health).clamp(0.0, 1.0)) as i32,
                4,
                color,
            );
        }
        if actor.slot == local_slot {
            let dir = Vec2::new(actor.aim.x, -actor.aim.y);
            for step in [42.0, 55.0, 68.0] {
                c.circle(p + dir * step, 1.0, 0xaaa1c9);
            }
        }
    }
    c.line(cursor - Vec2::X * 8.0, cursor + Vec2::X * 8.0, 1.0, INK);
    c.line(cursor - Vec2::Y * 8.0, cursor + Vec2::Y * 8.0, 1.0, INK);
    c.ring(cursor, 4.0, 1.0, INK);

    c.rect(0, 0, 1280, 116, 0x201d30);
    c.rect(0, 115, 1280, 1, 0x49405f);
    c.text(32, 20, "WILD MAGIC ACADEMY", 3, INK);
    c.text(34, 52, "CORE GAMEPLAY LAB  /  BUILD 0.1", 1, MUTED);
    let Some(local) = state
        .actors
        .iter()
        .find(|v| v.actor.slot == local_slot)
        .map(|v| &v.actor)
    else {
        c.text(34, 83, "NO LOCAL PLAYER", 2, MUTED);
        return c.pixels;
    };
    let spell_color = if local.spell == Spell::Fireball {
        FIRE
    } else {
        GUST
    };
    c.text(34, 83, &format!("HP {:03.0}", local.health), 2, BLUE);
    c.text(
        180,
        83,
        if local.spell == Spell::Fireball {
            "1 FIREBALL"
        } else {
            "2 GUST"
        },
        2,
        spell_color,
    );
    c.text(
        385,
        86,
        &format!(
            "CAST {:.1}S  DASH {:.1}S",
            local.cooldown, local.dash_cooldown
        ),
        1,
        MUTED,
    );
    c.text(850, 27, backend, 1, MUTED);
    if local.gust_launch_left > 0.0 {
        c.text(600, 86, "LAUNCHED", 1, GUST);
    } else if local.slow_left > 0.0 {
        c.text(600, 86, &format!("SLOWED {:.1}S", local.slow_left), 1, GUST);
    }
    c.text(
        385,
        101,
        if local.air_dash_available {
            "AIR DASH: READY"
        } else {
            "AIR DASH: LAND TO RECHARGE"
        },
        1,
        MUTED,
    );
    let firing = state.actors.iter().any(|a| a.dummy_firing == Some(true));
    let dummy_spell = state
        .actors
        .iter()
        .find(|a| a.dummy_firing.is_some())
        .map(|a| a.actor.spell)
        .unwrap_or_default();
    if practice {
        c.text(
            850,
            51,
            &format!(
                "DUMMY {}: {} [T]",
                if dummy_spell == Spell::Gust {
                    "GUST"
                } else {
                    "FIREBALL"
                },
                if firing { "ON" } else { "OFF" }
            ),
            2,
            if firing { FIRE } else { GUST },
        );
    } else {
        c.text(
            850,
            51,
            &status.chars().take(62).collect::<String>(),
            1,
            GUST,
        );
    }
    c.text(
        850,
        83,
        &format!(
            "HITS {}   RETURNS {}   RESPAWNS {}",
            state.diagnostics.hits, state.diagnostics.returns, state.diagnostics.respawns
        ),
        1,
        MUTED,
    );

    c.rect(0, 664, 1280, 56, 0x201d30);
    c.rect(0, 664, 1280, 1, 0x49405f);
    c.text(
        32,
        678,
        "A/D MOVE   SPACE DOUBLE JUMP   SHIFT DASH + WASD DIRECTION   MOUSE AIM   CLICK/F CAST",
        1,
        INK,
    );
    c.text(
        32,
        698,
        if practice { "1/2 SELECT SPELL   Q/RIGHT CLICK SWITCH   T DUMMY FIRE   G DUMMY SPELL   R RESET   ESC QUIT" }
        else { "1/2 SELECT SPELL   Q/RIGHT CLICK SWITCH   ESC LEAVE SESSION   HOST RESOLVES ALL COMBAT" },
        1,
        MUTED,
    );
    c.text(
        1100,
        689,
        &format!("{:.0} HZ CORE", 1.0 / state.tuning.tick),
        1,
        GUST,
    );
    if !focused {
        c.rect(420, 300, 440, 75, 0x29243b);
        c.text(448, 320, "PAUSED - FOCUS WINDOW", 3, INK);
    }
    c.pixels
}

fn glyph(c: char) -> [u8; 7] {
    match c {
        'A' => [14, 17, 17, 31, 17, 17, 17],
        'B' => [30, 17, 17, 30, 17, 17, 30],
        'C' => [14, 17, 16, 16, 16, 17, 14],
        'D' => [30, 17, 17, 17, 17, 17, 30],
        'E' => [31, 16, 16, 30, 16, 16, 31],
        'F' => [31, 16, 16, 30, 16, 16, 16],
        'G' => [14, 17, 16, 23, 17, 17, 15],
        'H' => [17, 17, 17, 31, 17, 17, 17],
        'I' => [31, 4, 4, 4, 4, 4, 31],
        'J' => [7, 2, 2, 2, 18, 18, 12],
        'K' => [17, 18, 20, 24, 20, 18, 17],
        'L' => [16, 16, 16, 16, 16, 16, 31],
        'M' => [17, 27, 21, 21, 17, 17, 17],
        'N' => [17, 25, 21, 19, 17, 17, 17],
        'O' => [14, 17, 17, 17, 17, 17, 14],
        'P' => [30, 17, 17, 30, 16, 16, 16],
        'Q' => [14, 17, 17, 17, 21, 18, 13],
        'R' => [30, 17, 17, 30, 20, 18, 17],
        'S' => [15, 16, 16, 14, 1, 1, 30],
        'T' => [31, 4, 4, 4, 4, 4, 4],
        'U' => [17, 17, 17, 17, 17, 17, 14],
        'V' => [17, 17, 17, 17, 17, 10, 4],
        'W' => [17, 17, 17, 21, 21, 21, 10],
        'X' => [17, 17, 10, 4, 10, 17, 17],
        'Y' => [17, 17, 10, 4, 4, 4, 4],
        'Z' => [31, 1, 2, 4, 8, 16, 31],
        '0' => [14, 17, 19, 21, 25, 17, 14],
        '1' => [4, 12, 4, 4, 4, 4, 14],
        '2' => [14, 17, 1, 2, 4, 8, 31],
        '3' => [30, 1, 1, 14, 1, 1, 30],
        '4' => [2, 6, 10, 18, 31, 2, 2],
        '5' => [31, 16, 16, 30, 1, 1, 30],
        '6' => [14, 16, 16, 30, 17, 17, 14],
        '7' => [31, 1, 2, 4, 8, 8, 8],
        '8' => [14, 17, 17, 14, 17, 17, 14],
        '9' => [14, 17, 17, 15, 1, 1, 14],
        '/' => [1, 2, 2, 4, 8, 8, 16],
        '-' => [0, 0, 0, 31, 0, 0, 0],
        '+' => [0, 4, 4, 31, 4, 4, 0],
        '.' => [0, 0, 0, 0, 0, 12, 12],
        ':' => [0, 12, 12, 0, 12, 12, 0],
        '[' => [14, 8, 8, 8, 8, 8, 14],
        ']' => [14, 2, 2, 2, 2, 2, 14],
        _ => [0; 7],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wild_magic_core::{Simulation, Tuning, model::Actor};

    #[test]
    fn rendered_pixels_and_aim_use_the_same_letterbox_transform() {
        let frame: Vec<_> = (0..WIDTH * HEIGHT).map(|i| i as u32).collect();
        for (width, height) in [(1001, 800), (1600, 601), (801, 451)] {
            let mut buffer = vec![0; width * height];
            fit(&frame, &mut buffer, width, height);
            let v = Viewport::new(width, height);
            for (x, y) in [
                (v.left, v.top),
                (width / 2, height / 2),
                (v.left + v.width - 1, v.top + v.height - 1),
            ] {
                let p = unfit(Vec2::new(x as f32, y as f32), width as u32, height as u32);
                let expected = frame[p.y as usize * WIDTH + p.x as usize];
                assert_eq!(buffer[y * width + x], expected);
            }
        }
        assert_eq!(unfit(Vec2::ZERO, 0, 0), Vec2::ZERO);
    }

    #[test]
    fn health_bars_follow_tuning_instead_of_assuming_one_hundred_hp() {
        let mut sim = Simulation::new(Tuning {
            health: 200.0,
            ..Default::default()
        });
        sim.world.get_mut::<Actor>(sim.local_player).unwrap().health = 100.0;
        let frame = draw(&sim.snapshot(), Vec2::ZERO, true);
        assert_eq!(frame[550 * WIDTH + 419], BLUE);
        assert_eq!(frame[550 * WIDTH + 420], 0x3d354f);
    }

    #[test]
    fn rendering_tolerates_a_missing_local_actor() {
        let mut snapshot = Simulation::default().snapshot();
        snapshot.actors.retain(|actor| actor.actor.slot != 0);
        assert_eq!(draw(&snapshot, Vec2::ZERO, true).len(), WIDTH * HEIGHT);
    }
}
