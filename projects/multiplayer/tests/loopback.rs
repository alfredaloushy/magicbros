//! Real TCP tests. Require permission to bind/connect to loopback sockets.
use glam::Vec2;
use std::{
    collections::BTreeMap,
    io::Write,
    net::{TcpListener, TcpStream},
    thread,
    time::{Duration, Instant},
};
use wild_magic_core::{
    client::BufferedInput,
    model::{Snapshot, Spell},
};
use wild_magic_lan::{
    Mode, Session,
    protocol::{Packet, VERSION},
    transport::{Connection, MAX_FRAME},
};

struct Pair {
    host: Session,
    client: Session,
    host_input: BufferedInput,
    client_input: BufferedInput,
    states: BTreeMap<u64, Snapshot>,
}

impl Pair {
    fn new() -> Self {
        let host = Session::new(Mode::Host("127.0.0.1:0".parse().unwrap())).unwrap();
        let client = Session::new(Mode::Join(host.host_address().unwrap())).unwrap();
        let mut pair = Self {
            host,
            client,
            host_input: BufferedInput::default(),
            client_input: BufferedInput::default(),
            states: BTreeMap::new(),
        };
        let deadline = Instant::now() + Duration::from_secs(3);
        while !pair.client.connected() {
            pair.step();
            assert!(!pair.host.ended() && !pair.client.ended());
            assert!(
                Instant::now() < deadline,
                "connection handshake did not finish"
            );
            thread::sleep(Duration::from_millis(1));
        }
        pair
    }

    fn step(&mut self) {
        self.client
            .update(1.0 / 120.0, true, &mut self.client_input);
        self.host.update(1.0 / 120.0, true, &mut self.host_input);
        self.states
            .insert(self.host.view().diagnostics.tick, self.host.view().clone());
        self.client.update(0.0, true, &mut self.client_input);
        thread::yield_now();
    }

    fn pump(&mut self, ticks: usize) {
        for tick in 0..ticks {
            self.step();
            if tick % 8 == 0 {
                thread::sleep(Duration::from_millis(1));
            }
            assert!(!self.host.ended(), "{}", self.host.status());
            assert!(!self.client.ended(), "{}", self.client.status());
        }
    }

    fn assert_agreement(&self) {
        let client = self.client.view();
        let host = self.states.get(&client.diagnostics.tick).unwrap();
        for slot in 0..2 {
            assert_eq!(
                serde_json::to_value(&host.actors[slot].actor).unwrap(),
                serde_json::to_value(&client.actors[slot].actor).unwrap()
            );
            assert_eq!(
                serde_json::to_value(&host.actors[slot].body).unwrap(),
                serde_json::to_value(&client.actors[slot].body).unwrap()
            );
        }
        assert_eq!(host.projectiles.len(), client.projectiles.len());
        for (a, b) in host.projectiles.iter().zip(&client.projectiles) {
            assert_eq!(a.id, b.id);
            assert_eq!(a.position, b.position);
            assert_eq!(a.reflected, b.reflected);
        }
    }
}

#[test]
fn real_peers_move_cast_and_agree_on_health_defeat_and_respawn() {
    let mut pair = Pair::new();
    assert_eq!(pair.client.slot(), 1);
    pair.host_input.command.movement = Vec2::X;
    pair.client_input.command.jump = true;
    pair.client_input.command.select = Some(Spell::Gust);
    pair.pump(40);
    assert!(pair.host.view().actors[0].body.position.x > -160.0);
    assert!(pair.host.view().actors[1].body.position.y > -233.0);
    assert_eq!(pair.client.view().actors[1].actor.spell, Spell::Gust);
    pair.assert_agreement();
    pair.host_input.command.movement = Vec2::ZERO;
    pair.host_input.held_cast = true;
    let (mut saw_damage, mut saw_death, mut saw_respawn) = (false, false, false);
    for tick in 0..1800 {
        pair.host_input.command.aim_world = pair.host.view().actors[1].body.position;
        pair.step();
        let actor = &pair.client.view().actors[1].actor;
        saw_damage |= actor.health < 100.0;
        saw_death |= !actor.alive();
        saw_respawn |= actor.generation > 0 && actor.alive();
        if tick % 8 == 0 {
            thread::sleep(Duration::from_millis(1));
        }
        if saw_respawn {
            break;
        }
    }
    assert!(
        saw_damage && saw_death && saw_respawn,
        "combat loop did not replicate"
    );
    pair.assert_agreement();
}

#[test]
fn real_gust_launch_slow_and_return_are_authoritative_on_both_views() {
    let mut pair = Pair::new();
    pair.client_input.command.movement = Vec2::NEG_X;
    pair.pump(110);
    pair.client_input.command.movement = Vec2::ZERO;
    pair.pump(65);
    let x = pair.host.view().actors[1].body.position.x;
    assert!(
        (-100.0..=20.0).contains(&x),
        "unexpected remote position {x}"
    );
    pair.host_input.command.select = Some(Spell::Gust);
    pair.host_input.command.aim_world = pair.host.view().actors[1].body.position;
    pair.host_input.press_cast();
    let start_y = pair.host.view().actors[1].body.position.y;
    pair.pump(10);
    assert!(pair.client.view().actors[1].actor.slow_left > 0.0);
    assert!(pair.client.view().actors[1].actor.gust_launch_left > 0.0);
    assert!(pair.client.view().actors[1].body.position.y > start_y);
    pair.assert_agreement();
    pair.client_input.held_cast = true;
    pair.client_input.command.select = Some(Spell::Fireball);
    pair.host_input.held_cast = true;
    for tick in 0..1800 {
        pair.client_input.command.aim_world = pair.host.view().actors[0].body.position;
        pair.host_input.command.aim_world = pair.host.view().actors[1].body.position;
        pair.step();
        if tick % 8 == 0 {
            thread::sleep(Duration::from_millis(1));
        }
        if pair.client.view().diagnostics.returns > 0 {
            break;
        }
    }
    assert!(
        pair.client.view().diagnostics.returns > 0,
        "no reflected Fireball reached the client"
    );
    pair.assert_agreement();
}

#[test]
fn host_keeps_ticking_without_focus_and_disconnect_ends_client() {
    let mut pair = Pair::new();
    let before = pair.host.view().diagnostics.tick;
    for _ in 0..20 {
        pair.host.update(1.0 / 60.0, false, &mut pair.host_input);
        pair.client.update(1.0 / 60.0, true, &mut pair.client_input);
    }
    assert!(pair.host.view().diagnostics.tick > before + 20);
    drop(pair.host);
    for _ in 0..50 {
        pair.client.update(0.01, true, &mut pair.client_input);
        if pair.client.ended() {
            break;
        }
        thread::sleep(Duration::from_millis(1));
    }
    assert!(pair.client.ended());
}

#[test]
fn client_disconnect_ends_host_and_third_player_is_rejected() {
    let mut pair = Pair::new();
    let mut extra = Session::new(Mode::Join(pair.host.host_address().unwrap())).unwrap();
    let mut extra_input = BufferedInput::default();
    for _ in 0..100 {
        pair.step();
        extra.update(0.01, true, &mut extra_input);
        if extra.ended() {
            break;
        }
        thread::sleep(Duration::from_millis(1));
    }
    assert!(extra.ended());
    assert!(pair.host.connected());
    drop(pair.client);
    for _ in 0..50 {
        pair.host.update(0.01, true, &mut pair.host_input);
        if pair.host.ended() {
            break;
        }
        thread::sleep(Duration::from_millis(1));
    }
    assert!(pair.host.ended());
}

#[test]
fn incompatible_protocol_is_rejected() {
    let mut host = Session::new(Mode::Host("127.0.0.1:0".parse().unwrap())).unwrap();
    let stream = TcpStream::connect(host.host_address().unwrap()).unwrap();
    let mut bad = Connection::new(stream).unwrap();
    bad.send(&Packet::Hello {
        version: VERSION + 1,
    })
    .unwrap();
    for _ in 0..50 {
        host.update(0.01, true, &mut BufferedInput::default());
        if host.ended() {
            break;
        }
        thread::sleep(Duration::from_millis(1));
    }
    assert!(host.ended());
}

#[test]
fn framing_handles_fragmented_coalesced_messages_and_rejects_oversize() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let mut writer = TcpStream::connect(listener.local_addr().unwrap()).unwrap();
    let mut reader = Connection::new(listener.accept().unwrap().0).unwrap();
    let payload = serde_json::to_vec(&Packet::Hello { version: VERSION }).unwrap();
    let mut frame = (payload.len() as u32).to_be_bytes().to_vec();
    frame.extend(payload);
    writer.write_all(&frame[..2]).unwrap();
    assert!(reader.poll().unwrap().is_empty());
    writer.write_all(&frame[2..]).unwrap();
    writer.write_all(&frame).unwrap();
    let mut count = 0;
    for _ in 0..50 {
        count += reader.poll().unwrap().len();
        if count == 2 {
            break;
        }
        thread::sleep(Duration::from_millis(1));
    }
    assert_eq!(count, 2);
    writer
        .write_all(&((MAX_FRAME + 1) as u32).to_be_bytes())
        .unwrap();
    let mut rejected = false;
    for _ in 0..50 {
        if reader.poll().is_err() {
            rejected = true;
            break;
        }
        thread::sleep(Duration::from_millis(1));
    }
    assert!(rejected);
}
