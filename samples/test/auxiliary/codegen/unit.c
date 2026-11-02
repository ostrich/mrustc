/* Compiled with the emitted C included, so the internal placeholder types
 * are checked directly, independently of Rust's zero-sized type metrics.
 * GNU backend policy currently represents an empty type with one byte. */
_Static_assert(sizeof(tUNIT) == 1, "unit placeholder size");
_Static_assert(__alignof__(tUNIT) == 1, "unit placeholder alignment");
_Static_assert(sizeof(tTYPEID) == 1, "type-id placeholder size");
_Static_assert(__alignof__(tTYPEID) == 1, "type-id placeholder alignment");
int main(void) { return 0; }
