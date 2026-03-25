#ifndef BIO_H
#define BIO_H

#include <vector>
#include <string>
#include <cstdint>

class BioCompiler {
public:
    static std::string encode(const std::vector<uint8_t>& data);
    static std::vector<uint8_t> decode(const std::string& dna);
};

#endif
