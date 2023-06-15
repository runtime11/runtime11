#!/usr/bin/python3
"""Code Generator for Common Code

This script is used to generate some of the platform-independent sources of
this project. It currently generates the following data:

   * Generic Syscall Numbers
     The linux kernel sources provide `include/uapi/asm-generic/unistd.h` with
     proposed system call numbers for new architectures. The system-call
     numbers are extracted by this tool and then provided as rust definitions.
"""


import argparse
import sys
import urllib.request


def systbl_fetch(*, args):
    """Fetch Syscall Table

    Fetch the asm-generic unistd.h header from the official git repository.
    """

    host = "git.kernel.org"
    repo = "pub/scm/linux/kernel/git/torvalds/linux.git"
    path = "include/uapi/asm-generic/unistd.h"
    branch = "master"

    url = "".join([
        "https://",
        host,
        "/", repo,
        "/", "plain",
        "/", path,
        "?h=", branch,
    ])

    print("Fetching from:", url, file=sys.stderr)

    with urllib.request.urlopen(url) as req:
        systbl = req.read()

    return systbl


def systbl_parse(*, args, data):
    """Parse Syscall Table

    The system-calls of asm-generic are provided in unistd.h. There is lots of
    conditionals and compat-syscall handling, but we ignore all that. We simply
    fetch all `__NR*` definitions that expand to a syscall-number. Aliases and
    other definitions are ignored.

    This is rather fragile and not a proper parser of the C pre-processor
    language. Hence, human verification of the result is recommended. A simple
    diff to the previous output should suffice to check for errors.
    """

    res = []

    lines = data.decode().splitlines()
    for line in lines:
        line = line.replace("\t", " ").strip()
        fields = line.split()

        # We are looking for lines like this:
        #     #define __NR<str> <digits>
        if (
            len(fields) < 3 or
            fields[0] != "#define" or
            not fields[1].startswith("__NR") or
            not fields[2].isdigit()
        ):
            continue

        # The syscall name can be prefixed with `__NR_` or `__NR3264_`. Strip
        # this from the name here.
        fields[1] = fields[1].removeprefix("__NR_")
        fields[1] = fields[1].removeprefix("__NR3264_")

        res.append(
            [
                fields[2],
                "common",
                fields[1],
            ]
        )

    return sorted(res, key=lambda v: int(v[0]))


def systbl_emit(*, systbl):
    """Emit Rustified Syscall Table

    Emit rust code as expected by the project, which contains the
    definitions of the system call numbers.
    """

    print("// This code is generated.")
    for entry in systbl:
        print(f"pub const {entry[2].upper()}: u32 = {entry[0]};")


def systbl(args):
    print("Fetch System Table...", file=sys.stderr)
    data = systbl_fetch(args=args)
    print("Parse System Table...", file=sys.stderr)
    systbl = systbl_parse(args=args, data=data)
    print("Emit System Table...", file=sys.stderr)
    systbl_emit(systbl=systbl)


def parse_args(argv):
    parser = argparse.ArgumentParser(
        add_help=True,
        allow_abbrev=False,
        argument_default=None,
        description="Code Generator for Common Code",
        prog="codegen-common.py",
    )

    parser.add_argument(
        "--generate",
        choices=["systbl"],
        help="What to generate",
        required=True,
        type=str,
    )

    return parser.parse_args(argv[1:])


def run(argv):
    args = parse_args(argv)

    if args.generate == "systbl":
        systbl(args)
    else:
        raise RuntimeError("Nothing to do")


if __name__ == "__main__":
    run(sys.argv)
