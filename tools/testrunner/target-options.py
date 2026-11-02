#!/usr/bin/env python3
"""Check target aggregate floor validation and lossless target serialization."""
import argparse
from pathlib import Path
import subprocess
import tempfile
p=argparse.ArgumentParser(description=__doc__)
p.add_argument('--mrustc',type=Path,required=True)
a=p.parse_args()
base='''[target]
family="unix"
os-name="linux"
env-name="gnu"
arch="m68k"
[backend.c]
variant="gnu"
target="m68k-linux-gnu"
'''
with tempfile.TemporaryDirectory(prefix='mrustc-target-') as directory:
    d=Path(directory)
    for value in [None,1,2,4,8,16,0,3,17,-1]:
        source=d/'target.toml'
        source.write_text(base+('' if value is None else 'aggregate-min-align='+str(value)+'\n'))
        output=d/'out.toml'
        r=subprocess.run([str(a.mrustc.resolve()),'--target',str(source),
            '--dump-target-spec',str(output)],stdout=subprocess.PIPE,stderr=subprocess.PIPE,timeout=30)
        valid=value is None or value in [1,2,4,8,16]
        if valid:
            assert r.returncode==0, r.stderr.decode()
            expected='aggregate-min-align = '+str(value or 1)
            assert expected in output.read_text(), output.read_text()
            # A dumped specification must be accepted by the same reader.
            r=subprocess.run([str(a.mrustc.resolve()),'--target',str(output),
                '--dump-target-spec',str(d/'again.toml')],capture_output=True,timeout=30)
            assert r.returncode==0, r.stderr.decode()
            assert (d/'again.toml').read_bytes()==output.read_bytes()
        else:
            assert r.returncode!=0 and b'aggregate-min-align must be' in r.stderr, r.stderr.decode()
        print('aggregate-min-align='+str(value)+': PASS')
