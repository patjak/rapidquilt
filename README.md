# Rapidquilt

[![Build Status](https://travis-ci.org/openSUSE/rapidquilt.svg?branch=master)](
https://travis-ci.org/openSUSE/rapidquilt)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](
https://github.com/openSUSE/rapidquilt)
[![Rust 1.31+](https://img.shields.io/badge/rust-1.31+-lightgray.svg)](
https://www.rust-lang.org)

This is very specialized reimplementation of quilt & patch in one. It supports
only the `push` command. The goal is to be very fast.


## Usage

    Usage: rapidquilt push [<options>] [num|patch]

    Options:
        -a, --all           apply all patches in series

        -d, --directory DIR working directory

        -p, --patch-directory DIR
                            directory with patches (default: "patches")

        -b, --backup always|onfail|never
                            create backup files for `quilt pop`
                            (default: onfail)

            --backup-count all|<n>
                            amount of backup files for `quilt pop` to create
                            (default: 100)

        -F, --fuzz <n>      maximal allowed fuzz (default: 0)

            --color always|auto|never
                            use colors in output (default: auto)

            --dry-run       do not save any changes

        -A, --analyze ANALYSIS
                            run additional analysis while patching. You can use
                            this option multiple times to run multiple analyses at
                            once. Available analyses: multiapply

            --stats         print statistics in the end

        -q, --quiet         only print errors

        -v, --verbose       print extra information. Repeat for more verbosity. It
                            may affect performance.

            --mmap          mmap files instead of reading into buffers. This may
                            reduce memory usage and improve performance in some
                            cases. Warning: You must ensure that no external
                            program will modify the files while rapidquilt is
                            running, otherwise you may get incorrect results or
                            even crash.

        -h, --help          print this help menu


## Limitations compared to quilt & patch

* only the `push` command
* only patches in unified format
* date in patch files is ignored
* endlines must be the same in patch and patched file (e.g. both "\n" or both "\r\n") (always `--binary` mode)
* ... probably more that I don't know about

## Screenshot

Patch application failure:

![example-apply-failure](https://raw.githubusercontent.com/openSUSE/rapidquilt/master/doc/example-apply-failure.png "Example Apply Failure")
