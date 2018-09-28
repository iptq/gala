import os
from subprocess import Popen, PIPE, STDOUT
from tempfile import NamedTemporaryFile

from gala0.parser import parser
from gala0.ast import *

def compile(data, out=None):
    if out is None:
        out = "a.out"

    raw_tree = parser.parse(data)
    print(raw_tree.pretty())

    p = Program(raw_tree)
    print(repr(p))

    ir = p.convert()
    print("===")
    print(ir)
    print("---")

    f = NamedTemporaryFile()
    f.write(bytes(ir, "utf-8"))
    f.flush()

    asm = NamedTemporaryFile(suffix=".S")
    p = Popen(["llc", f.name, "-o", asm.name], stdin=PIPE, stdout=PIPE, stderr=STDOUT, close_fds=True)
    output = p.stdout.read()
    f.close()
    print()
    print("LLC OUTPUT =================")
    print(output.decode("utf-8"))
    print("----------------------------")

    with open(out, "wb") as f:
        p = Popen(["gcc", asm.name, "-o", f.name], stdin=PIPE, stdout=PIPE, stderr=STDOUT, close_fds=True)
        output = p.stdout.read()
        asm.close()
        print()
        print("GCC OUTPUT =================")
        print(output.decode("utf-8"))
        print("----------------------------")
