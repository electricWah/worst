
local Port = require "lworst/port"
local Reader = require "lworst/reader"

return function(i)

i:define("current-input-port", function(i)
    i:stack_push(Port.stdin())
end)

i:define("current-output-port", function(i)
    i:stack_push(Port.stdout())
end)

i:define("current-error-port", function(i)
    i:stack_push(Port.stderr())
end)

i:define("open-input-file", function(i)
    local path = i:stack_pop("string")
    local f, err = Port.open_input_file(path)
    if f then
        i:stack_push(f)
    else
        i:stack_push(err)
        i:stack_push(false)
    end
end)

i:define("open-output-file", function(i)
    local path = i:stack_pop("string")
    local f, err = Port.open_output_file(path)
    if f then
        i:stack_push(f)
    else
        i:stack_push(err)
        i:stack_push(false)
    end
end)

i:define("open-input-string", function(i)
    local s = i:stack_pop("string")
    i:stack_push(Port.InputPort.string(s))
end)

i:define("port-read-value", function(i)
    local port = i:stack_ref(1, Port.InputPort)
    local v = Reader.read_next(port)
    if v == nil then
        i:stack_push(Port.EOF)
    else
        i:stack_push(v)
    end
end)

i:define("port-has-data", function(i)
    local port = i:stack_ref(1, Port.InputPort)
    i:stack_push(port:has_buffered_data())
end)

i:define("port-peek-char", function(i)
    local port = i:stack_ref(1, Port.InputPort)
    i:stack_push(port:peek(1) or Port.EOF)
end)

i:define("port-read-char", function(i)
    local port = i:stack_ref(1, Port.InputPort)
    i:stack_push(port:read(1))
end)

i:define("port-read-line", function(i)
    local port = i:stack_ref(1, Port.InputPort)
    local s = port:read("*L")
    if s == nil then
        i:stack_push(false)
    else
        i:stack_push(s)
    end
end)

i:define("port-read-all", function(i)
    local port = i:stack_ref(1, Port.InputPort)
    local s = port:read("*a")
    if s == nil then
        i:stack_push(false)
    else
        i:stack_push(s)
    end
end)

i:define("port-write-string", function(i)
    local v = i:stack_pop("string")
    local port = i:stack_ref(1, Port.OutputPort)
    port:write_string(v)
end)

i:define("port-close", function(i)
    local port = i:stack_pop({ Port.InputPort, Port.OutputPort })
    port:close()
end)

i:define("eof-object?", function(i)
    local v = i:stack_ref(1)
    i:stack_push(v == Port.EOF)
end)

end

