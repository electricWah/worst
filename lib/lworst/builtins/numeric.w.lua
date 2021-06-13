
return function(i)

i:define("add", function(i)
    local a = i:stack_pop("number")
    local b = i:stack_pop("number")
    i:stack_push(a + b)
end)

i:define("mul", function(i)
    local a = i:stack_pop("number")
    local b = i:stack_pop("number")
    i:stack_push(a * b)
end)

i:define("negate", function(i)
    local a = i:stack_pop("number")
    i:stack_push(-a)
end)

i:define("recip", function(i)
    local a = i:stack_pop("number")
    i:stack_push(1 / a)
end)

i:define("modulo", function(i)
    local a = i:stack_pop("number")
    local b = i:stack_pop("number")
    i:stack_push(math.fmod(b, a))
end)

i:define("ascending", function(i)
    local a = i:stack_ref(1, "number")
    local b = i:stack_ref(2, "number")
    i:stack_push(a > b)
end)

end

