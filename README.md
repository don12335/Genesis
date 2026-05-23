# Genesis: Neuroevolution & Pathfinding Simulation

Genesis is an advanced neuroevolution framework designed to compute and optimize pathfinding policies for autonomous agents traversing dynamically generated environments. The architecture integrates a high-throughput, Rust-based parallel processing engine for offline training with a deterministic, physics-based visualization interface deployed in the browser.

## Core Features
- **High-Performance Computation**: Utilizes a Rust backend powered by `rayon` to execute evolutionary cycles in parallel, achieving convergence over thousands of generations in minimal time.
- **Physics-Based Visualization**: Incorporates a 2D physics engine (`matter.js`) for the deterministic simulation and visual analysis of agent behavior and trajectory optimization.
- **Sensor-Driven Navigation**: Agents process environmental telemetry via 5 simulated LiDAR raycast sensors, identifying topological constraints in real time.
- **Evolutionary Optimization**: Employs genetic algorithms—specifically uniform crossover and tournament selection—to iteratively optimize the weights and biases of a Feedforward Neural Network, mapping sensor inputs to motor outputs.

## Deployment & Usage

### 1. Offline Training Phase (CLI Engine)
The offline training interface provides rapid convergence of the neural network policy.
1. Compile and execute the Rust environment: `cargo run --release`
2. Select **Train Neural Network** from the interactive prompt.
3. Specify the computational threshold (e.g., 500 generations).
4. Upon convergence, the serialized network topology and weight configuration will be exported to `visualizer/dna.js`.

### 2. Analytical Visualization (Web Interface)
The web visualization environment renders the learned policy and environmental interactions.
1. Initiate the visualizer directly from the CLI environment or launch `visualizer/index.html` locally.
2. Pre-computed topological weights are dynamically ingested into the physics environment.
3. Select **Start Evolution** to begin real-time rendering of the agent's spatial navigation.

## Repository Architecture
- `src/`: Core simulation logic, genetic algorithm implementation, and CLI interface written in Rust.
- `visualizer/`: Visualization logic, sensory rendering, and deterministic physics execution written in JavaScript.
