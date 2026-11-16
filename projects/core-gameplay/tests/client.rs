use wild_magic_core::client::{BufferedInput, FixedClock};

#[test]
fn press_release_between_ticks_still_casts_once() {
    let mut input = BufferedInput::default();
    input.press_cast();
    input.held_cast = false;
    assert!(input.take_tick().cast);
    assert!(!input.take_tick().cast);
}

#[test]
fn catchup_ticks_repeat_held_actions_but_not_edges() {
    let mut input = BufferedInput::default();
    input.held_cast = true;
    input.command.jump = true;
    input.command.switch = true;
    let first = input.take_tick();
    let second = input.take_tick();
    assert!(first.cast && first.jump && first.switch);
    assert!(second.cast && !second.jump && !second.switch);
}

#[test]
fn input_edges_wait_for_a_tick_and_focus_reset_clears_them() {
    let mut input = BufferedInput::default();
    let mut clock = FixedClock::default();
    input.command.jump = true;
    input.press_cast();
    assert_eq!(clock.advance(0.001, 1.0 / 120.0), 0);
    assert!(input.command.jump);
    input = BufferedInput::default(); // Focus loss/reset clears pending and held state.
    let command = input.take_tick();
    assert!(!command.cast && !command.jump);
}

#[test]
fn fixed_ticks_do_not_depend_on_render_rate() {
    for fps in [30, 60, 144, 240] {
        let mut clock = FixedClock::default();
        let ticks: usize = (0..fps)
            .map(|_| clock.advance(1.0 / fps as f64, 1.0 / 120.0))
            .sum();
        assert!(
            (119..=120).contains(&ticks),
            "{fps} fps produced {ticks} ticks"
        );
    }
}

#[test]
fn stalls_are_bounded_and_invalid_frame_times_do_not_poison_clock() {
    let mut clock = FixedClock::default();
    assert_eq!(clock.advance(f64::NAN, 1.0 / 120.0), 0);
    assert_eq!(clock.advance(-1.0, 1.0 / 120.0), 0);
    assert_eq!(clock.advance(0.016, 0.0), 0);
    assert_eq!(clock.advance(1000.0, 1.0 / 120.0), 12);
    assert_eq!(clock.advance(1.0 / 120.0, 1.0 / 120.0), 1);
    assert_eq!(clock.advance(0.1, 1e-100), 240);
}
