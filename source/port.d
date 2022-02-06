
import std.typecons: Nullable;
import std.algorithm: min, max;

// maybe peek is Peekable and read is Readable

interface Seekable {
    void seek(int);
    void seek_abs(int);
}

interface Closable {
    void close();
}

interface InputPort {
    bool eof();
    string read(uint);
    string peek(uint);
    /* string read_line(bool include_eol); */

    /* Nullable!string peek_line(bool include_eol); */
}

interface SeekableInputPort: Seekable, InputPort {}

interface OutputPort {
    void write(string);
}

class StringReader: SeekableInputPort {
    auto buf = "";
    uint pos = 0;
    this(string data) { buf = data; }
    bool eof() => pos >= buf.length;

    invariant {
        assert(pos >= 0);
        assert(pos <= buf.length);
    }

    string read(uint len) {
        if (eof) { return ""; }
        auto op = pos;
        pos = buf.length.min(pos + len);
        return buf[op .. pos];
    }

    string peek(uint len) {
        if (eof) { return ""; }
        return buf[pos .. buf.length.min(pos + len)];
    }

    @("read, peek, eof") unittest {
        auto s = new StringReader("blorp");
        assert(s.pos == 0);
        assert(!s.eof);
        assert(s.peek(0) == "");
        assert(s.read(0) == "");
        assert(s.pos == 0);
        assert(s.read(1) == "b");
        assert(s.pos == 1);
        assert(s.read(0) == "");
        assert(s.pos == 1);
        assert(s.peek(2) == "lo");
        assert(s.read(2) == "lo");
        assert(s.pos == 3);
        assert(s.read(3) == "rp");
        assert(s.eof);
        assert(s.peek(1) == "");
        assert(s.read(1) == "");
    }

    void seek_abs(int c) { pos = buf.length.min(c.max(0)); }
    void seek(int c) { seek_abs(pos + c); }

}

class StringWriter: OutputPort {
    string buf;
    this() {}
    this(string data) { buf = data; }

    T to(T)() if (is(T == string)) => buf;
    T to(T)() if (is(T == StringReader)) => StringReader(buf);

    @("init") unittest {
        auto s = new StringWriter("blub");
        assert(s.to!string == "blub");
    }

    void write(string data) {
        buf ~= data;
    }

    @("write") unittest {
        auto s = new StringWriter;
        s.write("hello");
        assert(s.to!string == "hello");
    }

}

