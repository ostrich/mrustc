#include <stdio.h>
extern unsigned rust_value(unsigned);
int main(void)
{
    const unsigned values[] = { 0, 65, 0x7f, 0x80, 0xffff, 0x10000, 0x10ffff };
    for (unsigned i = 0; i < sizeof(values) / sizeof(*values); i++) {
        if (rust_value(i) != values[i]) {
            printf("character %u\n", i);
            return 1;
        }
    }
    return 0;
}
