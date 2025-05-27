#include <iostream>
#include <cstdint>
#include <cstdint>
#include <cassert>

uint32_t bitwise_and(uint32_t a, uint32_t b) {
    uint32_t result = 0;
    uint32_t pow2 = 1;

    for (int i = 0; i < 32; ++i) {
        uint32_t a_shifted = a / pow2;
        uint32_t b_shifted = b / pow2;

        uint32_t a_bit = a_shifted - 2 * (a_shifted / 2);
        uint32_t b_bit = b_shifted - 2 * (b_shifted / 2);

        uint32_t and_bit = a_bit * b_bit;

        result = result + and_bit * pow2;
        pow2 = pow2 * 2;
    }

    return result;
}


void test(int32_t a, int32_t b) {
    uint32_t ua = static_cast<uint32_t>(a);
    uint32_t ub = static_cast<uint32_t>(b);

    uint32_t expected = ua & ub;
    uint32_t actual = bitwise_and(ua, ub);

    std::cout << "a = " << a << ", b = " << b
              << " → expected: " << static_cast<int32_t>(expected)
              << ", actual: "   << static_cast<int32_t>(actual) << '\n';

    assert(expected == actual);
}


int main() {
    // Basic tests
    test(0, 0);
    test(1, 1);
    test(1, 0);
    test(0xFFFFFFFF, 0xFFFFFFFF);
    test(0xAAAAAAAA, 0x55555555);
    test(0x12345678, 0x0F0F0F0F);
    test(0x7FFFFFFF, 0x80000000);

    // Signed values
    test(-1, 0);
    test(-1, -1);
    test(-123456789, 123456789);
    test(-1000, -500);

    std::cout << "All tests passed!\n";
    return 0;
}
