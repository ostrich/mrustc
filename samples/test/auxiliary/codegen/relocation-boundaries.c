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
static int base(void)
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

#include <stddef.h>
#include <string.h>
struct __attribute__((packed)) Multiple {
    uint8_t tag;
    const uint8_t* first;
    uint8_t gap;
    const uint8_t* second;
    uint8_t tail;
};
struct String {
    const char* bytes;
    size_t length;
};
struct __attribute__((packed)) Text {
    uint8_t tag;
    struct String text;
    uint8_t tail;
};
extern const struct Multiple* rust_multiple(void);
extern const struct Text* rust_text(void);
extern const struct Reloc* rust_array(void);
struct __attribute__((packed)) First {
    const uint8_t* pointer;
    uint8_t tail;
};
extern const struct First* rust_first(void);
int main(void)
{
    const struct Multiple* m = rust_multiple();
    const struct Text* t = rust_text();
    const struct Reloc* a = rust_array();
    if (*rust_first()->pointer != 9 || rust_first()->tail != 0x66)
        return 5;
    if (base())
        return 1;
    if (m->tag != 0x11 || *m->first != 7 || m->gap != 0x22 || *m->second != 9 || m->tail != 0x33)
        return 2;
    if (t->tag != 0x44 || t->text.length != 5 || memcmp(t->text.bytes, "hello", 5) || t->tail != 0x55)
        return 3;
    if (a[0].prefix != 0x1111 || *a[0].pointer != 7 || a[0].tail != 0x22 || a[1].prefix != 0x3333 || *a[1].pointer != 9 || a[1].tail != 0x44)
        return 4;
    puts("PASS multiple/string/array relocations");
    return 0;
}
