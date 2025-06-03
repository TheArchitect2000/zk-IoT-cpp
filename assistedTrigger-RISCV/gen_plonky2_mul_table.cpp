#include <cstdint>
#include <cstdlib>
#include <cstdio>
#include <cstring>
#include <inttypes.h>

// The Goldilocks prime used by Plonky2: 2^64 - 2^32 + 1
static constexpr uint64_t GOLDILOCKS_P =
    (uint64_t)(((__uint128_t)1 << 64) - ((uint64_t)1 << 32) + 1);


// Compute (a * b) mod GOLDILOCKS_P safely using 128-bit intermediate
static uint64_t mul_mod(uint64_t a, uint64_t b) {
    __uint128_t prod = ( __uint128_t )a * b;
    prod = (prod & 0xFFFFFFFFFFFFFFFFULL) + (prod >> 64) * 0x100000000ULL;
    // One more reduction if needed
    if (prod >= GOLDILOCKS_P) prod -= GOLDILOCKS_P;
    return (uint64_t)prod;
}

// Generates an N×N table of (i*j mod p), writes as binary little-endian.
// Returns 0 on success.
int generate_lookup(const char* out_filename, size_t N) {
    // Allocate N*N entries of uint64_t
    size_t total = N * N;
    uint64_t *table = (uint64_t*) std::malloc(total * sizeof(uint64_t));
    if (!table) {
        std::perror("malloc");
        return 1;
    }

    // Fill the table
    for (size_t i = 0; i < N; i++) {
        // pointer to row i
        uint64_t *row = table + (i * N);
        for (size_t j = 0; j < N; j++) {
            row[j] = mul_mod((uint64_t)i, (uint64_t)j);
        }
    }

    // Write to disk
    FILE *f = std::fopen(out_filename, "wb");
    if (!f) {
        std::perror("fopen");
        std::free(table);
        return 1;
    }
    size_t written = std::fwrite(table, sizeof(uint64_t), total, f);
    if (written != total) {
        std::perror("fwrite");
        std::fclose(f);
        std::free(table);
        return 1;
    }
    std::fclose(f);
    std::free(table);
    return 0;
}

int main(int argc, char** argv) {
    if (argc != 3) {
        std::fprintf(stderr, "Usage: %s <N> <output_file>\n", argv[0]);
        return 1;
    }
    size_t N = std::strtoull(argv[1], nullptr, 10);
    const char* out_file = argv[2];

    if (N == 0) {
        std::fprintf(stderr, "Error: N must be > 0\n");
        return 1;
    }

    std::printf("Generating %zu×%zu Goldilocks multiplication table…\n", N, N);
    if (generate_lookup(out_file, N) != 0) {
        std::fprintf(stderr, "Lookup generation failed\n");
        return 1;
    }
    std::printf("Lookup table written to %s (%" PRIu64 " entries)\n",
                out_file, (uint64_t)(N * N));
    return 0;
}
