#ifndef DARWIN_H
#define DARWIN_H

#include <vector>
#include <string>
#include <random>
#include "vm.h"

struct Organism {
    std::vector<uint8_t> dna;
    double fitness;
};

class DarwinEngine {
public:
    DarwinEngine(size_t pop_size, int dna_size);
    void set_target(const std::string& target_str);
    void set_mode(const std::string& m); // "string" or "math"
    void evolve(int generations);
    Organism get_best() const;

private:
    std::vector<Organism> population;
    std::string target;
    std::string mode = "string"; // Default
    std::mt19937 rng;
    size_t population_size;
    int dna_length;

    void calculate_fitness();
    void selection();
    void mutation();
    void crossover();
    
    double score_dna(const std::vector<uint8_t>& dna);
};

#endif
