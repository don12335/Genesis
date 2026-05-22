use crate::vm::{AstVM, Opcode, NUM_REGISTERS};
use rand::prelude::*;
use rayon::prelude::*;

pub const SEQUENCE_LENGTH: usize = 64;
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
            genotype.sequence.push(random_opcode(rng));
        }
        genotype
    }
}

pub fn random_opcode(rng: &mut impl Rng) -> Opcode {
    let op_type = rng.random_range(0..15);
    match op_type {
        0 => Opcode::Nop,
        1 => Opcode::Inc(rng.random_range(0..NUM_REGISTERS) as u8),
        2 => Opcode::Dec(rng.random_range(0..NUM_REGISTERS) as u8),
        3 => Opcode::Add(rng.random_range(0..NUM_REGISTERS) as u8, rng.random_range(0..NUM_REGISTERS) as u8),
        4 => Opcode::Sub(rng.random_range(0..NUM_REGISTERS) as u8, rng.random_range(0..NUM_REGISTERS) as u8),
        5 => Opcode::Mul(rng.random_range(0..NUM_REGISTERS) as u8, rng.random_range(0..NUM_REGISTERS) as u8),
        6 => Opcode::Div(rng.random_range(0..NUM_REGISTERS) as u8, rng.random_range(0..NUM_REGISTERS) as u8),
        7 => Opcode::Mov(rng.random_range(0..NUM_REGISTERS) as u8, rng.random_range(0..NUM_REGISTERS) as u8),
        8 => Opcode::Ldi(rng.random_range(0..NUM_REGISTERS) as u8, rng.random_range(-10..20)),
        9 => Opcode::Jmp(rng.random_range(-5..5)),
        10 => Opcode::Jz(rng.random_range(0..NUM_REGISTERS) as u8, rng.random_range(-5..5)),
        11 => Opcode::IoOut(rng.random_range(0..NUM_REGISTERS) as u8),
        12 => Opcode::Ld(rng.random_range(0..NUM_REGISTERS) as u8, rng.random_range(0..NUM_REGISTERS) as u8),
        13 => Opcode::St(rng.random_range(0..NUM_REGISTERS) as u8, rng.random_range(0..NUM_REGISTERS) as u8),
        _ => Opcode::Hlt,
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
            // XOR Gate Evaluation
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
            // Doubling: f(x) = x * 2
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
            // Aerospace Fault-Tolerance: f(x) = x * 2 but under heavy bit-flip radiation
            let tests = [2, 5, 10];
            let mut score = 0.0;
            for x in tests.iter() {
                // Run 5 separate trials per test case
                for _trial in 0..5 {
                    vm.load(sequence);
                    vm.registers[0] = *x;
                    vm.run_with_radiation(0.02); // 2% chance per cycle to bit-flip a register
                    
                    let result = vm.registers[0];
                    let expected = x * 2;
                    if result == expected {
                        score += 20.0; // 100 max per test case
                    } else {
                        score -= (result as f64 - expected as f64).abs() * 1.0;
                    }
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
                let target = if mode_ref == "logic" { 400.0 } else { 300.0 };
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
                    
                    let cross_point = rng.random_range(0..SEQUENCE_LENGTH);
                    for i in cross_point..SEQUENCE_LENGTH {
                        p1.sequence[i] = p2.sequence[i];
                    }
                }

                for i in 0..SEQUENCE_LENGTH {
                    if rng.random_bool(0.1) {
                        p1.sequence[i] = random_opcode(&mut rng);
                    }
                }

                next_gen.push(p1);
            }

            self.population = next_gen;
        }
    }
}
