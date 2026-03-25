#include "arena.h"
#include <iostream>
#include <cstring>
#include <thread>
#include <chrono>

Arena::Arena() {
    std::memset(memory, 0, MEM_SIZE);
    
    // Create VMs sharing the SAME memory
    p1 = new GenesisVM(memory, MEM_SIZE);
    p2 = new GenesisVM(memory, MEM_SIZE);
}

Arena::~Arena() {
    delete p1;
    delete p2;
}

void Arena::load_warriors(const std::vector<uint8_t>& dna1, const std::vector<uint8_t>& dna2) {
    // Clear Battleground
    std::memset(memory, 0, MEM_SIZE);
    
    // Load P1 at 0
    if (dna1.size() > 256) std::cerr << "P1 too fat!" << std::endl;
    std::memcpy(memory, dna1.data(), std::min(dna1.size(), (size_t)256));
    
    // Load P2 at 512 (Halfway)
    if (dna2.size() > 256) std::cerr << "P2 too fat!" << std::endl;
    std::memcpy(memory + 512, dna2.data(), std::min(dna2.size(), (size_t)256));
    
    // Reset Processors
    p1->reset();
    p1->ip = 0;
    
    p2->reset();
    p2->ip = 512;
}

void Arena::run_battle(int cycles) {
    std::cout << "⚔️  THE ARENA ⚔️" << std::endl;
    std::cout << "P1 (Red) vs P2 (Blue)" << std::endl;
    
    for (int i = 0; i < cycles; ++i) {
        // Round Robin Execution
        if (!p1->halted) p1->step();
        if (!p2->halted) p2->step();
        
        if (i % 50 == 0) { // Render every 50 cycles
            render();
            std::this_thread::sleep_for(std::chrono::milliseconds(50));
        }
        
        if (p1->halted && p2->halted) {
             std::cout << "Both warriors died." << std::endl;
             break;
        }
    }
}

void Arena::render() {
    // Clear screen (ANSI)
    std::cout << "\033[2J\033[H";
    
    std::cout << "Cycle: P1 IP=" << (int)p1->ip << " | P2 IP=" << (int)p2->ip << std::endl;
    
    for (int i = 0; i < MEM_SIZE; ++i) {
        // Color logic
        bool is_p1_ip = (i == p1->ip);
        bool is_p2_ip = (i == p2->ip);
        
        if (is_p1_ip && is_p2_ip) std::cout << "\033[1;45mXX\033[0m"; // Collision
        else if (is_p1_ip) std::cout << "\033[1;31m[]\033[0m";       // P1 Cursor
        else if (is_p2_ip) std::cout << "\033[1;34m[]\033[0m";       // P2 Cursor
        else if (memory[i] != 0) std::cout << "\033[1;32m" << (memory[i] > 32 && memory[i] < 126 ? (char)memory[i] : '.') << "\033[0m "; 
        else std::cout << ". ";
        
        if ((i + 1) % 64 == 0) std::cout << std::endl;
    }
}
