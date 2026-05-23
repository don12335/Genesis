# Genesis Neuroevolution Trainer

Genesis is a neuroevolution platform designed to train AI agents to solve dynamically generated mazes. It combines a high-performance Rust CLI trainer with a visual, physics-based Web interface, making it easy for both developers and laypeople to understand how neural networks and genetic algorithms work.

## Features
- **High-Performance CLI Trainer**: A fast, Rust-based backend that simulates thousands of generations in seconds using parallel processing.
- **Web Visualizer**: A 2D physics visualization built with `matter.js` that allows you to watch the trained brains (or untrained ones) navigate the maze in real-time.
- **Sensor-Based AI**: Each agent is equipped with 5 "raycast" sensors (similar to LiDAR) to detect walls and navigate.
- **Neuroevolution**: Agents evolve using natural selection—only the ones that get closest to the exit survive and pass their "DNA" (neural network weights) to the next generation.

## Getting Started

### 1. Training an Agent (CLI Tool)
The Genesis CLI tool is the fastest way to train a smart agent.
1. Run the `genesis` executable (or build the Rust project via `cargo run --release`).
2. Select **Train Neural Network** from the menu.
3. Choose your training intensity (e.g., 500 generations).
4. The CLI will quickly evolve the agents and save the best brain to `visualizer/dna.js`.

### 2. Watching the Evolution (Web Visualizer)
You can view the pre-trained agent or watch the evolution happen live in your browser:
1. Open the CLI and select **Open Web Visualizer**, or manually open `visualizer/index.html` in your web browser.
2. If you've trained an agent in the CLI, the web visualizer will automatically load its brain.
3. Click **Start Evolution** to watch the simulation!

## Project Structure
- `src/`: The Rust backend and CLI tool for fast evolutionary training.
- `visualizer/`: The HTML/JS frontend that visualizes the agents and the maze.
