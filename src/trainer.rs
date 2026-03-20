/// Agent training mode.
///
/// Runs the Q-learning training loop: the agent plays episodes, receives rewards,
/// updates the Q-table, and decays epsilon over time.
use crate::game::{self, GameState};
use crate::agent::Agent;

/// Calculates the reward for a single frame transition.
///
/// Returns:
///  -1.0  — ball was missed (game over)
///  +1.0+ — ball hit the racket (scaled by current score)
///  small — ball is falling and agent is roughly aligned (shaping signal)
///   0.0  — ball is rising, no learning signal needed
fn reward(prev: &GameState, curr: &GameState) -> f32
{
    if !curr.active
    {
        // Ball missed: strong negative signal
        return -1.0;
    }

    if (prev.ball_speed.speed_y < 0) && (curr.ball_speed.speed_y > 0)
    {
        // Racket hit: speed_y flipped from downward to upward
        return 1.0 + curr.score as f32 * 0.1;
    }

    if curr.ball_speed.speed_y < 0
    {
        // Ball is falling: small alignment reward to guide tracking.
        // Scaled to 0..0.005 so it nudges without overshadowing hit/miss.
        let dist = (curr.ball_pos.x as i32 - curr.racket.racket_position.x as i32).abs() as f32;
        return (1.0 - dist / game::FIELD_WIDTH as f32).max(0.0) * 0.005;
    }

    0.0 // Ball is rising: nothing to reinforce
}

/// Runs the full training session until the window is closed or Escape is pressed.
///
/// Prints average and max score every 50 000 episodes and renders one episode
/// visually at each milestone so training progress is visible.
pub fn run(window: &mut game::render::WindowState, agent: &mut Agent)
{
    let mut current  = GameState::new();
    let mut previous = GameState::new();
    let mut action: game::Action = game::Action::Stay;
    let mut episode: u32 = 0;

    // Rolling stats reset at each display milestone
    let mut score_sum:   i64 = 0;
    let mut score_count: u32 = 0;
    let mut max_score:   i32 = 0;
    let mut last_display_episode: u32 = u32::MAX;

    // Time-based OS event poll: minifb needs update() to flush key state.
    // Without this, is_key_down() never sees ESC between visual draws.
    let mut last_poll = std::time::Instant::now();

    current.active = true;

    while window.window.is_open()
    {
        // Keep window responsive (ESC, close button) even at full training speed
        if last_poll.elapsed() >= std::time::Duration::from_millis(100)
        {
            window.window.update();
            last_poll = std::time::Instant::now();
        }

        match game::input::read_input(&window.window)
        {
            game::input::Action::Stop => break,
            _ =>
            {
                action = agent.decide(&current);
                match action
                {
                    game::Action::Left  => current.move_racket(game::Action::Left),
                    game::Action::Right => current.move_racket(game::Action::Right),
                    _                   => {}
                }
            }
        }

        current.update();

        let r = reward(&previous, &current);
        agent.learn(&previous, &action, r, &current);

        if !current.active
        {
            score_sum   += current.score as i64;
            score_count += 1;
            if current.score > max_score { max_score = current.score; }

            current = GameState::new();
            current.active = true;
            episode += 1;

            agent.epsilon = (agent.epsilon * 0.995).max(0.001);
        }

        previous = current.clone();

        // Visual milestone: render one full episode + print stats every 50k episodes
        if episode % 50000 == 0
        {
            window.draw(&current);

            if episode != last_display_episode && score_count > 0
            {
                last_display_episode = episode;
                let avg = score_sum as f32 / score_count as f32;
                print!("\x1B[2J\x1B[1;1H");
                println!("Episode: {:>8},  Epsilon: {:.3},  Avg Score: {:.1},  Max Score: {}",
                         episode, agent.epsilon, avg, max_score);
                score_sum   = 0;
                score_count = 0;
                max_score   = 0;
            }
        }
    }
}
