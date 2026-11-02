#include <stdint.h>
#include <stdio.h>
extern uint16_t rust_half_pos(void), rust_half_neg(void);
int main(void)
{
    unsigned p = rust_half_pos(), n = rust_half_neg();
    printf("half direct %04x %04x\n", p, n);
    return p != 0x3c01 || n != 0xbc01;
}
