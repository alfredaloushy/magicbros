use glam::Vec2;
use wild_magic_core::{
    Simulation,
    model::{PlayerCommand, Spell},
};
use wild_magic_lan::{
    Mode,
    protocol::{Frame, Packet},
    session::RemoteControls,
};

#[test]
fn remote_edges_are_consumed_once_and_held_movement_persists() {
    let mut remote = RemoteControls::default();
    remote.receive(
        1,
        0,
        PlayerCommand {
            movement: Vec2::X,
            jump: true,
            switch: true,
            cast: true,
            ..Default::default()
        },
        0,
    );
    let first = remote.take_tick(0);
    let second = remote.take_tick(0);
    assert!(first.jump && first.switch && first.cast);
    assert!(!second.jump && !second.switch && second.cast);
    assert_eq!(second.movement, Vec2::X);
}

#[test]
fn short_cast_is_preserved_when_press_and_release_arrive_together() {
    let mut remote = RemoteControls::default();
    remote.receive(
        1,
        0,
        PlayerCommand {
            cast: true,
            ..Default::default()
        },
        0,
    );
    remote.receive(2, 0, PlayerCommand::default(), 0);
    assert!(remote.take_tick(0).cast);
    assert!(!remote.take_tick(0).cast);
}

#[test]
fn duplicate_sequence_and_old_respawn_generation_cannot_replay_actions() {
    let mut remote = RemoteControls::default();
    remote.receive(
        5,
        0,
        PlayerCommand {
            jump: true,
            ..Default::default()
        },
        0,
    );
    assert!(remote.take_tick(0).jump);
    remote.receive(
        5,
        0,
        PlayerCommand {
            jump: true,
            ..Default::default()
        },
        0,
    );
    remote.receive(
        4,
        0,
        PlayerCommand {
            dash: true,
            ..Default::default()
        },
        0,
    );
    assert!(!remote.take_tick(0).jump);
    remote.receive(
        6,
        0,
        PlayerCommand {
            cast: true,
            movement: Vec2::X,
            ..Default::default()
        },
        1,
    );
    let command = remote.take_tick(1);
    assert!(!command.cast && !command.dash);
    assert_eq!(command.movement, Vec2::ZERO);
    remote.receive(
        7,
        1,
        PlayerCommand {
            jump: true,
            ..Default::default()
        },
        1,
    );
    assert!(remote.take_tick(1).jump);
}

#[test]
fn new_generation_discards_pending_actions_and_movement_is_sanitized() {
    let mut remote = RemoteControls::default();
    remote.receive(
        1,
        0,
        PlayerCommand {
            dash: true,
            ..Default::default()
        },
        0,
    );
    remote.receive(
        2,
        1,
        PlayerCommand {
            movement: Vec2::new(20.0, f32::NAN),
            ..Default::default()
        },
        1,
    );
    let command = remote.take_tick(1);
    assert!(!command.dash);
    assert_eq!(command.movement, Vec2::X);
}

#[test]
fn two_real_players_have_independent_commands_and_no_dummy() {
    let mut simulation = Simulation::duel();
    simulation.step_players([
        PlayerCommand {
            movement: Vec2::X,
            ..Default::default()
        },
        PlayerCommand {
            jump: true,
            select: Some(Spell::Gust),
            ..Default::default()
        },
    ]);
    let snapshot = simulation.snapshot();
    assert!(snapshot.actors.iter().all(|a| a.dummy_firing.is_none()));
    assert!(snapshot.actors[0].body.position.x > -220.0);
    assert!(snapshot.actors[1].body.position.y > -233.0);
    assert_eq!(snapshot.actors[1].actor.spell, Spell::Gust);
}

#[test]
fn snapshots_roundtrip_all_combat_state_without_ecs_entity_ids() {
    let mut simulation = Simulation::duel();
    simulation.step_players([
        PlayerCommand {
            cast: true,
            ..Default::default()
        },
        PlayerCommand::default(),
    ]);
    let frame = Frame::from_snapshot(&simulation.snapshot(), 42);
    let json = serde_json::to_vec(&Packet::State(Box::new(frame))).unwrap();
    assert!(!String::from_utf8_lossy(&json).contains("entity"));
    let Packet::State(frame) = serde_json::from_slice(&json).unwrap() else {
        panic!("missing state")
    };
    assert_eq!(frame.acknowledged_input, 42);
    assert_eq!(frame.shots.len(), 1);
    assert_eq!(frame.shots[0].id, 1);
    assert_eq!(frame.shots[0].owner_slot, 0);
    assert_eq!(frame.shots[0].original_slot, 0);
    let view = frame.view();
    assert_eq!(view.projectiles[0].id, 1);
    assert_eq!(view.tuning.gust_launch_time, 0.2);
    assert!(view.actors[1].actor.air_dash_available);
}

#[test]
fn host_and_join_arguments_are_unambiguous() {
    let args = |s: &[&str]| s.iter().map(|s| s.to_string()).collect::<Vec<_>>();
    assert_eq!(Mode::parse(args(&[])).unwrap(), Mode::Practice);
    assert!(matches!(
        Mode::parse(args(&["--host"])).unwrap(),
        Mode::Host(_)
    ));
    assert_eq!(
        Mode::parse(args(&["--join", "127.0.0.1"])).unwrap(),
        Mode::Join("127.0.0.1:24560".parse().unwrap())
    );
    for input in [
        vec!["--join"],
        vec!["--join", "not-an-ip"],
        vec!["--host", "--join"],
        vec!["--join", "127.0.0.1", "--host"],
    ] {
        assert!(Mode::parse(args(&input)).is_err());
    }
}
