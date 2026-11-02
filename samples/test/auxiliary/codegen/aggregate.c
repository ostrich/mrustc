#include <stddef.h>
#include <stdio.h>
struct ByteC {
    unsigned char value;
};
union ByteUnion {
    unsigned char value;
};
struct __attribute__((packed)) Packed {
    unsigned char byte;
    unsigned word;
};
struct __attribute__((aligned(4))) Aligned {
    unsigned char byte;
};
struct __attribute__((packed, aligned(2))) PackedTwo {
    unsigned char byte;
    unsigned char pad;
    unsigned word;
};
extern size_t rust_metric(unsigned);
extern size_t rust_extra(unsigned);
int main(void)
{
    const size_t expected[] = {
        sizeof(struct ByteC), __alignof__(struct ByteC),
        sizeof(union ByteUnion), __alignof__(union ByteUnion),
        sizeof(struct Packed), __alignof__(struct Packed),
        1, 1, 3, 1, 0, 1, 0, 1
    };
    for (unsigned i = 0; i < sizeof(expected) / sizeof(*expected); i++) {
        if (rust_metric(i) != expected[i]) {
            printf("layout metric %u: %lu != %lu\n", i,
                (unsigned long)rust_metric(i), (unsigned long)expected[i]);
            return 1;
        }
    }
    const size_t extra[] = { sizeof(struct Aligned), __alignof__(struct Aligned),
        sizeof(struct PackedTwo), __alignof__(struct PackedTwo), 1, 1 };
    for (unsigned i = 0; i < sizeof(extra) / sizeof(*extra); i++) {
        if (rust_extra(i) != extra[i]) {
            printf("extra layout metric %u: %lu != %lu\n", i,
                (unsigned long)rust_extra(i), (unsigned long)extra[i]);
            return 1;
        }
    }
    return 0;
}
