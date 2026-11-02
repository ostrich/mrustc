#!/usr/bin/env python3
"""Small C-ABI/constant regression tests, without building target Rust libraries.

Run from any directory. For cross execution supply --target, --cc and --runner.
A custom target TOML can be used for a non-default C aggregate alignment floor;
the C compiler must actually implement that ABI. --compile-only retains binaries
for a separate machine and does not claim execution success.
"""
import argparse
import os
from pathlib import Path
import shlex
import subprocess

root = Path(__file__).resolve().parents[2]
p = argparse.ArgumentParser(description=__doc__)
p.add_argument('--mrustc', type=Path, default=root/'bin/mrustc')
p.add_argument('--cc', default='cc', help='C compiler command, shell-quoted if necessary')
p.add_argument('--target', help='mrustc target name or TOML path')
p.add_argument('--runner', default='', help='cross runner command, e.g. qemu-m68k -L SYSROOT')
p.add_argument('--compile-only', action='store_true')
p.add_argument('--output', type=Path, required=True)
p.add_argument('cases', nargs='*')
a = p.parse_args()
fixtures = root/'samples/test/auxiliary/codegen'
names = a.cases or sorted(f.stem for f in fixtures.glob('*.c'))
a.output = a.output.resolve()
a.output.mkdir(parents=True, exist_ok=False)
env = dict(os.environ, MRUSTC_TARGET_VER='1.90')
# mrustc expects CC to name a single executable, so use a local wrapper when
# compiler arguments are needed. No shell command supplied by source fixtures.
cc = a.output/'cc'
cc.write_text('#!/bin/sh\nexec ' + shlex.join(shlex.split(a.cc)) + ' "$@"\n')
cc.chmod(0o755)
env['CC'] = str(cc)
if a.target:
    # A named target may prefer CC_<triple>; explicitly inherited overrides are
    # still honored by mrustc. Do not alter unrelated process-global settings.
    env['CC_' + a.target.replace('-', '_')] = str(cc)
for name in names:
    if name not in {f.stem for f in fixtures.glob('*.c')}:
        p.error('unknown case: ' + name)
    d = a.output/name
    d.mkdir()
    lib = d/'case.rlib'
    command = [str(a.mrustc.resolve()), str(fixtures/(name+'.rs')),
        '--cfg', 'mrustc', '--edition', '2021', '--crate-type', 'rlib',
        '--crate-name', 'codegen_case', '-C', 'panic=abort', '-O', '-o', str(lib)]
    if a.target:
        command += ['--target', a.target]
    with (d/'log.txt').open('wb') as log:
        def run(cmd):
            log.write((shlex.join(cmd)+'\n').encode()); log.flush()
            subprocess.run(cmd, env=env, stdout=log, stderr=subprocess.STDOUT,
                check=True, timeout=120)
        run(command)
        binary = d/'case'
        # Only the placeholder test needs private types from emitted C.
        # Other harnesses use independent ABI declarations in a separate TU.
        inputs = (['-include', str(lib)+'.c'] if name == 'unit'
            else [str(lib)+'.o'])
        run(shlex.split(a.cc) + ['-std=gnu11', '-O2'] + inputs +
            [str(fixtures/(name+'.c')), '-o', str(binary)])
        if not a.compile_only:
            run(shlex.split(a.runner) + [str(binary)])
    print(name + (': COMPILED (not executed)' if a.compile_only else ': PASS'), flush=True)
