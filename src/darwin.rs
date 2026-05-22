use crate::vm::{AstVM, Opcode};
use rand::prelude::*;
use rayon::prelude::*;

pub const SEQUENCE_LENGTH: usize = 128;
pub const POPULATION_SIZE: usize = 2000;

#[derive(Clone)]
pub struct Genotype {
    pub sequence: Vec<Opcode>,
    pub fitness: f64,
}

impl Genotype {
    pub fn new() -> Self {
        Self {
            sequence: Vec::with_capacity(SEQUENCE_LENGTH),
            fitness: 0.0,
        }
    }

    pub fn random(rng: &mut impl Rng) -> Self {
        let mut genotype = Self::new();
        for _ in 0..SEQUENCE_LENGTH {
            genotype.sequence.push(Opcode::random(rng));
        }
        genotype
    }
}



pub struct EvolutionaryEngine {
    pub population: Vec<Genotype>,
    pub mode: String,
}

impl EvolutionaryEngine {
    pub fn new(mode: &str) -> Self {
        let mut rng = rand::rng();
        let mut population = Vec::with_capacity(POPULATION_SIZE);
        for _ in 0..POPULATION_SIZE {
            population.push(Genotype::random(&mut rng));
        }

        Self {
            population,
            mode: mode.to_string(),
        }
    }

    pub fn evaluate_fitness(sequence: &[Opcode], mode: &str) -> f64 {
        let mut vm = AstVM::new();
        
        if mode == "logic" {
            let table = [
                (0, 0, 0),
                (0, 1, 1),
                (1, 0, 1),
                (1, 1, 0),
            ];
            
            let mut score = 0.0;
            for (a, b, expected) in table.iter() {
                vm.load(sequence);
                vm.registers[0] = *a;
                vm.registers[1] = *b;
                vm.run();
                
                let result = vm.registers[0];
                if result == *expected {
                    score += 100.0;
                } else {
                    score -= (result as f64 - *expected as f64).abs() * 10.0;
                }
            }
            return score;
        } else if mode == "math" {
            let tests = [2, 5, 10];
            let mut score = 0.0;
            for x in tests.iter() {
                vm.load(sequence);
                vm.registers[0] = *x;
                vm.run();
                let result = vm.registers[0];
                let expected = x * 2;
                if result == expected {
                    score += 100.0;
                } else {
                    score -= (result as f64 - expected as f64).abs() * 5.0;
                }
            }
            return score;
        } else if mode == "fault_tolerance" {
            let tests = [2, 5, 10];
            let mut score = 0.0;
            for x in tests.iter() {
                for _trial in 0..5 {
                    vm.load(sequence);
                    vm.registers[0] = *x;
                    vm.run_with_radiation(0.02);
                    
                    let result = vm.registers[0];
                    let expected = x * 2;
                    if result == expected {
                        score += 20.0;
                    } else {
                        score -= (result as f64 - expected as f64).abs() * 1.0;
                    }
                }
            }
            return score;
        } else if mode == "fibonacci" {
            let target_seq = [1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
            vm.load(sequence);
            vm.run(); // It might run until MAX_CYCLES or Hlt

            let mut score = 0.0;
            let outputs = &vm.output_buffer;
            
            for i in 0..target_seq.len() {
                if i < outputs.len() {
                    if outputs[i] == target_seq[i] {
                        score += 100.0;
                    } else {
                        score -= (outputs[i] as f64 - target_seq[i] as f64).abs() * 2.0;
                    }
                } else {
                    score -= 50.0; // Penalty for missing output
                }
            }
            return score;
        }
        
        0.0
    }

    pub fn evolve(&mut self, generations: usize) {
        let mode_ref = &self.mode;
        
        for g in 0..generations {
            self.population.par_iter_mut().for_each(|genotype| {
                genotype.fitness = Self::evaluate_fitness(&genotype.sequence, mode_ref);
            });

            self.population.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap_or(std::cmp::Ordering::Equal));

            let best_fitness = self.population[0].fitness;
            if g % 100 == 0 {
                println!("Generation {} | Optimal Fitness: {}", g, best_fitness);
                let target = if mode_ref == "logic" { 400.0 } else if mode_ref == "fibonacci" { 1000.0 } else { 300.0 };
                if best_fitness >= target {
                    println!("Convergence reached at Generation {}.", g);
                    break;
                }
            }

            let mut next_gen = Vec::with_capacity(POPULATION_SIZE);
            let elite_count = POPULATION_SIZE / 10;
            
            for i in 0..elite_count {
                next_gen.push(self.population[i].clone());
            }

            let mut rng = rand::rng();
            
            while next_gen.len() < POPULATION_SIZE {
                let mut best_parent = 0;
                for _ in 0..3 {
                    let idx = rng.random_range(0..POPULATION_SIZE);
                    if self.population[idx].fitness > self.population[best_parent].fitness {
                        best_parent = idx;
                    }
                }
                
                let mut p1 = self.population[best_parent].clone();

                if rng.random_bool(0.3) {
                    let mut best_parent2 = 0;
                    for _ in 0..3 {
                        let idx = rng.random_range(0..POPULATION_SIZE);
                        if self.population[idx].fitness > self.population[best_parent2].fitness {
                            best_parent2 = idx;
                        }
                    }
                    let p2 = &self.population[best_parent2];
                    
                    for i in 0..SEQUENCE_LENGTH {
                        if rng.random_bool(0.5) {
                            p1.sequence[i] = p2.sequence[i];
                        }
                    }
                }

                for i in 0..SEQUENCE_LENGTH {
                    if rng.random_bool(0.1) {
                        p1.sequence[i] = Opcode::random(&mut rng);
                    }
                }

                next_gen.push(p1);
            }

            self.population = next_gen;
        }
    }
}
