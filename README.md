# randlines

WIP: currently broken.

[![crates.io](https://img.shields.io/crates/v/randlines.svg)](https://crates.io/crates/randlines)

Print out random number of lines from a line oriented file. Pick up where
[shuf](https://www.gnu.org/software/coreutils/manual/html_node/shuf-invocation.html)
gets killed.

## Installation

```
$ cargo install randlines
```

## Usage

```shell
$ randlines -h
randlines 0.1.1

Emit a random subset of lines from a file. This is a probabilistic program, you
will not get exactly `n` lines.

Typically, you can use shuf(1) which uses reservoir sampling and is very
efficient. However, if we want to extract 10M random lines from a file of 100M
lines, shuf(1) might be killed. However, randlines will not shuffle lines, just
skip over random number of lines.

USAGE:
    randlines [OPTIONS] [input]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n <n>                          [default: 16]
    -s, --size-hint <size-hint>

ARGS:
    <input>
```

Emit a random subset of lines from a file. This is a probabilistic program, you
will not get exactly `n` lines.

Typically, you can use shuf(1) which uses reservoir sampling and is very
efficient. However, if we want to extract 10M random lines from a file of 100M
lines, shuf(1) might be killed. However, randlines will not shuffle lines, just
skip over random number of lines.

## TODO

* [ ] compress temporary output when reading from stdin
* [ ] make --size-hint actually work
