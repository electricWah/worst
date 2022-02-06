
import std.stdio;
import std.typecons: Nullable;
import base;
import base: Value, Stack;
import list: List;
import std.range;
import std.uni: isWhite;
import std.conv: to, parse, ConvException;
import std.container.array;
import std.algorithm.mutation: reverse;

auto drop_blanks(T)(T src) {
    bool did_stuff = false;
    do {
        did_stuff = false;
        // whitespace
        while (!src.empty && src.front.isWhite) {
            did_stuff = true;
            src.popFront;
        }
        // ; comment \n
        if (!src.empty && src.front == ';') {
            did_stuff = true;
            while (!src.empty && src.front != '\n') {
                src.popFront;
            }
        }
        // #! comment !# ?
    } while (did_stuff);

    return src;
}

@("drop_blanks") unittest {
    assert("".drop_blanks == "");
    assert(" ; test \nok".drop_blanks.to!string == "ok");
}

struct Reader(T) {
    // should be a WithSourceLoc!T or something
    T src;
    // maybe an Either(Error,Value)
    Nullable!Value next;

    void trim() { src = src.drop_blanks; }
    bool empty() { trim(); return src.empty && next.isNull; }

    auto read_bool(ref Value r) {
        if (src.walkLength(2) < 2) return false;
        else switch (src.take(2).to!string) {
            case "#t":
                src.popFrontN(2);
                r = new base.Bool(true);
                return true;
            case "#f":
                src.popFrontN(2);
                r = new base.Bool(false);
                return true;
            default: return false;
        }
    }

    auto read_string(ref Value r) {
        // string must be at least ""
        if (src.walkLength(2) < 2) return false;
        else if (src.front != '"') return false;
        auto buf = "";
        src.popFront();
        quotes: while (true) {
            if (src.empty) return false; // or report error
            switch (src.front) {
                case '"':
                    src.popFront();
                    break quotes;
                case '\\':
                    src.popFront();
                    switch (src.front) {
                        case '"': buf ~= "\""; break;
                        case 'n': buf ~= "\n"; break;
                        default: buf ~= src.front;
                    }
                    src.popFront();
                    break;
                default:
                    buf ~= src.front;
                    src.popFront();
            }
        }
        r = new base.String(buf.to!string);
        return true;
    }

    auto read_int(ref Value r) {
        try {
            auto v = parse!(int, T, Yes.doCount)(src);
            r = new base.Number(v.data);
            return true;
        } catch (ConvException) {
            return false;
        }
    }

    auto read_list(ref Value r) {
        if (src.walkLength(2) < 2) return false;
        char endch;
        switch (src.front) {
            case '(': endch = ')'; break;
            case '[': endch = ']'; break;
            case '{': endch = '}'; break;
            default: return false;
        }
        src.popFront();
        Array!Value list;
        while (true) {
            trim();
            if (src.empty) return false;
            if (src.front == endch) {
                src.popFront();
                r = new List(list[]);
                return true;
            }
            auto f = front();
            if (f.isNull) return false;
            list.insert(f.get);
            popFront();
        }
    }

    auto read_symbol(ref Value r) {
        if (src.walkLength(1) < 1) return false;
        auto buf = "";
        symbol: while (!src.empty) {
            if (src.front.isWhite) break symbol;
            switch (src.front) {
                case '(': case ')':
                case '[': case ']':
                case '{': case '}':
                case '"':
                    break symbol;
                default:
                    buf ~= src.front;
                    src.popFront();
            }
        }
        if (buf.length > 0) {
            r = new base.Symbol(buf.to!string);
            return true;
        } else {
            return false;
        }
    }

    auto front() {
        if (!next.isNull) return next;
        trim();
        Value r;
        if (read_bool(r) ||
            read_list(r) ||
            read_string(r) ||
            read_int(r) ||
            read_symbol(r))
            next = r;
        return next;
    }

    auto popFront() {
        next.nullify();
    }
}
auto reader(T)(T r) => Reader!T(r);

@("read_bool") unittest {
    assert("".reader.empty);
    assert(" \n ; test\n".reader.empty);
    auto b = "  #t ".reader.front;
    assert(!b.isNull);
    assert((cast(base.Bool) b.get).v == true);
    // maybe figure out how to .map! from Reader
    auto r = "#f#t ;yeah\n #f #t ".reader;
    auto baseTrue = new base.Bool(true);
    auto baseFalse = new base.Bool(false);
    assert(!r.empty);
    assert(r.front.get.equal(baseFalse));
    r.popFront();
    assert(!r.empty);
    assert(r.front.get.equal(baseTrue));
    r.popFront();
    assert(!r.empty);
    assert(r.front.get.equal(baseFalse));
    r.popFront();
    assert(!r.empty);
    assert(r.front.get.equal(baseTrue));
    r.popFront();
    assert(r.empty);
}

@("read_string") unittest {
    auto r = "\"egg\" \"blub\\nbo\"\"\"#t \"okok\"".reader;
    assert(!r.empty);
    assert(r.front.get.equal(new base.String("egg")));
    r.popFront();
    assert(!r.empty);
    assert(r.front.get.equal(new base.String("blub\nbo")));
    r.popFront();
    assert(!r.empty);
    assert(r.front.get.equal(new base.String("")));
    r.popFront();
    assert(!r.empty);
    assert(r.front.get.equal(new base.Bool(true)));
    r.popFront();
    assert(!r.empty);
    assert(r.front.get.equal(new base.String("okok")));
    r.popFront();
    assert(r.empty);
}

@("read_int") unittest {
    assert("123".reader.front.get.equal(new base.Number(123)));
    auto r = "12#t34".reader;
    assert(!r.empty);
    assert(!r.front.isNull);
    assert(r.front.get.equal(new base.Number(12)));
    r.popFront();
    assert(!r.empty);
    assert(!r.front.isNull);
    assert(r.front.get.equal(new base.Bool(true)));
    r.popFront();
    assert(!r.empty);
    assert(!r.front.isNull);
    assert(r.front.get.equal(new base.Number(34)));
    r.popFront();
    assert(r.empty);
}

@("read_symbol") unittest {
    assert("eggs".reader.front.get.equal(new base.Symbol("eggs")));
    auto r = "time for-some .cool.beans".reader;
    assert(!r.empty);
    assert(!r.front.isNull);
    assert(r.front.get.equal(new base.Symbol("time")));
    r.popFront();
    assert(!r.empty);
    assert(!r.front.isNull);
    assert(r.front.get.equal(new base.Symbol("for-some")));
    r.popFront();
    assert(!r.empty);
    assert(!r.front.isNull);
    assert(r.front.get.equal(new base.Symbol(".cool.beans")));
    r.popFront();
    assert(r.empty);
}

@("read_list") unittest {
    auto r = "bean (bag muffins) ok{}[y(e p)s]".reader;
    assert(!r.empty);
    assert(!r.front.isNull);
    assert(r.front.get.equal(new base.Symbol("bean")));
    r.popFront();
    assert(!r.empty);
    assert(!r.front.isNull);
    assert(r.front.get.equal(new List([
                    new base.Symbol("bag"),
                    new base.Symbol("muffins"),
    ])));
    r.popFront();
    assert(!r.empty);
    assert(!r.front.isNull);
    assert(r.front.get.equal(new base.Symbol("ok")));
    r.popFront();
    assert(!r.empty);
    assert(!r.front.isNull);
    assert(r.front.get.equal(new List));
    r.popFront();
    assert(!r.empty);
    assert(!r.front.isNull);
    auto inner = new List([new base.Symbol("e"), new base.Symbol("p")]);
    assert(r.front.get.equal(new List([
                    cast(Value)new base.Symbol("y"),
                    inner,
                    new base.Symbol("s"),
    ])));
    r.popFront();
    assert(r.empty);
}

