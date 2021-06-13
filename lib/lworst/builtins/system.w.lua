
return function(i)

i:define("env-get", function(i)
    local name = i:stack_ref(1, "string")
    local value = os.getenv(name) or false
    i:stack_push(value)
end)

end

