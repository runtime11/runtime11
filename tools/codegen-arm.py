#!/usr/bin/python3
"""Code Generator for ARM

This script is used to generate some of the sources of this crate. It currently
generates the following data:

   * ARM Syscall Numbers
     The system call numbers for ARM are kept in a simple table in the linux
     kernel sources. This table contains the system-call number, the ABI its
     included in, the system call name, and possibly the system call entry
     point.
     We fetch this table from `git.kernel.org`, parse it, and then generate the
     system call number definitions for the rust crate.
"""


import argparse
import sys
import urllib.request


def systbl_fetch(*, args):
    """Fetch Syscall Table

    Fetch the ARM syscall-table from the official git repository.
    """

    host = "git.kernel.org"
    repo = "pub/scm/linux/kernel/git/torvalds/linux.git"
    path = "arch/arm/tools/syscall.tbl"
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

    The syscall table contains entries in the style of:
        <number>  <abi>   <name>  <entry point>  <compat entry point>
    For instance:
        0         eabi    read    sys_read       compat_sys_read
    The file can contain comments, and anything but the first three columns
    are optional.

    Valid ABI identifiers are `oabi`, `eabi`, and `common`. We are not
    interested in `oabi` values.
    """

    res = []

    lines = data.decode().splitlines()
    for line in lines:
        line = line.replace("\t", " ").strip()
        if not line or line.startswith("#"):
            continue

        fields = line.split()
        assert len(fields) >= 3

        if fields[1] in ["common", "eabi"]:
            res.append(fields)

    return sorted(res, key=lambda v: int(v[0]))


def systbl_emit(*, args, systbl):
    """Emit Rustified Syscall Table

    Emit rust code as expected by the crate, which contains the
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
    systbl_emit(args=args, systbl=systbl)


def parse_args(argv):
    parser = argparse.ArgumentParser(
        add_help=True,
        allow_abbrev=False,
        argument_default=None,
        description="Code Generator for ARM",
        prog="codegen-arm.py",
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
