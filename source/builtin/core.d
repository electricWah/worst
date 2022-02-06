
module builtin.core;

import base;
import list;
import interpreter;
import std.functional;
import std.stdio;

auto drop(Interpreter i) {
    i.stack.pop();
}

@("drop") unittest {
    auto i = new Interpreter(new List([new Symbol("drop")]));
    i.define("drop", &drop);
    i.stack.push(new base.Number(5));
    assert(!i.stack.empty);
    assert(i.run.isNull);
    assert(i.stack.empty);
}

