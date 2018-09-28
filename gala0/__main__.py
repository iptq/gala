"""
Entrypoint for the compiler.
"""

import sys
import argparse
import gala0

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("file", help="The file to compile.")

    args = parser.parse_args()
    with open(args.file, "r") as f:
        source = f.read()

    sys.exit(gala0.compile(source))
