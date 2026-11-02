#pragma once
#include <cstdint>
#include <cstring>

namespace float_helpers {
    // Round a significand of at most 113 bits to nearest, ties to even.
    // Callers shift far enough that the rounded result fits in uint64_t.
    inline uint64_t round_significand(uint64_t hi, uint64_t lo, unsigned shift)
    {
        if( shift == 0 ) {
            return lo;
        }
        if( shift >= 128 ) {
            return 0;
        }
        uint64_t result;
        bool guard, sticky;
        if( shift < 64 ) {
            result = (hi << (64 - shift)) | (lo >> shift);
            guard = ((lo >> (shift - 1)) & 1) != 0;
            sticky = (lo & ((uint64_t(1) << (shift - 1)) - 1)) != 0;
        }
        else if( shift == 64 ) {
            result = hi;
            guard = (lo >> 63) != 0;
            sticky = (lo & UINT64_C(0x7fffffffffffffff)) != 0;
        }
        else {
            const unsigned high_shift = shift - 64;
            result = hi >> high_shift;
            guard = ((hi >> (high_shift - 1)) & 1) != 0;
            sticky = lo != 0 || (hi & ((uint64_t(1) << (high_shift - 1)) - 1)) != 0;
        }
        return result + (guard && (sticky || (result & 1)));
    }
}

struct F16
{
    // IEEE binary16: 1 sign, 5 exponent and 10 fraction bits.
    uint16_t v;

    F16(): v(0) {}

    F16(float f)
    {
        uint32_t bits;
        static_assert(sizeof(f) == sizeof(bits), "F16 requires IEEE binary32 float");
        std::memcpy(&bits, &f, sizeof(bits));
        v = encode(bits, 23, 8);
    }

    // Constant evaluation stores its intermediate in double. Narrow directly,
    // avoiding double rounding through an intermediate binary32 value.
    static F16 from_double(double f)
    {
        uint64_t bits;
        static_assert(sizeof(f) == sizeof(bits), "F16 requires IEEE binary64 double");
        std::memcpy(&bits, &f, sizeof(bits));
        F16 result;
        result.v = encode(bits, 52, 11);
        return result;
    }

    operator float() const
    {
        const uint32_t sign = uint32_t(v & 0x8000) << 16;
        const unsigned exponent = (v >> 10) & 0x1f;
        uint32_t fraction = v & 0x3ff;
        uint32_t bits;
        if( exponent == 0x1f ) {
            // Exact widening retains the NaN payload and quiet/signaling bit.
            bits = sign | 0x7f800000 | (fraction << 13);
        }
        else if( exponent != 0 ) {
            bits = sign | ((exponent + 112) << 23) | (fraction << 13);
        }
        else if( fraction != 0 ) {
            int e = -14;
            while( (fraction & 0x400) == 0 ) {
                fraction <<= 1;
                --e;
            }
            bits = sign | (uint32_t(e + 127) << 23) | ((fraction & 0x3ff) << 13);
        }
        else {
            bits = sign;
        }
        float result;
        std::memcpy(&result, &bits, sizeof(bits));
        return result;
    }

private:
    static uint16_t encode(uint64_t bits, unsigned fraction_bits, unsigned exponent_bits)
    {
        const uint16_t sign = uint16_t(bits >> (fraction_bits + exponent_bits)) << 15;
        const unsigned max_exponent = (1u << exponent_bits) - 1;
        const unsigned exponent = unsigned(bits >> fraction_bits) & max_exponent;
        const uint64_t fraction = bits & ((uint64_t(1) << fraction_bits) - 1);
        if( exponent == max_exponent ) {
            // Narrowing quiets NaNs and retains the high payload bits and sign.
            return sign | 0x7c00 | (fraction != 0
                ? 0x200 | uint16_t((fraction >> (fraction_bits - 10)) & 0x1ff) : 0);
        }
        const int target_exponent = int(exponent ? exponent : 1) - int(max_exponent / 2) + 15;
        if( target_exponent >= 31 ) {
            return sign | 0x7c00;
        }
        const uint64_t significand = fraction | (exponent ? uint64_t(1) << fraction_bits : 0);
        unsigned shift = fraction_bits - 10;
        if( target_exponent <= 0 ) {
            shift += 1 - target_exponent;
        }
        const uint64_t rounded = float_helpers::round_significand(0, significand, shift);
        // Including the hidden bit also handles carries into the next exponent,
        // subnormal-to-normal rounding and finite-to-infinity rounding.
        return sign | uint16_t(target_exponent <= 0
            ? rounded : (uint64_t(target_exponent - 1) << 10) + rounded);
    }
};

struct F128
{
    // IEEE binary128, stored as two numeric words, independent of byte order.
    uint64_t lo;
    uint64_t hi;

    F128(): lo(0), hi(0) {}

    F128(double value)
    {
        uint64_t bits;
        static_assert(sizeof(value) == sizeof(bits), "F128 requires IEEE binary64 double");
        std::memcpy(&bits, &value, sizeof(bits));
        const uint64_t sign = bits & UINT64_C(0x8000000000000000);
        const unsigned exponent = unsigned(bits >> 52) & 0x7ff;
        uint64_t fraction = bits & UINT64_C(0x000fffffffffffff);
        unsigned target_exponent;
        if( exponent == 0x7ff ) {
            target_exponent = 0x7fff;
        }
        else if( exponent != 0 ) {
            target_exponent = int(exponent) - 1023 + 16383;
        }
        else if( fraction != 0 ) {
            int e = -1022;
            while( (fraction & (uint64_t(1) << 52)) == 0 ) {
                fraction <<= 1;
                --e;
            }
            fraction &= UINT64_C(0x000fffffffffffff);
            target_exponent = e + 16383;
        }
        else {
            target_exponent = 0;
        }
        hi = sign | (uint64_t(target_exponent) << 48) | (fraction >> 4);
        lo = fraction << 60;
    }

    operator double() const
    {
        const uint64_t sign = hi & UINT64_C(0x8000000000000000);
        const unsigned exponent = unsigned(hi >> 48) & 0x7fff;
        const uint64_t fraction_hi = hi & UINT64_C(0x0000ffffffffffff);
        uint64_t bits;
        if( exponent == 0x7fff ) {
            const bool nan = fraction_hi != 0 || lo != 0;
            const uint64_t payload = ((fraction_hi << 4) | (lo >> 60)) & UINT64_C(0x0007ffffffffffff);
            bits = sign | UINT64_C(0x7ff0000000000000)
                | (nan ? UINT64_C(0x0008000000000000) | payload : 0);
        }
        else {
            const int target_exponent = int(exponent ? exponent : 1) - 16383 + 1023;
            if( target_exponent >= 0x7ff ) {
                bits = sign | UINT64_C(0x7ff0000000000000);
            }
            else {
                const uint64_t significand_hi = fraction_hi | (exponent ? uint64_t(1) << 48 : 0);
                const unsigned shift = target_exponent <= 0 ? 61 - target_exponent : 60;
                const uint64_t rounded = float_helpers::round_significand(significand_hi, lo, shift);
                bits = sign | (target_exponent <= 0
                    ? rounded : (uint64_t(target_exponent - 1) << 52) + rounded);
            }
        }
        double result;
        std::memcpy(&result, &bits, sizeof(bits));
        return result;
    }
};
