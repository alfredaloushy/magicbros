use crate::{
    protocol::{DEFAULT_PORT, Frame, Packet, VERSION},
    transport::{Connection, invalid},
};
use std::{
    io,
    net::{IpAddr, SocketAddr, TcpListener, TcpStream},
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};
use wild_magic_core::{
    Simulation,
    client::{BufferedInput, FixedClock},
    model::{Actor, PlayerCommand, Snapshot},
};

const TIMEOUT: Duration = Duration::from_secs(5);
const INPUT_TIMEOUT: Duration = Duration::from_millis(250);

#[derive(Clone, Debug, PartialEq)]
pub enum Mode {
    Practice,
    Host(SocketAddr),
    Join(SocketAddr),
}

impl Mode {
    pub fn parse(args: impl IntoIterator<Item = String>) -> Result<Self, String> {
        let mut args = args.into_iter();
        match args.next().as_deref() {
            None => Ok(Self::Practice),
            Some("--host") => {
                let address = args
                    .next()
                    .unwrap_or_else(|| format!("0.0.0.0:{DEFAULT_PORT}"));
                if args.next().is_some() {
                    return Err("unexpected arguments after --host address".into());
                }
                Ok(Self::Host(
                    address
                        .parse()
                        .map_err(|_| "host address must be IP:PORT")?,
                ))
            }
            Some("--join") => {
                let address = args
                    .next()
                    .ok_or("--join requires the host IP or IP:PORT")?;
                if args.next().is_some() {
                    return Err("unexpected arguments after --join address".into());
                }
                let address = address
                    .parse::<SocketAddr>()
                    .or_else(|_| {
                        address
                            .parse::<IpAddr>()
                            .map(|ip| SocketAddr::new(ip, DEFAULT_PORT))
                    })
                    .map_err(|_| "use the host IP, optionally with :PORT")?;
                Ok(Self::Join(address))
            }
            _ => Err(
                "usage: --host [IP:PORT] or --join HOST_IP[:PORT]; omit both for practice".into(),
            ),
        }
    }
}

enum State {
    Practice(Box<Simulation>),
    Host(Box<Host>),
    Client(Box<Client>),
    Ended,
}

pub struct Session {
    state: State,
    clock: FixedClock,
    view: Snapshot,
    slot: u8,
    status: String,
    practice: bool,
}

impl Session {
    pub fn new(mode: Mode) -> io::Result<Self> {
        let practice = mode == Mode::Practice;
        let mut simulation = if practice {
            Simulation::default()
        } else {
            Simulation::duel()
        };
        let view = simulation.snapshot();
        let (state, slot, status) = match mode {
            Mode::Practice => (
                State::Practice(Box::new(simulation)),
                0,
                "SOLO PRACTICE".into(),
            ),
            Mode::Host(address) => {
                let listener = TcpListener::bind(address)?;
                listener.set_nonblocking(true)?;
                let bound = listener.local_addr()?;
                (
                    State::Host(Box::new(Host {
                        listener,
                        simulation,
                        peer: None,
                        ready: false,
                        remote: RemoteControls::default(),
                        seen: Instant::now(),
                    })),
                    0,
                    format!("HOST WAITING / {bound}"),
                )
            }
            Mode::Join(address) => {
                let connect = thread::spawn(move || TcpStream::connect_timeout(&address, TIMEOUT));
                (
                    State::Client(Box::new(Client {
                        connect: Some(connect),
                        peer: None,
                        welcomed: false,
                        sequence: 0,
                        send_time: 0.0,
                        seen: Instant::now(),
                        last_tick: None,
                    })),
                    1,
                    format!("JOINING {address}"),
                )
            }
        };
        Ok(Self {
            state,
            clock: FixedClock::default(),
            view,
            slot,
            status,
            practice,
        })
    }

    pub fn view(&self) -> &Snapshot {
        &self.view
    }
    pub fn slot(&self) -> u8 {
        self.slot
    }
    pub fn status(&self) -> &str {
        &self.status
    }
    pub fn is_practice(&self) -> bool {
        self.practice
    }
    pub fn ended(&self) -> bool {
        matches!(self.state, State::Ended)
    }
    pub fn connected(&self) -> bool {
        match &self.state {
            State::Host(h) => h.ready,
            State::Client(c) => c.welcomed && c.last_tick.is_some(),
            _ => false,
        }
    }
    pub fn host_address(&self) -> Option<SocketAddr> {
        if let State::Host(h) = &self.state {
            h.listener.local_addr().ok()
        } else {
            None
        }
    }

    pub fn reset_practice(&mut self) {
        if self.practice {
            *self = Self::new(Mode::Practice).unwrap();
        }
    }
    pub fn toggle_dummy(&mut self) {
        if let State::Practice(s) = &mut self.state {
            s.toggle_dummy();
        }
    }
    pub fn toggle_dummy_spell(&mut self) {
        if let State::Practice(s) = &mut self.state {
            s.toggle_dummy_spell();
        }
    }

    pub fn update(&mut self, seconds: f64, focused: bool, input: &mut BufferedInput) {
        if !focused {
            *input = BufferedInput::default();
        }
        let result = match &mut self.state {
            State::Practice(s) => {
                if focused {
                    for _ in 0..self.clock.advance(seconds, self.view.tuning.tick as f64) {
                        s.step(input.take_tick());
                    }
                } else {
                    self.clock = FixedClock::default();
                }
                self.view = s.snapshot();
                Ok(())
            }
            State::Host(h) => h.update(
                seconds,
                input,
                &mut self.clock,
                &mut self.view,
                &mut self.status,
            ),
            State::Client(c) => c.update(seconds, input, &mut self.view, &mut self.status),
            State::Ended => Ok(()),
        };
        if let Err(error) = result {
            eprintln!("LAN session ended: {error}");
            self.status = format!("SESSION ENDED: {error}");
            self.state = State::Ended; // Drop sockets; never silently fall back to local combat.
            *input = BufferedInput::default();
        }
    }
}

#[derive(Default)]
pub struct RemoteControls {
    command: PlayerCommand,
    cast_pending: bool,
    pub sequence: u64,
    generation: u32,
    last_input: Option<Instant>,
}

impl RemoteControls {
    pub fn receive(
        &mut self,
        sequence: u64,
        generation: u32,
        command: PlayerCommand,
        current_generation: u32,
    ) {
        if sequence <= self.sequence {
            return;
        }
        self.sequence = sequence;
        if generation != current_generation {
            self.clear();
            return;
        }
        if generation != self.generation {
            self.clear();
        }
        let mut command = command.sanitized(self.command.aim_world);
        command.jump |= self.command.jump;
        command.dash |= self.command.dash;
        command.switch ^= self.command.switch;
        command.select = command.select.or(self.command.select);
        self.cast_pending |= command.cast;
        self.command = command;
        self.generation = generation;
        self.last_input = Some(Instant::now());
    }

    pub fn take_tick(&mut self, current_generation: u32) -> PlayerCommand {
        if current_generation != self.generation
            || self.last_input.is_none_or(|t| t.elapsed() > INPUT_TIMEOUT)
        {
            self.clear();
        }
        let mut command = self.command;
        command.cast |= self.cast_pending;
        self.command.clear_edges();
        self.cast_pending = false;
        command
    }

    fn clear(&mut self) {
        self.command = PlayerCommand::default();
        self.cast_pending = false;
        self.last_input = None;
    }
}

struct Host {
    listener: TcpListener,
    simulation: Simulation,
    peer: Option<Connection>,
    ready: bool,
    remote: RemoteControls,
    seen: Instant,
}

impl Host {
    fn update(
        &mut self,
        seconds: f64,
        input: &mut BufferedInput,
        clock: &mut FixedClock,
        view: &mut Snapshot,
        status: &mut String,
    ) -> io::Result<()> {
        // Exactly one remote actor; extra connections cannot claim a player slot.
        for _ in 0..8 {
            match self.listener.accept() {
                Ok((stream, _)) if self.peer.is_none() => {
                    self.peer = Some(Connection::new(stream)?);
                    self.seen = Instant::now();
                }
                Ok((stream, _)) => drop(stream),
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => break,
                Err(e) => return Err(e),
            }
        }
        let Some(peer) = &mut self.peer else {
            *input = BufferedInput::default();
            return Ok(());
        };
        for packet in peer.poll()? {
            self.seen = Instant::now();
            match packet {
                Packet::Hello { version: VERSION } if !self.ready => {
                    peer.send(&Packet::Welcome {
                        version: VERSION,
                        slot: 1,
                    })?;
                    self.ready = true;
                    *input = BufferedInput::default();
                    *status = "HOST CONNECTED / PLAYER 1".into();
                    println!("{status}");
                    peer.send(&Packet::State(Box::new(Frame::from_snapshot(view, 0))))?;
                }
                Packet::Input {
                    sequence,
                    generation,
                    command,
                } if self.ready => {
                    let current = self
                        .simulation
                        .world
                        .get::<Actor>(self.simulation.dummy)
                        .unwrap()
                        .generation;
                    self.remote.receive(sequence, generation, command, current);
                }
                _ => return Err(invalid("incompatible or unexpected client message")),
            }
        }
        if self.seen.elapsed() > TIMEOUT {
            return Err(invalid("client timed out"));
        }
        if !self.ready {
            return Ok(());
        }
        for _ in 0..clock.advance(seconds, view.tuning.tick as f64) {
            let generation = self
                .simulation
                .world
                .get::<Actor>(self.simulation.dummy)
                .unwrap()
                .generation;
            self.simulation
                .step_players([input.take_tick(), self.remote.take_tick(generation)]);
            if self
                .simulation
                .world
                .resource::<wild_magic_core::model::Diagnostics>()
                .tick
                .is_multiple_of(4)
            {
                let snapshot = self.simulation.snapshot();
                peer.send(&Packet::State(Box::new(Frame::from_snapshot(
                    &snapshot,
                    self.remote.sequence,
                ))))?;
            }
        }
        *view = self.simulation.snapshot();
        Ok(())
    }
}

struct Client {
    connect: Option<JoinHandle<io::Result<TcpStream>>>,
    peer: Option<Connection>,
    welcomed: bool,
    sequence: u64,
    send_time: f64,
    seen: Instant,
    last_tick: Option<u64>,
}

impl Client {
    fn update(
        &mut self,
        seconds: f64,
        input: &mut BufferedInput,
        view: &mut Snapshot,
        status: &mut String,
    ) -> io::Result<()> {
        if self.connect.as_ref().is_some_and(|task| task.is_finished()) {
            let stream = self
                .connect
                .take()
                .unwrap()
                .join()
                .map_err(|_| invalid("connection worker failed"))??;
            let mut peer = Connection::new(stream)?;
            peer.send(&Packet::Hello { version: VERSION })?;
            self.peer = Some(peer);
            self.seen = Instant::now();
        }
        let Some(peer) = &mut self.peer else {
            return Ok(());
        };
        for packet in peer.poll()? {
            self.seen = Instant::now();
            match packet {
                Packet::Welcome {
                    version: VERSION,
                    slot: 1,
                } if !self.welcomed => {
                    self.welcomed = true;
                    *status = "CLIENT CONNECTED / PLAYER 2".into();
                    println!("{status}");
                }
                Packet::State(frame) if self.welcomed => {
                    if frame.players.len() != 2
                        || frame.players[0].actor.slot != 0
                        || frame.players[1].actor.slot != 1
                        || frame.shots.len() > 128
                        || frame.effects.len() > 128
                        || frame.platforms.len() > 64
                        || frame.tuning.validate().is_err()
                    {
                        return Err(invalid("invalid host state"));
                    }
                    if self.last_tick.is_none_or(|tick| frame.tick > tick) {
                        self.last_tick = Some(frame.tick);
                        *view = frame.view();
                    }
                }
                Packet::End { reason } => return Err(invalid(reason)),
                _ => return Err(invalid("incompatible or unexpected host message")),
            }
        }
        if self.seen.elapsed() > TIMEOUT {
            return Err(invalid("host timed out"));
        }
        self.send_time += seconds.clamp(0.0, 0.1);
        if self.welcomed && self.send_time >= 1.0 / 60.0 {
            self.send_time %= 1.0 / 60.0;
            self.sequence += 1;
            let generation = view
                .actors
                .iter()
                .find(|a| a.actor.slot == 1)
                .unwrap()
                .actor
                .generation;
            peer.send(&Packet::Input {
                sequence: self.sequence,
                generation,
                command: input.take_tick(),
            })?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stalled_remote_input_clears_held_controls_and_pending_edges() {
        let mut remote = RemoteControls::default();
        remote.receive(
            1,
            0,
            PlayerCommand {
                movement: glam::Vec2::X,
                cast: true,
                jump: true,
                dash: true,
                ..Default::default()
            },
            0,
        );
        remote.last_input = Some(Instant::now() - INPUT_TIMEOUT - Duration::from_millis(1));
        let command = remote.take_tick(0);
        assert_eq!(command.movement, glam::Vec2::ZERO);
        assert!(!command.cast && !command.jump && !command.dash);
    }
}
