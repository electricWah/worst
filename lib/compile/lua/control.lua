
local base = require "lworst/base"
local Type = base.Type
local List = require "lworst/list"

local eval = require "compile/evaluate"

local luabase = require "compile/lua/base"
local Expr = luabase.Expr

local mod = {}

function mod.emit_if_then_else(ctx, i, ifcond, iftbody, iffbody)

    local tstmts, tinputs, toutputs = ctx:evaluate(i, iftbody)

    local tilen = tinputs:length()
    local tolen = toutputs:length()
    local tarity = toutputs:length() - tinputs:length()

    local fstmts, finputs, foutputs = ctx:evaluate(i, iffbody, tinputs)

    local filen = finputs:length()
    local folen = foutputs:length()
    local farity = foutputs:length() - finputs:length()

    if tarity ~= farity then
        i:error("true and false arms have different arity",
                tarity, farity)
    end

    local arglen = math.max(tilen, tolen, filen, folen)

    local invars = tinputs
    if filen > tilen then invars = finputs end
    invars = invars:to_table()

    -- replace as many invars with stack values as possible
    local invals = {}
    for _, iv in List.ipairs(invars) do
        if i:stack_length() > 0 then
            table.insert(invals, ctx:stack_pop(i))
        else
            table.insert(invals, iv)
        end
    end

    -- declare extra outputs
    local nouts = math.max(0, arglen - math.max(tilen, filen))
    local outvars = {}
    for n = 1, nouts do
        table.insert(outvars, ctx:gensym("ifout"))
    end

    local ifargs = {}
    for _, v in ipairs(outvars) do table.insert(ifargs, v) end
    for _, v in ipairs(invals) do table.insert(ifargs, v) end

    -- Assign output vals for both arms
    local utargs, utouts = luabase.unique_pairs(ifargs, toutputs)
    table.insert(tstmts, {luabase.assignment(utargs, utouts, false, tolen)})
    local ufargs, ufouts = luabase.unique_pairs(ifargs, foutputs)
    table.insert(fstmts, {luabase.assignment(ufargs, ufouts, false, folen)})

    -- Nothing to do inside either arm? Don't emit anything
    -- (might uncomment this if it happens a lot)
    if #tstmts == 0 and #fstmts == 0 then return end

    -- Declare out-only vars
    ctx:emit(luabase.assignment(outvars, {}, true))

    -- Init input vars
    local uins, uvals = luabase.unique_pairs(invars, invals)
    ctx:emit(luabase.assignment(uins, uvals, true))

    -- Convert empty true arm to empty false arm
    -- if expr then else ... end -> if not expr then ... else end
    if #tstmts == 0 then
        ifcond = luabase.syntax["not"](ifcond)
        tstmts, fstmts = fstmts, tstmts
    end


    ctx:emit("if ", luabase.value_tostring_prec(ifcond), " then")

    ctx:indent()
    for _, s in ipairs(tstmts) do ctx:emit(unpack(s)) end
    ctx:unindent()

    -- Convert "else end" into nothing
    if #fstmts > 0 then
        ctx:emit("else")
        ctx:indent()
        for _, s in ipairs(fstmts) do ctx:emit(unpack(s)) end
        ctx:unindent()
    end

    ctx:emit("end")

    -- leave outputs on stack
    while #ifargs > 0 do
        i:stack_push(table.remove(ifargs))
    end

end

-- [ ... -> bool ] cil/lua/loop
-- keep doing body while its top value is true
function mod.emit_loop(i, body)

    error("it brokem ):")
    local stmts, ins, outs = eval.evaluate(i, body)

    local ilen = ins:length()
    local olen = outs:length()
    if olen ~= ilen + 1 then
        return i:error("in arity must be out arity - 1", ilen, olen)
    end

    local invars = {}
    for _, iv in List.ipairs(ins) do
        if i:stack_length() > 0 then
            table.insert(invars, i:stack_pop())
        else
            table.insert(invars, iv)
        end
    end

    local ocont
    outs, ocont = outs:pop()
    local outvars = outs:to_table()

    local uargs, uouts = luabase.unique_pairs(invars, outvars)
    table.insert(stmts, {luabase.assignment(uargs, uouts, false, ilen)})

    eval.emit(i, {"repeat"})

    eval.indent(i)
    for _, s in List.ipairs(stmts) do eval.emit(i, s) end
    eval.unindent(i)

    local continue = luabase.syntax["not"](ocont)
    local condstr = luabase.value_tostring_prec(continue)
    eval.emit(i, {"until ", condstr})

    while #outvars > 0 do
        i:stack_push(table.remove(outvars))
    end

end

function mod.emit_break(i) eval.emit(i, {"break"}) end

function mod.emit_function_res(ctx, name, stmts, ins, outs)
    local fvar
    if base.Symbol.is(name) then
        fvar = ctx:gensym(luabase.value_tostring_prec(name))
        name = luabase.value_tostring_prec(fvar)
    elseif base.Type.is("string", name) then
        fvar = S(name)
    else
        fvar = name or ctx:gensym("func")
        name = luabase.value_tostring_prec(fvar)
    end
    local head = {"function ", name, "("}
    luabase.csv_into(head, ins)
    table.insert(head, ")")

    ctx:emit(head)
    ctx:indent()
    for _, s in ipairs(stmts) do ctx:emit(s) end
    if outs:length() > 0 then
        local r = {"return "}
        luabase.csv_into(r, outs)
        ctx:emit(r)
    end
    ctx:unindent()
    ctx:emit({"end"})

    return fvar, List.length(ins), List.length(outs)
end

-- recursive functions (local function ...) are a different construct
-- body name cil/lua-function
-- [ body ] name cil/lua-function => function name() ... end
-- [ body ] #f cil/lua-function => local func1 = function() ... end
-- in either case, the function value itself is put on the stack after
function mod.emit_function(ctx, i, name, body)
    local stmts, ins, outs = ctx:evaluate(i, body)
    return mod.emit_function_res(ctx, name, stmts, ins, outs)
end

return mod

-- ; init limit step [ body : ... var -> ... ] cil/lua-for-iter =
-- ; for var = init, limit, step do body end
-- define cil/lua-for-iter [
--     const %fibody

--     cil/expect-value const %fistep
--     cil/expect-value const %filimit
--     cil/expect-value const %fiinit

--     ; enter emit state and eval chunk to get name of input var for body
--     cil/indent>
--     cil/enter-new-emit-state

--     %fibody
--     ; interpreter-dump-stack
--     cil/eval-chunk
--     list-length const ilen const args
--     list-length const olen const outs

--     ilen olen 1 add equal? if [drop drop] [
--         interpreter-dump-stack
--         ["cil/lua-for-iter: wrong arity"] abort
--     ]

--     args list-pop const carg
--     const args

--     cil/do-unindent [
--         cil/emit-state-do-uplevel [
--             define S [cil/expr->string]
--             ["for " carg S " = " %fiinit S ", " %filimit S
--                 %fistep equals? 1 if [drop] [", " swap S]
--                 " do"]
--             list-eval cil/emit-statement
--         ]
--     ]

--     outs args #f cil/emit-assignment

--     cil/indent<
--     cil/leave-emit-state

--     ["end"] cil/emit-statement


-- ]
-- export-name cil/lua-for-iter





