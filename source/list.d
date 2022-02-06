
import std.array;
import std.range;
import std.typecons: Nullable;
import std.algorithm.mutation: reverse;

import base;
import base: Value;

/// Immutable lists
class List: Value {
    // just dirt simple full copy
    Value[] data = [];

    // @safe: // nothrow:

    this() { this.data = []; }
    this(Value[] data) {
        this.data = data.dup.reverse;
    }
    this(R)(R data) if (isInputRange!R && is(ElementType!R == Value)) {
        this.data = data.array.reverse;
        import std.stdio;
        // writeln(this);
    }

    bool empty() const { return data.length == 0; }
    size_t length() const { return data.length; }

    @("constructors and length") unittest {
        assert((new List).empty);
        assert((new List).length == 0);
        assert(!(new List([new List])).empty);
        assert((new List([new List])).length == 1);
    }

    @system override string toString() {
        import std.array: join;
        import std.algorithm: map;
        import std.range: retro;
        return "(" ~ map!(d => (cast(Object)d).toString)(data).retro.join(" ") ~ ")";
    }

    @("toString") unittest {
        assert(new List().toString == "()");
        assert(new List([new List]).toString == "(())");
        assert(new List([new List, new List]).toString == "(() ())");
        assert(new List([new List([new List]), new List]).toString == "((()) ())");
    }

    bool equal(Value v) {
        if (auto that = cast(List)v) {
            if (this.length != that.length) return false;
            for (int i = 0; i < this.length; i++) {
                if (!this.data[i].equal(that.data[i])) return false;
            }
            return true;
        } else {
            return false;
        }
    }
    
    @("equal") unittest {
        auto a = new List;
        assert(a.equal(a));
        assert(!a.equal(new base.Bool(true)));

        auto b = new List;
        assert(a.equal(b));

        auto c = new List([a]);
        auto d = new List([a]);
        assert(c.equal(d));

        auto e = new List([a, b]);
        assert(!d.equal(e));
    }

    List clone() {
        return new List(this.data);
    }

    List push(Value v) {
        auto l = clone();
        l.data.length++;
        l.data[$-1] = v;
        return l;
    }

    @("push") unittest {
        auto l = new List;
        assert((l.push(new List)).length == 1);
        assert(l.length == 0);
    }

    Nullable!Value pop(out List self) {
        if (this.data.length > 0) {
            auto v = this.data[$-1];
            self = this.clone();
            self.data.length--;
            /* return Nullable!Value(v); */
            return typeof(return)(v);
        } else {
            self = new List;
            return typeof(return)();
        }
    }

    @("pop empty") unittest {
        auto l = new List;
        List l2;
        assert(l.pop(l2).isNull);
        assert(l.pop(l2).isNull);
        assert(l.length == 0);
        assert(l2.length == 0);
    }
    @("pop") unittest {
        auto l = new List([new base.Bool(true)]);
        List l2;
        auto o = l.pop(l2);
        assert(!o.isNull);
        assert(o.get.equal(new base.Bool(true)));
        assert(l.length == 1);
        assert(l2.length == 0);
    }
    @("pop self") unittest {
        auto l = new List([new base.Bool(true)]);
        assert(l.length == 1);
        auto o = l.pop(l);
        assert(!o.isNull);
        assert(o.get.equal(new base.Bool(true)));
        assert(l.length == 0);
    }

}

