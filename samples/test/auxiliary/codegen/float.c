#include <math.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>
extern double rust_double(unsigned);
extern float rust_single(void);
extern uint64_t rust_bits(unsigned);
int main(void)
{
    const uint64_t expected[] = { UINT64_C(0x400c000000000000), UINT64_C(0x8000000000000000), UINT64_C(0x7ff8123456789abc), UINT64_C(0xc029000000000000) };
    unsigned i, failed = 0;
    float single = rust_single();
    uint32_t sb;
    memcpy(&sb, &single, 4);
    for (i = 0; i < 4; i++) {
        double d = rust_double(i);
        uint64_t bits;
        memcpy(&bits, &d, 8);
        if (i == 2 ? !isnan(d) : bits != expected[i]) {
            printf("FAIL double %u: %llx != %llx\n", i, (unsigned long long)bits, (unsigned long long)expected[i]);
            failed++;
        }
    }
    for (i = 0; i < 3; i++) {
        uint64_t bits = rust_bits(i);
        if (bits != expected[i]) {
            printf("FAIL const bits %u\n", i);
            failed++;
        }
    }
    if (sb != 0xc0400000U) {
        puts("FAIL single");
        failed++;
    }
    return failed != 0;
}
