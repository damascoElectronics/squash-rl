# Arcade III: Squash RL

A terminal-based simulation of the classic TV Arcade III game "Squash", implemented in Rust. This repository serves as an educational environment for applying Artificial Intelligence, specifically a **Q-learning Reinforcement Learning (RL)** model, to learn and play the game autonomously.

## Overview

The goal of this project is to recreate the game dynamics of the retro Arcade III: Squash and train an intelligent agent to master it. The project implements a custom physics engine and rendering loop from scratch using `minifb` for lightweight window management, alongside a discrete Q-learning implementation.

### Features
* **Two Play Modes:**
  * **Manual Mode:** Take control of the racket using your keyboard.
  * **Agent Mode:** Watch the Q-learning agent train in real-time or play using its pre-trained memory.
* **Custom Physics Engine:** Handles ball movement, racket collisions, and screen boundaries.
* **Reinforcement Learning Agent:** A discrete state-space Q-learning agent capable of saving and loading its acquired knowledge (`qtable.bin`).
* **Visual Rendering:** A simple, high-performance pixel-buffer rendering system using `minifb`.

---

## Installation

### Prerequisites
You will need the Rust toolchain installed. If you don't have Rust installed, get it from [rustup.rs](https://rustup.rs/).

```bash
# Clone the repository
git clone https://github.com/yourusername/squash-rl.git
cd squash-rl

# Build the project
cargo build --release
```

## Usage

The executable takes a single command-line argument to determine the runtime mode.

### 1. Manual Play
To play the game yourself using the keyboard (Left/Right arrows, Escape to quit):
```bash
cargo run --release -- manual
```


### 2. Autonomous Agent
To watch the Reinforcement Learning agent play (and train):
```bash
cargo run --release -- agent
```
*Note: The agent will continuously learn as it plays. Every 100 episodes, it will output its current training metrics (Episode, Epsilon, Score) to the terminal.*

---

## Technical Details

### Project Structure
* `src/main.rs`: The application entry point, argument parser, and main loops.
* `src/game/mod.rs`: The core game state math, including physics and collision detection.
* `src/game/render.rs`: The visual pixel-buffer manager.
* `src/game/input.rs`: Keyboard input parsing.
* `src/agent/mod.rs`: The Reinforcement Learning agent and Q-table logic.

### Reinforcement Learning Implementation
The AI uses a **Q-learning algorithm** with an **epsilon-greedy exploration strategy**.
To make the continuous environment solvable, the game state is discretized into 40 distinct zones across the X and Y axes (`NUM_ZONES = 40`).

The state space consists of a tuple: 
`(zone_x_ball, zone_y_ball, dir_x_ball, dir_y_ball, zone_x_racket)`
* **Reward Structure:**
  * `+1 Rweard`: Successfully bouncing the ball upwards.
  * `-1 Reward`: Missing the ball (Game Over).
  * `0 Reward`: Intermediate moves.

## License

This project is licensed under the terms found in the `LICENSE` file.
