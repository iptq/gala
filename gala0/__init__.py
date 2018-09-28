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

    anf = p.convert()
    print("GENERATED LLVM IR ==========")
    print(ir)
    print("----------------------------")

    f = NamedTemporaryFile()
    f.write(bytes(ir, "utf-8"))
    f.flush()

    asm = NamedTemporaryFile(suffix=".S")
    p = Popen(["llc", f.name, "-o", asm.name], stdin=PIPE, stdout=PIPE, stderr=STDOUT, close_fds=True)
    stdout, stderr = p.communicate()
    f.close()
    print()
    print("LLC STDOUT =================")
    print(stdout.decode("utf-8"))
    if stderr:
        print("LLC STDERR =================")
        print(stderr.decode("utf-8"))
    print("----------------------------")
    if p.returncode != 0:
        asm.close()
        return p.returncode

    with open(out, "wb") as f:
        p = Popen(["gcc", asm.name, "-o", f.name], stdin=PIPE, stdout=PIPE, stderr=STDOUT, close_fds=True)
        stdout, stderr = p.communicate()
        asm.close()
        print()
        print("GCC STDOUT =================")
        print(stdout.decode("utf-8"))
        if stderr:
            print("GCC STDERR =================")
            print(stderr.decode("utf-8"))
        print("----------------------------")
        if p.returncode != 0:
            f.close()
            return p.returncode

    return 0
