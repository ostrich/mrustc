#include <stddef.h>
#include <stdio.h>
struct Fields {
    unsigned char byte;
    const unsigned char* pointer;
};
extern size_t rust_metric(unsigned);
int main(void)
{
    const size_t expected[] = { sizeof(void*), _Alignof(void*), _Alignof(void*), _Alignof(void (*)(void)), sizeof(struct Fields), _Alignof(struct Fields), 2 * sizeof(void*), _Alignof(void*) };
    unsigned i, failed = 0;
    for (i = 0; i < sizeof(expected) / sizeof(*expected); i++) {
        size_t actual = rust_metric(i);
        if (actual != expected[i]) {
            printf("FAIL metric %u: expected %lu got %lu\n", i, (unsigned long)expected[i], (unsigned long)actual);
            failed++;
        }
    }
    return failed != 0;
}
