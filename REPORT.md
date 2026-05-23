# Genesis Project Report: Maze Escape Neuroevolution

## Executive Summary
The Genesis project has successfully transitioned into a full-fledged Neuroevolution platform. The core objective is to demonstrate the principles of Genetic Algorithms and Neural Networks by training an AI agent to solve a dynamically generated 16x10 maze. The system provides both a high-speed command-line trainer and an intuitive web visualization, making complex AI concepts accessible and easy to understand for everyone.

## Architecture & System Design
The project is divided into two synergistic components:
1. **Rust CLI Trainer**: A high-performance command-line application that runs the evolution engine. It uses `rayon` for parallel processing, simulating thousands of agents in seconds without graphical overhead.
2. **Web Visualizer**: A browser-based frontend powered by `matter.js`. It visually replicates the exact physics and logic of the Rust engine, allowing users to watch the evolved brains in action.

## The AI Model (Brain & Sensors)
- **Inputs (Sensors)**: The agent utilizes 5 raycast sensors spread in an arc in front of it to detect the distance to nearby walls, along with its relative position to the next checkpoint.
- **Neural Network**: A Feedforward Neural Network serves as the brain. It contains:
  - 7 Input nodes (5 sensors + X/Y distance to target)
  - 6 Hidden nodes
  - 2 Output nodes (X and Y thrust/forces)
- **DNA (Genotype)**: The agent's DNA is a continuous array of 62 floating-point numbers representing the weights and biases of the neural network.

## The Evolution Engine
The biological evolution principles are applied as follows:
1. **Fitness Function**: The maze is solved using a Breadth-First Search (BFS) algorithm to establish checkpoints. Agents are rewarded based on how many checkpoints they reach and how close they get to the next one.
2. **Selection & Crossover**: The best-performing agents (the top 30%) are selected as parents. We use Uniform Crossover to combine their DNA, producing offspring for the next generation.
3. **Mutation**: A 5% mutation rate introduces slight variations to the neural network weights, allowing the population to discover new paths and avoid getting stuck.

## Conclusion
By integrating a fast Rust backend with an accessible web frontend, Genesis serves as a highly effective and professional educational tool for understanding AI. The separation of the training engine and the visualizer ensures both high performance during computation and an excellent user experience during observation.
