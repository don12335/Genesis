#ifndef ARENA_H
#define ARENA_H

#include <vector>
#include <string>
#include <cstdint>
#include "vm.h"

class Arena {
public:
    static const size_t MEM_SIZE = 1024;
    uint8_t memory[MEM_SIZE];
    
    GenesisVM* p1;
    GenesisVM* p2;
    
    Arena();
    ~Arena();
    
    void load_warriors(const std::vector<uint8_t>& dna1, const std::vector<uint8_t>& dna2);
    void run_battle(int cycles);
    void render();
};

#endif
