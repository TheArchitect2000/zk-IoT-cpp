// To compile this code, you need a C++ compiler that supports C++11 or later.
// This code generates a lookup table for the 8-bit addition operation
// To compile the code for RISCV while you are using x86, you can use the following command:
// g++ -std=c++11 -o add_opcode_table main.cpp -march=rv32i -mabi=ilp32
// This code generates a lookup table for the 8-bit addition operation


#include <iostream>
#include <vector>
#include <cstdint>
#include <fstream>

// Parameters for Plonky2 field (Goldilocks field)
constexpr uint64_t MODULUS = 0xffffffff00000001ULL;

// Number of possible input values for 8-bit add opcode
constexpr size_t TABLE_SIZE = 256;

// Structure to hold a row of the lookup table
struct AddTableRow {
    uint8_t a;
    uint8_t b;
    uint8_t sum;
    uint8_t carry;
};

// Function to compute sum and carry for 8-bit addition
AddTableRow compute_row(uint8_t a, uint8_t b) {
    uint16_t result = static_cast<uint16_t>(a) + static_cast<uint16_t>(b);
    AddTableRow row;
    row.a = a;
    row.b = b;
    row.sum = static_cast<uint8_t>(result & 0xFF);
    row.carry = static_cast<uint8_t>((result >> 8) & 0x1);
    return row;
}

int main() {
    std::vector<AddTableRow> table;
    table.reserve(TABLE_SIZE * TABLE_SIZE);

    // Precompute the table for all possible 8-bit inputs
    for (uint16_t a = 0; a < TABLE_SIZE; ++a) {
        for (uint16_t b = 0; b < TABLE_SIZE; ++b) {
            table.push_back(compute_row(static_cast<uint8_t>(a), static_cast<uint8_t>(b)));
        }
    }

    // Optionally, write the table to a CSV file
    std::ofstream fout("add_opcode_table.csv");
    fout << "a,b,sum,carry\n";
    for (const auto& row : table) {
        fout << static_cast<int>(row.a) << ","
             << static_cast<int>(row.b) << ","
             << static_cast<int>(row.sum) << ","
             << static_cast<int>(row.carry) << "\n";
    }
    fout.close();

    std::cout << "Add opcode table generated and saved to add_opcode_table.csv\n";
    return 0;
}