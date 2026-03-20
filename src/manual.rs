/// Manual (human) game mode.
///
/// Runs the game in a window driven entirely by keyboard input.
use crate::game;

/// Starts a human-controlled game session.
///
/// Reads keyboard input each frame, updates physics, and renders until
/// the player presses Escape or misses the ball.
pub fn run(window: &mut game::render::WindowState)
{
    let mut state = game::GameState::new();
    state.active = true;

    while window.window.is_open()
    {
        match game::input::read_input(&window.window)
        {
            game::input::Action::Stop  => break,
            game::input::Action::Left  => state.move_racket(game::Action::Left),
            game::input::Action::Right => state.move_racket(game::Action::Right),
            _                          => {}
        }

        state.update();
        window.draw(&state);

        if !state.active { break; }
    }
}
