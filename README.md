# DCPU-16 GUI

DCPU-16 emulator GUI written in Rust 1.0 and Piston.

## Example

Prints "Hello world!" in green to the screen:

    $ dcpu16-gui examples/hello.bin

To automatically attach a monitor to address `0x8000` (as in older programs),
run with the `-p` flag.

## Tools

To assemble `dasm16` files, use [dcpu16](https://github.com/gustavla/dcpu16).
