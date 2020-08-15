
# The Worst Programming Language

Worst is a simple, malleable programming language
built for extensibility and creativity.
Its syntax system allows you to redefine the entire language as you wish.

More information is available on
[the Worst homepage](http://worst.mitten.party).

## Try it

Run `rlwrap ./lworsti.sh` to open the interactive interpreter.
Requires LuaJIT.

## Build it

The interpreter can also be built as a self-contained executable,
`lworsti`, with `make lworsti`.

Requires LuaJIT, LuaRocks, `pkg-config`, and `minizip.a` (from zlib).
Only tested on NixOS so far.

The resulting executable is a zip archive.
If you'd like to include extra Worst modules,
you can copy them into `lib/` before building.
You can also copy data files into `build/libworst.zip` and rebuild.

Also, the un-bundled binary (without the zip) is
available in the bundle under `bin/lworsti`,
so you can build another self-contained executable by unzipping it,
altering the contents, zipping it up again,
and gluing everything back together with
`cat bin/lworsti bundle.zip > lworsti`.
This source repository is not required for this method.

Open bundled files with e.g. `"%/test.txt" open-input-file`.

## License

GPLv3.

The compiled executable contains binary code from LuaJIT - see bundle/LICENSE.

