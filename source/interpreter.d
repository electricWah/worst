
import std.typecons: Nullable, nullable;
import std.sumtype: SumType, match;
import std.concurrency: Generator, yield;
import base;
import base: Value, Stack;
import list: List;

import std.stdio;

nothrow:

alias Builtin = void delegate(Interpreter) nothrow;
// needs to be a wrapper for some reason
private class Definition {
    // @safe: nothrow:
    SumType!(List, Builtin) the;
    this(List v) { the = v; }
    this(Builtin v) { the = v; }

    private ChildFrame start(Interpreter target) {
        return the.match!(
            (List l) => cast(ChildFrame) new ListFrame(l),
            (Builtin b) => new PausedFrame(() => b(target)),
        );
    }
}

alias FrameYield = SumType!(Value, ChildFrame);
private class ChildFrame {}

private class ListFrame: ChildFrame {
    auto childs = new Stack!ChildFrame;
    auto body = new List;
    Definition[string] defs;
    this() {}
    this(List l) {
        body = l;
    }
    
    @("create") unittest {
        auto t = new ListFrame(new List);
        t.defs["test"] = new Definition(new List);
    }
}

private class PausedFrame: ChildFrame {
    Generator!FrameYield body;
    this(void delegate() b) {
        body = new Generator!FrameYield(b);
    }
    bool empty() => body.empty;
    FrameYield next() {
        auto r = body.front;
        body.popFront();
        return r;
    }
}

class Interpreter: Value {
    auto parents = new Stack!ListFrame;
    auto stack = new Stack!Value;
    Stack!Definition[string] defstacks;
    auto frame = new ListFrame;

    this() { }
    this(List body) {
        this.frame.body = body;
    }

    bool equal(Value that) { return this is that; }

    Nullable!Value read_body() { return frame.body.pop(frame.body); }

    Nullable!Value run() {
        while (true) {
            auto f = frame.childs.pop();
            if (!f.isNull) {
                Nullable!Value r; // match! is a delegate
                if (auto l = cast(ListFrame)f.get) {
                    this.enter_child_frame(l);
                } else if (auto p = cast(PausedFrame)f.get) {
                    if (!p.empty) {
                        auto next = p.next;
                        next.match!(
                            (Value v) => { r = v; },
                            (ChildFrame c) => {
                                frame.childs.push(c);
                            },
                        );
                    }
                }
                if (!r.isNull) return r;
            } else {
                auto next = read_body();
                if (!next.isNull) {
                    if (auto s = cast(base.Symbol)next.get) {
                        frame.childs.push(new PausedFrame({ this.call(s); }));
                    } else {
                        stack.push(next.get);
                    }
                } else {
                    if (!enter_parent_frame()) {
                        return typeof(return)();
                    }
                }
            }
        }
    }

    void define(string name, List v) {
        frame.defs[name] = new Definition(v);
    }

    @("define list") unittest {
        auto i = new Interpreter;
        i.define("egg", new List);
    }

    void define(string name, Builtin v) {
        frame.defs[name] = new Definition(v);
    }

    void define(string name, void function(Interpreter) nothrow v) {
        import std.stdio;
        auto def = new Definition(i => v(i));
        frame.defs[name] = def;
    }

    void eval(Definition d) {
        SumType!(Value, ChildFrame) fy = d.start(this);
        yield!FrameYield(fy);
    }

    void call(base.Symbol s) {
        eval(resolve(s).get);
    }

    Nullable!Definition resolve(base.Symbol s) {
        auto name = s.value;
        if (name in frame.defs) {
            return frame.defs[name].nullable;
        } else if (name in defstacks) {
            // assume it has something since otherwise it would be removed
            return defstacks[name].top.nullable;
        } else {
            return Nullable!Definition();
        }
    }

    private bool enter_parent_frame() {
        auto fr = parents.pop();
        if (fr.isNull) return false;
        auto f = frame;
        frame = fr.get;

        foreach (name; frame.defs.byKey) {
            defstacks[name].pop();
            if (defstacks[name].empty) defstacks.remove(name);
        }

        if (!f.body.empty) frame.childs.push(f);
        return true;
    }

    private void enter_child_frame(ListFrame f) {
        foreach (name, def; frame.defs) {
            defstacks.require(name, new Stack!Definition).push(def);
        }
        parents.push(frame);
        frame = f;
    }

}

