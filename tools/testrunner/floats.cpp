// Representation conversions, not f128 arithmetic. No external oracle needed.
#include "floats.hpp"
#include <cmath>
#include <cstdio>
#include <cstring>
#include <limits>

template <class To, class From>
static To bits(From value)
{
    static_assert(sizeof(To) == sizeof(From), "bit width");
    To out;
    std::memcpy(&out, &value, sizeof(out));
    return out;
}
static unsigned failures;
static void check(bool ok, const char* what)
{
    if (!ok && failures++ < 8)
        std::printf("FAIL %s\n", what);
}
int main()
{
    static_assert(sizeof(F16) == 2 && alignof(F16) == alignof(uint16_t), "F16 ABI");
    static_assert(sizeof(F128) == 16 && alignof(F128) == alignof(uint64_t), "F128 ABI");
    // Exhaust all encodings using exact powers of two as an independent oracle.
    for (unsigned h = 0; h < 65536; ++h) {
        const unsigned exp = (h >> 10) & 31, frac = h & 1023;
        F16 value;
        value.v = h;
        uint32_t expected;
        if (exp == 31) {
            // Exact widening preserves the NaN payload and signaling bit.
            expected = ((h & 0x8000u) << 16) | 0x7f800000u | (frac << 13);
        } else {
            float f = exp ? std::ldexp(float(1024 + frac), int(exp) - 25)
                          : std::ldexp(float(frac), -24);
            if (h & 0x8000)
                f = -f;
            expected = bits<uint32_t>(f);
        }
        check(bits<uint32_t>(float(value)) == expected, "half widening");
        if (exp != 31 || frac == 0)
            check(F16(float(value)).v == h, "half round trip");
    }
    struct Half {
        uint64_t input;
        uint16_t output;
    };
    const Half halves[] = {
        { 0x0000000000000000ULL, 0 }, { 0x8000000000000000ULL, 0x8000 },
        { 0x3ff0000000000000ULL, 0x3c00 }, { 0x3ff0020000000000ULL, 0x3c00 },
        { 0x3ff0020000001000ULL, 0x3c01 }, { 0xbff0020000001000ULL, 0xbc01 },
        { 0x3e60000000000000ULL, 0 }, { 0x3e60000000000001ULL, 1 },
        { 0x3e70000000000000ULL, 1 }, { 0x40effc0000000000ULL, 0x7bff },
        { 0x40effe0000000000ULL, 0x7c00 }, { 0x7ff0000000000000ULL, 0x7c00 },
        { 0xfff0000000000000ULL, 0xfc00 }, { 0x7ff0000000000001ULL, 0x7e00 }
    };
    for (const auto& v : halves)
        check(F16::from_double(bits<double>(v.input)).v == v.output, "double to half boundary");
    struct Wide {
        uint64_t input, hi, lo;
    };
    const Wide wide[] = {
        { 0, 0, 0 }, { 0x8000000000000000ULL, 0x8000000000000000ULL, 0 },
        { 1, 0x3bcd000000000000ULL, 0 },
        { 0x000fffffffffffffULL, 0x3c00ffffffffffffULL, 0xe000000000000000ULL },
        { 0x0010000000000000ULL, 0x3c01000000000000ULL, 0 },
        { 0x3ff8000000000000ULL, 0x3fff800000000000ULL, 0 },
        { 0x7fefffffffffffffULL, 0x43feffffffffffffULL, 0xf000000000000000ULL },
        { 0x7ff0000000000000ULL, 0x7fff000000000000ULL, 0 },
        { 0x7ff8123456789abcULL, 0x7fff8123456789abULL, 0xc000000000000000ULL }
    };
    for (const auto& v : wide) {
        F128 q(bits<double>(v.input));
        check(q.hi == v.hi && q.lo == v.lo, "double widening");
        check(bits<uint64_t>(double(q)) == v.input, "double round trip");
    }
    struct Narrow {
        uint64_t hi, lo, output;
    };
    const Narrow narrow[] = {
        { 0x3fff000000000000ULL, 0x0800000000000000ULL, 0x3ff0000000000000ULL },
        { 0x3fff000000000000ULL, 0x0800000000000001ULL, 0x3ff0000000000001ULL },
        { 0x3fff000000000000ULL, 0x1800000000000000ULL, 0x3ff0000000000002ULL },
        { 0x3bcc000000000000ULL, 0, 0 }, { 0x3bcc000000000000ULL, 1, 1 },
        { 0x3c00ffffffffffffULL, 0xffffffffffffffffULL, 0x0010000000000000ULL },
        { 0x43feffffffffffffULL, 0xf7ffffffffffffffULL, 0x7fefffffffffffffULL },
        { 0x43feffffffffffffULL, 0xf800000000000000ULL, 0x7ff0000000000000ULL }
    };
    for (const auto& v : narrow) {
        F128 q;
        q.hi = v.hi;
        q.lo = v.lo;
        check(bits<uint64_t>(double(q)) == v.output, "quad narrowing boundary");
    }
    return failures ? 1 : 0;
}
