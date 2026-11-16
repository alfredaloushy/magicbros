//! Full Bevy window/renderer adapter.
//! The disposable CPU preview is uploaded as one texture; future art replaces it.
use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    window::PrimaryWindow,
};
use wild_magic_core::{
    Simulation,
    client::{BufferedInput, FixedClock},
    model::Spell,
};

#[allow(dead_code)]
#[path = "../../desktop/src/render.rs"]
mod render;

#[derive(Resource)]
struct Lab {
    simulation: Simulation,
    input: BufferedInput,
    clock: FixedClock,
    cursor: glam::Vec2,
}

#[derive(Resource)]
struct PreviewImage(Handle<Image>);

type LabView<'w, 's> = (
    Single<'w, 's, &'static Window, With<PrimaryWindow>>,
    Single<'w, 's, (&'static Camera, &'static GlobalTransform)>,
);

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Wild Magic Academy | Bevy Core Gameplay Lab".into(),
                        resolution: (1280, 720).into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .insert_resource(Lab {
            simulation: Simulation::default(),
            input: BufferedInput::default(),
            clock: FixedClock::default(),
            cursor: glam::Vec2::new(860.0, 593.0),
        })
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    // Fixed virtual canvas keeps the prototype's aim and collision units consistent.
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: bevy::camera::ScalingMode::AutoMin {
                min_width: 1280.0,
                min_height: 720.0,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
    let image = Image::new_fill(
        Extent3d {
            width: 1280,
            height: 720,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[22, 21, 35, 255],
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );
    let handle = images.add(image);
    commands.spawn(Sprite::from_image(handle.clone()));
    commands.insert_resource(PreviewImage(handle));
}

fn update(
    time: Res<Time>,
    controls: (Res<ButtonInput<KeyCode>>, Res<ButtonInput<MouseButton>>),
    view: LabView,
    mut lab: ResMut<Lab>,
    preview: Res<PreviewImage>,
    mut images: ResMut<Assets<Image>>,
    mut exit: MessageWriter<AppExit>,
) {
    let (keys, mouse) = controls;
    let (window, camera) = view;
    if keys.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
    if !window.focused {
        lab.input = BufferedInput::default();
        lab.clock = FixedClock::default();
    } else {
        if keys.just_pressed(KeyCode::KeyR) {
            lab.simulation = Simulation::default();
            lab.input = BufferedInput::default();
            lab.clock = FixedClock::default();
        }
        if keys.just_pressed(KeyCode::KeyT) {
            lab.simulation.toggle_dummy();
        }
        if keys.just_pressed(KeyCode::KeyG) {
            lab.simulation.toggle_dummy_spell();
        }
        if let Some(cursor) = window.cursor_position()
            && let Ok(world) = camera.0.viewport_to_world_2d(camera.1, cursor)
        {
            lab.cursor = glam::Vec2::new(world.x + 640.0, 360.0 - world.y);
        }
        let down = |a, b| (keys.pressed(a) || keys.pressed(b)) as u8 as f32;
        lab.input.command.movement = glam::Vec2::new(
            down(KeyCode::KeyD, KeyCode::ArrowRight) - down(KeyCode::KeyA, KeyCode::ArrowLeft),
            down(KeyCode::KeyW, KeyCode::ArrowUp) - down(KeyCode::KeyS, KeyCode::ArrowDown),
        );
        lab.input.command.aim_world = render::screen_to_world(lab.cursor);
        lab.input.held_cast = mouse.pressed(MouseButton::Left) || keys.pressed(KeyCode::KeyF);
        if mouse.just_pressed(MouseButton::Left) || keys.just_pressed(KeyCode::KeyF) {
            lab.input.press_cast();
        }
        lab.input.command.jump |= keys.just_pressed(KeyCode::Space);
        lab.input.command.dash |=
            keys.just_pressed(KeyCode::ShiftLeft) || keys.just_pressed(KeyCode::ShiftRight);
        lab.input.command.switch |=
            keys.just_pressed(KeyCode::KeyQ) || mouse.just_pressed(MouseButton::Right);
        if keys.just_pressed(KeyCode::Digit1) {
            lab.input.command.select = Some(Spell::Fireball);
        }
        if keys.just_pressed(KeyCode::Digit2) {
            lab.input.command.select = Some(Spell::Gust);
        }
        let tick = lab
            .simulation
            .world
            .resource::<wild_magic_core::Tuning>()
            .tick as f64;
        let ticks = lab.clock.advance(time.delta_secs_f64(), tick);
        for _ in 0..ticks {
            let input = lab.input.take_tick();
            lab.simulation.step(input);
        }
    }
    let cursor = lab.cursor;
    let frame = render::draw_with_backend(
        &lab.simulation.snapshot(),
        cursor,
        window.focused,
        "BEVY 0.18 / RENDERER ADAPTER",
    );
    if let Some(image) = images.get_mut(&preview.0)
        && let Some(data) = image.data.as_mut()
    {
        for (pixel, color) in data.as_chunks_mut::<4>().0.iter_mut().zip(frame) {
            pixel.copy_from_slice(&[(color >> 16) as u8, (color >> 8) as u8, color as u8, 255]);
        }
    }
}
