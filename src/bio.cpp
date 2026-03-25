#include "bio.h"
#include <stdexcept>
#include <iostream>

std::string BioCompiler::encode(const std::vector<uint8_t>& data) {
    std::string dna = "ATG"; // Start Codon (Methionine)
    
    for (uint8_t byte : data) {
        // Process 4 pairs of bits: 76 54 32 10
        for (int i = 3; i >= 0; --i) {
            uint8_t pair = (byte >> (i * 2)) & 0x03;
            switch (pair) {
                case 0b00: dna += 'A'; break;
                case 0b01: dna += 'C'; break;
                case 0b10: dna += 'G'; break;
                case 0b11: dna += 'T'; break;
            }
        }
    }
    
    dna += "TAA"; // Stop Codon (Ochre)
    return dna;
}

std::vector<uint8_t> BioCompiler::decode(const std::string& dna) {
    std::vector<uint8_t> data;
    
    // Simple parser: skip invalid chars, look for pairs
    // Real bio-compiler would look for ATG/TAA, but here we assume clean input or just parse body
    
    size_t start = dna.find("ATG");
    if (start == std::string::npos) start = 0; else start += 3;
    
    size_t end = dna.rfind("TAA");
    if (end == std::string::npos) end = dna.length();
    
    uint8_t current_byte = 0;
    int bits_filled = 0;
    
    for (size_t i = start; i < end; ++i) {
        char base = dna[i];
        uint8_t val = 0;
        
        switch (base) {
            case 'A': val = 0b00; break;
            case 'C': val = 0b01; break;
            case 'G': val = 0b10; break;
            case 'T': val = 0b11; break;
            default: continue; // Noise
        }
        
        current_byte = (current_byte << 2) | val;
        bits_filled += 2;
        
        if (bits_filled == 8) {
            data.push_back(current_byte);
            current_byte = 0;
            bits_filled = 0;
        }
    }
    
    return data;
}
