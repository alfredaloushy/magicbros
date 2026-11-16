mod cli;
mod render;

use glam::Vec2;
use softbuffer::{Context, Surface};
use std::{
    collections::HashSet,
    error::Error,
    num::NonZeroU32,
    sync::Arc,
    time::{Duration, Instant},
};
use wild_magic_core::{
    Simulation,
    client::{BufferedInput, FixedClock},
    model::{PlayerCommand, Spell},
};
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::{ElementState, MouseButton, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};

struct Desktop {
    simulation: Simulation,
    window: Option<Arc<Window>>,
    context: Option<Context<Arc<Window>>>,
    surface: Option<Surface<Arc<Window>, Arc<Window>>>,
    keys: HashSet<KeyCode>,
    input: BufferedInput,
    cursor_window: Vec2,
    mouse_cast: bool,
    clock: FixedClock,
    last_frame: Instant,
    next_frame: Instant,
    frames: u32,
    smoke_frames: Option<u32>,
    focused: bool,
    failure: Option<String>,
}

impl Desktop {
    fn new(smoke_frames: Option<u32>) -> Self {
        Self {
            simulation: Simulation::default(),
            window: None,
            context: None,
            surface: None,
            keys: HashSet::new(),
            input: BufferedInput::default(),
            cursor_window: Vec2::new(860.0, 593.0),
            mouse_cast: false,
            clock: FixedClock::default(),
            last_frame: Instant::now(),
            next_frame: Instant::now(),
            frames: 0,
            smoke_frames,
            focused: true,
            failure: None,
        }
    }

    fn key(&mut self, key: KeyCode, pressed: bool, repeat: bool, event_loop: &ActiveEventLoop) {
        if pressed {
            self.keys.insert(key);
        } else {
            self.keys.remove(&key);
        }
        if !pressed || repeat {
            return;
        }
        match key {
            KeyCode::Escape => event_loop.exit(),
            KeyCode::Space => self.input.command.jump = true,
            KeyCode::ShiftLeft | KeyCode::ShiftRight => self.input.command.dash = true,
            KeyCode::KeyQ => self.input.command.switch = true,
            KeyCode::Digit1 => self.input.command.select = Some(Spell::Fireball),
            KeyCode::Digit2 => self.input.command.select = Some(Spell::Gust),
            KeyCode::KeyF => self.input.press_cast(),
            KeyCode::KeyT => self.simulation.toggle_dummy(),
            KeyCode::KeyG => self.simulation.toggle_dummy_spell(),
            KeyCode::KeyR => {
                self.simulation = Simulation::default();
                self.input = BufferedInput::default();
                self.clock = FixedClock::default();
            }
            _ => (),
        }
    }

    fn redraw(&mut self, event_loop: &ActiveEventLoop) -> Result<(), Box<dyn Error>> {
        let window = self.window.as_ref().unwrap();
        let size = window.inner_size();
        let cursor = render::unfit(self.cursor_window, size.width, size.height);
        let now = Instant::now();
        let dt = now.duration_since(self.last_frame).as_secs_f64().min(0.1);
        self.last_frame = now;
        self.next_frame = now + Duration::from_secs_f64(1.0 / 60.0);
        if size.width == 0 || size.height == 0 {
            self.clock = FixedClock::default();
            return Ok(());
        }
        if self.focused {
            let down = |a, b| (self.keys.contains(&a) || self.keys.contains(&b)) as u8 as f32;
            self.input.command.movement = Vec2::new(
                down(KeyCode::KeyD, KeyCode::ArrowRight) - down(KeyCode::KeyA, KeyCode::ArrowLeft),
                down(KeyCode::KeyW, KeyCode::ArrowUp) - down(KeyCode::KeyS, KeyCode::ArrowDown),
            );
            self.input.held_cast = self.mouse_cast || self.keys.contains(&KeyCode::KeyF);
            self.input.command.aim_world = render::screen_to_world(cursor);
            let tick = self
                .simulation
                .world
                .resource::<wild_magic_core::Tuning>()
                .tick as f64;
            for _ in 0..self.clock.advance(dt, tick) {
                self.simulation.step(self.input.take_tick());
            }
        } else {
            self.clock = FixedClock::default();
        }
        let frame = render::draw(&self.simulation.snapshot(), cursor, self.focused);
        if let (Some(width), Some(height)) =
            (NonZeroU32::new(size.width), NonZeroU32::new(size.height))
        {
            let surface = self.surface.as_mut().unwrap();
            surface.resize(width, height)?;
            let mut buffer = surface.buffer_mut()?;
            render::fit(
                &frame,
                &mut buffer,
                size.width as usize,
                size.height as usize,
            );
            buffer.present()?;
        }
        // A smoke frame counts only after successful presentation to the surface.
        self.frames += 1;
        if self.smoke_frames.is_some_and(|limit| self.frames >= limit) {
            println!(
                "Desktop smoke passed: {} presented frames, {} simulation ticks",
                self.frames,
                self.simulation.snapshot().diagnostics.tick
            );
            event_loop.exit();
        }
        Ok(())
    }
}

impl ApplicationHandler for Desktop {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        let result = (|| -> Result<(), Box<dyn Error>> {
            let window = Arc::new(
                event_loop.create_window(
                    Window::default_attributes()
                        .with_title("Wild Magic Academy | Core Gameplay Lab")
                        .with_inner_size(LogicalSize::new(1280.0, 720.0))
                        .with_min_inner_size(LogicalSize::new(800.0, 450.0)),
                )?,
            );
            let context = Context::new(window.clone())?;
            let surface = Surface::new(&context, window.clone())?;
            self.context = Some(context);
            self.surface = Some(surface);
            self.window = Some(window);
            self.last_frame = Instant::now();
            Ok(())
        })();
        if let Err(error) = result {
            self.failure = Some(format!("Cannot open desktop test window: {error}"));
            event_loop.exit();
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Focused(focused) => {
                self.focused = focused;
                self.keys.clear();
                self.mouse_cast = false;
                self.input = BufferedInput::default();
                self.clock = FixedClock::default();
                self.last_frame = Instant::now();
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(key) = event.physical_key {
                    self.key(
                        key,
                        event.state == ElementState::Pressed,
                        event.repeat,
                        event_loop,
                    );
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.cursor_window = Vec2::new(position.x as f32, position.y as f32);
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if button == MouseButton::Left {
                    self.mouse_cast = state == ElementState::Pressed;
                    if self.mouse_cast {
                        self.input.press_cast();
                    }
                }
                if button == MouseButton::Right && state == ElementState::Pressed {
                    self.input.command.switch = true;
                }
            }
            WindowEvent::RedrawRequested => {
                if let Err(error) = self.redraw(event_loop) {
                    self.failure = Some(format!("Frame presentation failed: {error}"));
                    event_loop.exit();
                }
            }
            _ => (),
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if let Some(window) = &self.window
            && Instant::now() >= self.next_frame
        {
            window.request_redraw();
        }
        event_loop.set_control_flow(ControlFlow::WaitUntil(self.next_frame));
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let options = cli::Options::parse(std::env::args().skip(1))?;
    if options.help {
        println!(
            "Wild Magic Academy — core gameplay lab\n\nUsage: wild-magic-desktop [--x11] [--smoke [FRAMES]]\n       wild-magic-desktop --snapshot OUTPUT.ppm\n\n--smoke checks a real desktop window. --snapshot needs no display.\n--x11 selects X11 on Linux when the configured Wayland compositor is unavailable."
        );
        return Ok(());
    }
    if let Some(path) = options.snapshot {
        let mut simulation = Simulation::default();
        for _ in 0..150 {
            simulation.step(PlayerCommand::default());
        }
        let frame = render::draw(&simulation.snapshot(), Vec2::new(860.0, 593.0), true);
        render::save_ppm(&frame, &path)?;
        println!("Saved prototype snapshot to {path}");
        return Ok(());
    }
    let smoke_frames = options.smoke_frames;
    let mut desktop = Desktop::new(smoke_frames);
    let mut event_loop = EventLoop::builder();
    #[cfg(target_os = "linux")]
    if options.x11 {
        use winit::platform::x11::EventLoopBuilderExtX11;
        event_loop.with_x11();
    }
    event_loop.build().map_err(|e| format!("Cannot connect to desktop display: {e}. Run in a desktop session, or use --snapshot OUTPUT.ppm for a display-free check."))?
        .run_app(&mut desktop)?;
    if let Some(error) = desktop.failure {
        return Err(error.into());
    }
    if smoke_frames.is_some_and(|limit| desktop.frames < limit) {
        return Err("desktop smoke interrupted before enough frames were presented".into());
    }
    Ok(())
}
