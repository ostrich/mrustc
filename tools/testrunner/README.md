# Regression tests

The usual Rust execution tests are in `samples/test/`; `make -f minicargo.mk
local_tests` builds their Rust library dependencies and runs them.

The optional C ABI and constant tests use small Rust libraries and independent
C harnesses in `samples/test/auxiliary/codegen/`. They need Python 3 and a GNU C
compiler, but no target Rust libraries. From the repository root:

```
python3 tools/testrunner/codegen-tests.py --output output/codegen-native
```

The output directory must be new. Each case retains its compiler commands and
output in `log.txt`. An error, timeout or nonzero program exit fails the run.
To run a subset, append case names (without extensions).

Cross execution requires an existing C cross compiler, target sysroot and runner:

```
python3 tools/testrunner/codegen-tests.py --target m68k-linux-gnu \
    --cc m68k-linux-gnu-gcc --runner 'qemu-m68k -L /path/to/sysroot' \
    --output output/codegen-m68k
```

`--cc` accepts a quoted compiler command. A wrapper can supply target-specific
flags. Explicit `CC_<backend-triple>` environment variables are honored by
mrustc and must agree with `--cc`. `--compile-only` builds without executing;
it is useful for separately tested machines and does not establish runtime
success. The minimal `support.rs` supplies only the language items these test
fixtures need for Rust 1.90. It is not an alternative core library.
