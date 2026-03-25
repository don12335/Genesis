#include <iostream>
#include <fstream>
#include "darwin.h"
#include "bio.h"
#include "arena.h"

void print_asm_trace(const std::vector<uint8_t>& bytecode) {
    std::cout << "Bytecode Size: " << bytecode.size() << " bytes" << std::endl;
    std::cout << "Assembly Trace:" << std::endl;
    
    for (size_t i = 0; i < bytecode.size(); ++i) {
        uint8_t op = bytecode[i];
        printf("[%02X] ", op);
        
        switch(op) {
            case NOP: std::cout << "NOP"; break;
            case INC: std::cout << "INC"; break;
            case DEC: std::cout << "DEC"; break;
            case ADD: std::cout << "ADD"; break;
            case SUB: std::cout << "SUB"; break;
            case MOV: std::cout << "MOV"; break;
            case LDI: std::cout << "LDI"; break;
            case JMP: std::cout << "JMP"; break;
            case JZ:  std::cout << "JZ "; break;
            case IO:  std::cout << "IO "; break;
            case LD:  std::cout << "LD "; break;
            case ST:  std::cout << "ST "; break;
            case HLT: std::cout << "HLT"; break;
            default: std::cout << "???"; break;
        }
        std::cout << std::endl;
    }
}

int main(int argc, char* argv[]) {
    // --- MODE 1: DECODE ---
    if (argc > 1 && std::string(argv[1]) == "decode") {
        if (argc < 3) {
            std::cout << "Usage: genesis.exe decode <DNA_SEQUENCE>" << std::endl;
            return 1;
        }
        std::string dna = argv[2];
        std::cout << "🧬 Bio-Decoder: Translating DNA..." << std::endl;
        std::vector<uint8_t> bytecode = BioCompiler::decode(dna);
        print_asm_trace(bytecode);
        return 0;
    }
    
    // --- MODE 1.5: EXPORT ---
    if (argc > 1 && std::string(argv[1]) == "export") {
        if (argc < 3) return 1;
        std::string type = argv[2];
        std::vector<uint8_t> bytecode;
        
        if (type == "bomber") {
            bytecode = { LDI, 0, 0, LDI, 1, 20, ST, 1, 0, INC, 1, JMP, 6 };
        } else if (type == "runner") {
            bytecode = { JMP, 0 }; 
        } else if (type == "replicator") {
            bytecode = { LDI, 0, 0, LDI, 1, 64, LD, 3, 0, ST, 1, 3, INC, 0, INC, 1, JMP, 6 };
        } else {
            return 1;
        }
        std::cout << BioCompiler::encode(bytecode) << std::endl;
        return 0;
    }

    // --- MODE 1.8: TRANSPILE (Bio-Universal Translator) ---
    if (argc > 1 && std::string(argv[1]) == "transpile") {
        if (argc < 3) return 1;
        std::vector<uint8_t> bytecode = BioCompiler::decode(argv[2]);
        
        std::cout << "// Bio-Transpiled C++ Source" << std::endl;
        std::cout << "#include <iostream>" << std::endl;
        std::cout << "#include <vector>" << std::endl;
        std::cout << "int main() {" << std::endl;
        std::cout << "    uint8_t r[4] = {0,0,0,0};" << std::endl;
        std::cout << "    uint8_t m[1024] = {0};" << std::endl;
        std::cout << "    int ip = 0;" << std::endl;
        
        // Naive linear transpilation (Labels for jumps)
        for (size_t i = 0; i < bytecode.size(); ++i) {
             std::cout << "L" << i << ": ";
             uint8_t op = bytecode[i];
             
             // Helpers for args
             auto Fetch = [&]() { return (i + 1 < bytecode.size()) ? bytecode[++i] : 0; };
             
             switch(op) {
                 case NOP: std::cout << ";" << std::endl; break;
                 case INC: { int reg = Fetch() % 4; std::cout << "r[" << reg << "]++;" << std::endl; break; }
                 case DEC: { int reg = Fetch() % 4; std::cout << "r[" << reg << "]--;" << std::endl; break; }
                 case ADD: { int d = Fetch() % 4; int s = Fetch() % 4; std::cout << "r[" << d << "] += r[" << s << "];" << std::endl; break; }
                 case SUB: { int d = Fetch() % 4; int s = Fetch() % 4; std::cout << "r[" << d << "] -= r[" << s << "];" << std::endl; break; }
                 case MOV: { int d = Fetch() % 4; int s = Fetch() % 4; std::cout << "r[" << d << "] = r[" << s << "];" << std::endl; break; }
                 case LDI: { int d = Fetch() % 4; int v = Fetch(); std::cout << "r[" << d << "] = " << (int)v << ";" << std::endl; break; }
                 case JMP: { int t = Fetch(); std::cout << "goto L" << (int)t << ";" << std::endl; break; } // Note: Might jump to invalid label if not watchful
                 case JZ:  { int t = Fetch(); std::cout << "if (r[0]==0) goto L" << (int)t << ";" << std::endl; break; }
                 case IO:  { int p = Fetch(); if (p==0) std::cout << "std::cout << (char)r[0];" << std::endl; else std::cout << "std::cout << (int)r[0];" << std::endl; break; }
                 case HLT: std::cout << "return 0;" << std::endl; break;
                 default: std::cout << "// ??? " << (int)op << std::endl; break;
             }
        }
        
        std::cout << "    return 0;" << std::endl;
        std::cout << "}" << std::endl;
        return 0;
    }
    
    // --- MODE 2: ARENA ---
    if (argc > 1 && std::string(argv[1]) == "arena") {
        std::cout << "⚔️  Preparing Arena..." << std::endl;
        Arena arena;
        std::vector<uint8_t> p1 = { LDI, 0, 0, LDI, 1, 20, ST, 1, 0, INC, 1, JMP, 6 };
        std::vector<uint8_t> p2 = { JMP, 0 };
        if (argc >= 4) {
            p1 = BioCompiler::decode(argv[2]);
            p2 = BioCompiler::decode(argv[3]);
        }
        arena.load_warriors(p1, p2);
        arena.run_battle(5000);
        return 0;
    }

    // --- MODE 3: EVOLVE (Default) ---
    std::cout << "🧬 Project Genesis: Starting Evolution..." << std::endl;
    
    bool math_mode = (argc > 1 && std::string(argv[1]) == "math");
    
    DarwinEngine engine(1000, 32); 
    
    if (math_mode) {
         std::cout << "Target: Logic f(x) = x + x (Doubling)" << std::endl;
         engine.set_mode("math"); 
    } else if (argc > 1 && std::string(argv[1]) == "survival") {
         std::cout << "Target: Immortal Kernel (Survive Memory Corruption)" << std::endl;
         engine.set_mode("survival");
         engine.set_target("Hi"); 
         // Re-initialize population with LARGER DNA (128 bytes) for redundancy
         engine = DarwinEngine(1000, 128); 
         engine.set_mode("survival");
         engine.set_target("Hi");
    } else if (argc > 1 && std::string(argv[1]) == "consciousness") {
         std::cout << "Target: Vant-Genesis Merger (XOR Logic Gate)" << std::endl;
         std::cout << "Goal: Evolve Non-Linear Decision Making." << std::endl;
         engine.set_mode("consciousness");
    } else {
         std::string target = "Hi"; 
         engine.set_target(target);
         std::cout << "Target: String [" << target << "]" << std::endl;
    }
    
    std::cout << "Population: 1000 | DNA Size: 32 bytes" << std::endl;
    engine.evolve(5000); 
    
    Organism best = engine.get_best();
    std::cout << "\n------------------------------------------------" << std::endl;
    std::cout << "Evolution Complete." << std::endl;
    std::cout << "Best DNA (Hex): ";
    for (uint8_t b : best.dna) printf("%02X ", b);
    std::cout << "\nFinal Output: ";
    
    GenesisVM vm;
    
    if (math_mode) {
        std::cout << "\nLogic Verification (Doubling):" << std::endl;
        for (int i = 1; i <= 5; ++i) {
             vm.reset();
             vm.load_program(best.dna);
             vm.registers[0] = i; // Input
             vm.run();
             std::cout << "f(" << i << ") = " << (int)vm.registers[0] << std::endl;
        }
    } else if (argc > 1 && std::string(argv[1]) == "consciousness") {
        std::cout << "\nConsciousness Verification (XOR Truth Table):" << std::endl;
        struct XORCase { uint8_t a; uint8_t b; };
        std::vector<XORCase> table = {{0,0}, {0,1}, {1,0}, {1,1}};
        
        for (const auto& t : table) {
             vm.reset();
             vm.load_program(best.dna);
             vm.registers[0] = t.a; 
             vm.registers[1] = t.b;
             vm.run();
             std::cout << t.a << " XOR " << t.b << " = " << (int)vm.registers[0] 
                       << " | Cycles: " << vm.instructions_executed << std::endl;
        }
    } else {
        vm.load_program(best.dna);
        vm.run();
        std::cout << "[" << vm.get_output_string() << "]" << std::endl;
    }
    
    std::cout << "\n------------------------------------------------" << std::endl;
    std::cout << "Bio-Compilation..." << std::endl;
    std::string bio_dna = BioCompiler::encode(best.dna);
    std::cout << "DNA Sequence: " << bio_dna << std::endl;
    
    std::ofstream out("artifact.dna");
    out << bio_dna;
    out.close();
    std::cout << "Saved to 'artifact.dna'." << std::endl;

    return 0;
}
