#include <stdint.h>
#include <stdio.h>
extern uint16_t rust_half(void);
extern uint64_t rust_quad_hi(void), rust_quad_lo(void);
int main(void)
{
    unsigned h = rust_half();
    uint64_t hi = rust_quad_hi(), lo = rust_quad_lo();
    printf("half=%x quad=%llx:%llx\n", h, (unsigned long long)hi, (unsigned long long)lo);
    return h != 0x3e00 || hi != UINT64_C(0x3fff800000000000) || lo != 0;
}
