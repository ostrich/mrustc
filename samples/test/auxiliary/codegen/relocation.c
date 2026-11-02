#include <stdint.h>
#include <stdio.h>
struct Reloc {
    uint16_t prefix;
    const uint8_t* pointer;
    uint8_t tail;
};
struct __attribute__((packed)) Packed {
    uint8_t prefix;
    const uint8_t* pointer;
    uint8_t tail;
};
extern const struct Reloc* rust_reloc(void);
extern const struct Packed* rust_packed(void);
int main(void)
{
    const struct Reloc* r = rust_reloc();
    const struct Packed* p = rust_packed();
    if (r->prefix != 0x1234 || r->tail != 0x56 || *r->pointer != 7) {
        puts("FAIL halfword relocation/trailing bytes");
        return 1;
    }
    if (p->prefix != 0x12 || p->tail != 0x34 || *p->pointer != 7) {
        puts("FAIL byte relocation/trailing bytes");
        return 1;
    }
    return 0;
}
