#include <stdio.h>
extern unsigned rust_value(unsigned);
int main(void)
{
    for (unsigned i = 0; i < 6; i++) {
        unsigned expected = i == 5 ? 0 : i + 1;
        if (rust_value(i) != expected) {
            printf("switch %u: %u != %u\n", i, rust_value(i), expected);
            return 1;
        }
    }
    return 0;
}
