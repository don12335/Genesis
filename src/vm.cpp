#include "vm.h"
#include <iostream>
#include <cstring>

GenesisVM::GenesisVM(uint8_t* shared_mem, size_t size) : mem_size(size) {
    if (shared_mem) {
        memory = shared_mem;
        owns_memory = false;
    } else {
        memory = new uint8_t[size];
        owns_memory = true;
    }
    reset();
}

GenesisVM::~GenesisVM() {
    if (owns_memory) {
        delete[] memory;
    }
}

void GenesisVM::reset() {
    if (owns_memory) {
        std::memset(memory, 0, mem_size);
    }
    // If shared memory, we ideally don't wipe it on reset? 
    // But for Darwin loop we do. For Arena we might not?
    // Let's assume reset() wipes memory for now. 
    // Actually for Arena P2, we shouldn't wipe P1's code.
    // Let's make reset partial.
    std::memset(registers, 0, sizeof(registers));
    output_buffer.clear();
    ip = 0;
    halted = false;
    instructions_executed = 0;
}

std::string GenesisVM::get_output_string() {
    return std::string(output_buffer.begin(), output_buffer.end());
}

void GenesisVM::load_program(const std::vector<uint8_t>& program) {
    if (program.size() > mem_size) {
        std::cerr << "Error: DNA too long for cell memory." << std::endl;
        return;
    }
    std::memcpy(memory, program.data(), program.size());
}

uint8_t GenesisVM::fetch() {
    return memory[ip++ % mem_size]; // Safe wrap
}

void GenesisVM::step() {
    if (halted) return;
    
    if (instructions_executed >= MAX_CYCLES) {
        halted = true; 
        return;
    }

    uint8_t opcode = fetch();
    execute(opcode);
    instructions_executed++;
}

void GenesisVM::run() {
    while (!halted) {
        step();
    }
}

void GenesisVM::execute(uint8_t opcode) {
    // Helper macros for extracting operands safely
    #define ARG_REG(x) (memory[(ip + x) % mem_size] % 4) // Safe Modulo 4 for valid reg
    #define ARG_IMM(x) (memory[(ip + x) % mem_size])     // Safe memory access

    switch (opcode) {
        case NOP:
            break;

        case INC: {
            uint8_t reg = fetch() % 4; 
            registers[reg]++;
            break;
        }

        case DEC: {
            uint8_t reg = fetch() % 4;
            registers[reg]--;
            break;
        }

        case ADD: {
            uint8_t dst = fetch() % 4;
            uint8_t src = fetch() % 4;
            registers[dst] += registers[src];
            break;
        }

        case SUB: {
            uint8_t dst = fetch() % 4;
            uint8_t src = fetch() % 4;
            registers[dst] -= registers[src];
            break;
        }

        case MOV: {
            uint8_t dst = fetch() % 4;
            uint8_t src = fetch() % 4;
            registers[dst] = registers[src];
            break;
        }

        case LDI: {
            uint8_t dst = fetch() % 4;
            uint8_t val = fetch();
            registers[dst] = val;
            break;
        }

        case JMP: {
            uint8_t target = fetch();
            ip = target % mem_size;
            break;
        }

        case JZ: {
            uint8_t target = fetch();
            if (registers[0] == 0) {
                ip = target % mem_size;
            }
            break;
        }

        case IO: {
            uint8_t port = fetch();
            if (port == 0) {
                output_buffer.push_back(registers[0]);
            } else if (port == 1) {
                 std::string s = std::to_string(registers[0]);
                 for (char c : s) output_buffer.push_back(c);
            }
            break;
        }
        
        case LD: {
            // LD R<dst>, R<addr_reg>
            uint8_t dst = fetch() % 4;
            uint8_t addr_reg = fetch() % 4;
            uint8_t addr = registers[addr_reg];
            registers[dst] = memory[addr % mem_size];
            break;
        }
        
        case ST: {
            // ST R<addr_reg>, R<src>
            uint8_t addr_reg = fetch() % 4;
            uint8_t src = fetch() % 4;
            uint8_t addr = registers[addr_reg];
            memory[addr % mem_size] = registers[src];
            break;
        }

        case HLT:
            halted = true;
            break;

        default:
            break;
    }
}
