/// Entry point for Squash RL.
///
/// Parses the run mode from the command line and delegates to the
/// appropriate module. All game and training logic lives elsewhere.
mod game;
mod agent;
mod manual;
mod trainer;

fn main()
{
    let args: Vec<String> = std::env::args().collect();
    let mode: &str        = if args.len() > 1 { &args[1] } else { "" };

    match mode
    {
        "manual" =>
        {
            let mut window = game::render::WindowState::new();
            manual::run(&mut window);
        }
        "agent" =>
        {
            let mut window = game::render::WindowState::new();
            let mut agent  = agent::Agent::new();
            trainer::run(&mut window, &mut agent);
            agent.save();
        }
        _ => eprintln!("Usage: squash-rl [manual|agent]"),
    }
}
