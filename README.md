
# The Worst Programming Language

Worst is a simple, malleable programming language
built for extensibility and creativity.
Its syntax system allows you to redefine the entire language as you go.

Full documentation is available on
[the Worst homepage](http://worst.mitten.party).

## In this repository

#### `mworst`
A Rust program interpreting a subset of Worst
(no character-level syntax).
It provides a number of useful pre-defined functions and data types
including machine-sized integers, 64-bit floats,
booleans, symbols, strings, lists, hashtables, bytevectors,
_places_ (reference-counted storage slots),
environment variables, filesystem operations, and subprocess operations.

#### `worstc`
A wrapper program for `mworst`
that defines several useful procedures and concepts
such as `define`, `if`, `while`,
local variables,
a program history for debugging, error handling,
and a primitive library system.

#### Lua compiler library
A work-in-progress compiler library, currently targeting Lua.

Insert `target lua` at the top of a Worst source file
to have the program output Lua source code instead of executing.

## Installation

This implementation of Worst
should be considered alpha-quality software
and is not currently recommended for installation.

However, you can run `./worstc` from a checkout of this directory.
It requires a recent version of Rust, obtainable elsewhere.

## License

GPLv3


