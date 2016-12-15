[![Crates.io](https://img.shields.io/crates/v/dcpu16-gui.svg)](https://crates.io/crates/dcpu16-gui)

# DCPU-16 GUI

DCPU-16 emulator GUI written in Rust and Piston.

## Installation

Make sure you have Rust/Cargo and SDL2. One of these lines might help with the latter:

    $ brew install sdl2                 # macOS
    $ apt-get install libsdl2-dev       # Ubuntu

Now install DCPU-16 and DCPU-16-GUI through Cargo:

    $ cargo install dcpu16
    $ cargo install dcpu16-gui

This will install a variety of binaries that all start with `dcpu16`, so try
typing that in and hit tab.

## Example

Prints "Hello world!" in green to the screen:

    $ dcpu16-gui examples/hello.bin

To automatically attach a monitor to address `0x8000` (as in older programs),
run with `-m 0x8000`.

## Example 2

The next example is not assembled yet, so let's do that first:

    $ dcpu16-assembler examples/rainbow.dasm16 -o examples/rainbow.bin

Now we can run it:

    $ dcpu16-gui examples/rainbow.bin

## Tools

To assemble `dasm16` files into `bin` files, use [dcpu16](https://github.com/gustavla/dcpu16).
