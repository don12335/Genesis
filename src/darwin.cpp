#include "darwin.h"
#include <algorithm>
#include <iostream>
#include <cmath>

DarwinEngine::DarwinEngine(size_t pop_size, int dna_size) 
    : population_size(pop_size), dna_length(dna_size) {
    
    std::random_device rd;
    rng.seed(rd());
    
    std::uniform_int_distribution<int> byte_dist(0, 255);
    for (size_t i = 0; i < pop_size; ++i) {
        Organism org;
        org.dna.resize(dna_size);
        for (int j = 0; j < dna_size; ++j) {
            org.dna[j] = byte_dist(rng);
        }
        org.fitness = 0.0;
        population.push_back(org);
    }
}

void DarwinEngine::set_target(const std::string& t) {
    target = t;
}

void DarwinEngine::set_mode(const std::string& m) {
    mode = m;
}

double DarwinEngine::score_dna(const std::vector<uint8_t>& dna) {
    GenesisVM vm;
    
    if (mode == "string") {
        vm.load_program(dna);
        vm.run();
        std::string output = vm.get_output_string();
        
        double score = 0.0;
        size_t len = std::min(output.length(), target.length());
        for (size_t i = 0; i < len; ++i) {
            int diff = std::abs((int)output[i] - (int)target[i]);
            if (diff == 0) score += 100.0;
            else score -= diff;
        }
        score -= std::abs((int)output.length() - (int)target.length()) * 50.0;
        return score;
    
    } else if (mode == "math") {
        struct TestCase { uint8_t in; uint8_t out; };
        std::vector<TestCase> tests = {{2, 4}, {5, 10}, {10, 20}};
        double score = 0.0;
        for (const auto& t : tests) {
            vm.reset();
            vm.load_program(dna);
            vm.registers[0] = t.in;
            vm.run();
            int result = vm.registers[0];
            int diff = std::abs(result - t.out);
            if (diff == 0) score += 100.0;
            else score -= diff * 2;
        }
        return score;

    } else if (mode == "survival") {
        vm.reset();
        vm.load_program(dna);
        for(int i=0; i<50; ++i) vm.step();
        
        if (dna.size() > 0) {
            std::uniform_int_distribution<int> pos_dist(0, dna.size()-1);
            std::uniform_int_distribution<int> val_dist(0, 255);
            vm.memory[pos_dist(rng)] = val_dist(rng); 
            vm.memory[pos_dist(rng)] = val_dist(rng); 
        }
        
        vm.run(); 
        std::string output = vm.get_output_string();
        double score = 0.0;
        if (output.find(target) != std::string::npos) {
            score += 200.0; 
            if (output.length() == target.length()) score += 50.0;
        }
        return score;

    } else if (mode == "consciousness") {
        // XOR Gate: Non-linear problem requiring JZ (Branching)
        // 0,0 -> 0
        // 0,1 -> 1
        // 1,0 -> 1
        // 1,1 -> 0
        struct XORCase { uint8_t a; uint8_t b; uint8_t out; };
        std::vector<XORCase> table = {{0,0,0}, {0,1,1}, {1,0,1}, {1,1,0}};
        
        double score = 0.0;
        
        for (const auto& t : table) {
            vm.reset();
            vm.load_program(dna);
            vm.registers[0] = t.a;
            vm.registers[1] = t.b;
            vm.run();
            
            int result = vm.registers[0]; // Output strictly in R0
            if (result == t.out) score += 100.0;
            else score -= std::abs(result - t.out) * 10; // Punish deviation
        }
        return score;
    }
    
    return 0.0;
}

void DarwinEngine::calculate_fitness() {
    for (auto& org : population) {
        org.fitness = score_dna(org.dna);
    }
    std::sort(population.begin(), population.end(), [](const Organism& a, const Organism& b) {
        return a.fitness > b.fitness;
    });
}

void DarwinEngine::selection() {
    size_t elite_count = population_size / 5;
    std::vector<Organism> next_gen;
    
    for (size_t i = 0; i < elite_count; ++i) {
        next_gen.push_back(population[i]);
    }
    
    std::uniform_int_distribution<size_t> index_dist(0, population_size - 1);
    while (next_gen.size() < population_size) {
        size_t i1 = index_dist(rng);
        size_t i2 = index_dist(rng);
        size_t i3 = index_dist(rng);
        
        const Organism* winner = &population[i1];
        if (population[i2].fitness > winner->fitness) winner = &population[i2];
        if (population[i3].fitness > winner->fitness) winner = &population[i3];
        
        next_gen.push_back(*winner); 
    }
    
    population = next_gen;
}

void DarwinEngine::mutation() {
    std::uniform_real_distribution<double> chance(0.0, 1.0);
    std::uniform_int_distribution<int> byte_dist(0, 255);
    std::uniform_int_distribution<int> pos_dist(0, dna_length - 1);
    
    size_t elite_count = population_size / 5;
    
    for (size_t i = elite_count; i < population_size; ++i) {
        if (chance(rng) < 0.1) { 
             int pos = pos_dist(rng);
             population[i].dna[pos] = byte_dist(rng); 
        }
    }
}

void DarwinEngine::evolve(int generations) {
    for (int g = 0; g < generations; ++g) {
        calculate_fitness();
        selection();
        mutation();
        
        if (g % 100 == 0) {
            std::cout << "Gen " << g << " | Best Fitness: " << population[0].fitness << std::endl;
            if (population[0].fitness >= 400.0 && mode == "consciousness") return; // Perfect XOR (4*100)
            if (population[0].fitness >= 300.0 && mode == "math") return; 
        }
    }
}

Organism DarwinEngine::get_best() const {
    return population[0];
}
