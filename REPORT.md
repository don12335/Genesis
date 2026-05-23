# Genesis Project Architecture Report

## Executive Summary
The Genesis project functions as a comprehensive neuroevolution simulation framework, formulated to demonstrate the optimization capabilities of Genetic Algorithms paired with Neural Networks. The platform trains autonomous agents to resolve dynamically structured geometric mazes. By decoupling the computational training pipeline from the visual rendering environment, the system achieves both computational efficiency and high-fidelity analytical feedback.

## System Architecture
The framework operates on a bifurcated architecture:
1. **Computational Engine (Rust)**: A headless, multi-threaded application tasked with the execution of the evolutionary algorithm. Utilizing `rayon` for data parallelism, it evaluates the fitness landscape of large agent populations concurrently, bypassing graphical rendering overhead during the optimization phase.
2. **Visualization Engine (JavaScript)**: A deterministic 2D physics environment utilizing `matter.js`. This component parses the serialized genomic data (neural network parameters) produced by the computational engine, enabling real-time qualitative analysis of the agent's behavioral policy.

## Model Formulation
- **Telemetry Processing (Sensors)**: The agent observes the state space through an array of 5 simulated raycast sensors. These sensors compute the intersection distance with solid geometry, which is concatenated with the normalized vector delta to the current topological checkpoint.
- **Neural Topology**: The autonomous policy is governed by a Feedforward Neural Network architecture:
  - Input Layer: 7 nodes (5 raycast readings + 2 spatial deltas).
  - Hidden Layer: 6 nodes utilizing a hyperbolic tangent (tanh) activation function.
  - Output Layer: 2 nodes specifying Cartesian force vectors.
- **Genomic Representation**: The parametric state of the neural network is serialized into a continuous, 62-dimensional floating-point array, serving as the evolutionary genotype.

## Evolutionary Mechanics
The optimization heuristic applies the following principles:
1. **Fitness Evaluation**: Topological traversal is quantified using a Breadth-First Search (BFS) to map the optimal path. The fitness function calculates a scalar reward proportional to the agent's progression along discrete waypoints.
2. **Selection and Recombination**: The algorithm employs an elitism-based selection protocol, isolating the upper 30% of the population curve. Uniform crossover operators recombine parametric data from successful candidates to synthesize the subsequent generation.
3. **Stochastic Mutation**: A 5% mutation probability introduces parametric noise, facilitating exploration of the policy space and preventing convergence on local optima.

## Conclusion
The Genesis framework successfully implements a robust neuroevolutionary pipeline. The segregation of the high-throughput Rust processing engine and the qualitative JavaScript visualization environment results in a highly scalable and analytically rigorous simulation platform.
