#include <stdint.h>
#include <stdio.h>
extern const unsigned char *rust_promoted(unsigned), *rust_static_base(void), *rust_static_field(void);
int main(void)
{
    unsigned i, failed = 0;
    for (i = 0; i < 3; i++) {
        const unsigned char* p = rust_promoted(i);
        printf("field %u offset modulo 64: %u\n", i, (unsigned)((uintptr_t)p % 64));
        if ((uintptr_t)p % 64 != 2) {
            puts("FAIL containing alignment");
            failed++;
        }
        if (p[0] != (i == 1 ? 8 : 7) || p[1] != 8 || p[2] != 9) {
            puts("FAIL field bytes");
            failed++;
        }
    }
    if (rust_static_field() != rust_static_base() + 2) {
        puts("FAIL static identity");
        failed++;
    }
    return failed != 0;
}
