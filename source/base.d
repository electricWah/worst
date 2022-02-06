
import std.conv: to;
import std.typecons: Nullable;
import std.container.array: Array;

interface Value {
    // @safe: // nothrow:
    bool equal(Value);
}

class BasicValue(T): Value {
    T v;
    @safe: nothrow:
    this(T v) { this.v = v; }
    bool equal(Value v) {
        if (auto that = cast(BasicValue!T)v) {
            return this.v == that.v;
        } else {
            return false;
        }
    }
    // override string toString() => v.to!string;
}

// builtin basic types
// I'm sure these can be templated or something
class Symbol: Value {
    string v;
    @safe: nothrow:
    this(string thing) { v = thing; }
    string value() => v;
    override string toString() => v;
    bool equal(Value v) {
        if (auto that = cast(Symbol)v) {
            return this.v == that.v;
        } else {
            return false;
        }
    }
}
alias Bool = BasicValue!bool;
alias String = BasicValue!string;
alias Number = BasicValue!int;

class Stack(T) {
    @safe:
    Array!T data;
    bool empty() => data.empty;
    @trusted void push(T v) {
        data.insert(v);
    }
    nothrow @trusted Nullable!T pop() {
        if (!data.empty) {
            try {
                auto v = data.removeAny;
                return typeof(return)(v);
            } catch (Throwable) {
                // TODO unreachable thanks to !.empty
                return typeof(return)();
            }
        } else {
            return typeof(return)();
        }
    }

    T top()
        in { assert(data.length > 0); }
        do { return data[$-1]; }

    Nullable!T peek() {
        if (!data.empty) {
            return typeof(return)(top);
        } else {
            return typeof(return)();
        }
    }
}

