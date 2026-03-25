#ifndef GENESIS_VM_H
#define GENESIS_VM_H

#include <vector>
#include <cstdint>
#include <string>

// Virtual Instruction Set (VIS) Opcodes
enum OpCode : uint8_t {
    NOP = 0x00,
    INC = 0x01, // INC R<x>
    DEC = 0x02, // DEC R<x>
    ADD = 0x03, // ADD R<dest>, R<src>
    SUB = 0x04, // SUB R<dest>, R<src>
    MOV = 0x05, // MOV R<dest>, R<src>
    LDI = 0x06, // LDI R<dest>, <imm8>
    JMP = 0x07, // JMP <imm8>
    JZ  = 0x08, // JZ  <imm8> (Jump if R0 == 0)
    IO  = 0x09, // IO <port> (0=PrintChar R0, 1=PrintInt R0)
    LD  = 0x0A, // LD R<dest>, R<addr_reg> (Load from [R<addr>])
    ST  = 0x0B, // ST R<addr_reg>, R<src> (Store to [R<addr>])
    HLT = 0xFF
};

// The Cell: Small, atomic execution environment
struct GenesisVM {
    uint8_t* memory; // Pointer to memory (can be shared)
    size_t mem_size;
    bool owns_memory; // Did we allocate it?

    uint8_t registers[4]; // R0, R1, R2, R3
    uint8_t ip;           // Instruction Pointer
    bool halted;
    int instructions_executed;
    const int MAX_CYCLES = 1000; // Prevent infinite loops
    std::vector<uint8_t> output_buffer; // New: Capture IO for fitness

    GenesisVM(uint8_t* shared_mem = nullptr, size_t size = 256);
    ~GenesisVM();
    
    void reset();
    void load_program(const std::vector<uint8_t>& program);
    void step();
    void run();
    
    // Helpers
    uint8_t fetch();
    void execute(uint8_t opcode);
    std::string get_output_string();
};

#endif
