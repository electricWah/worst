
local base = require "lworst/base"
local Type = base.Type
local List = require "lworst/list"

local cil = require "cil/base"
local Expr = cil.Expr
local EvalContext = cil.EvalContext

local luabase = require "cil/lua/base"
local luaexpr = require "cil/lua/expr"

local S = base.Symbol.new

local mod = {}

function mod.emit_if_then_else(i, ifcond, iftbody, iffbody)
    EvalContext.expect(i, function(i, ectx)

        EvalContext.eval(i, iftbody, List.new{}, function(i, toutputs, tinputs, tstmts)
            local tilen = tinputs:length()
            local tolen = toutputs:length()
            local tarity = toutputs:length() - tinputs:length()

            EvalContext.eval(i, iffbody, tinputs, function(i, foutputs, finputs, fstmts)
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

                -- read all expected values (or vars if there aren't enough)
                local invals = ectx:expect_all(i, "ifv", invars)

                -- declare extra outputs
                local nouts = math.max(0, arglen - math.max(tilen, filen))
                local outvars = {}
                for n = 1, nouts do
                    table.insert(outvars, ectx:new_var("ifout"))
                end

                local ifargs = {}
                for _, v in ipairs(outvars) do table.insert(ifargs, v) end
                for _, v in ipairs(invals) do table.insert(ifargs, v) end

                -- Assign output vals for both arms
                local utargs, utouts = luabase.unique_pairs(ifargs, toutputs)
                table.insert(tstmts,
                    luabase.assignment(utargs, utouts, false, tolen))
                local ufargs, ufouts = luabase.unique_pairs(ifargs, foutputs)
                table.insert(fstmts,
                    luabase.assignment(ufargs, ufouts, false, folen))

                -- Nothing to do inside either arm? Don't emit anything
                -- (might uncomment this if it happens a lot)
                if #tstmts == 0 and #fstmts == 0 then return end

                -- Declare out-only vars
                luabase.emit_assignment(ectx, outvars, {}, true)

                -- Init input vars
                local uins, uvals = luabase.unique_pairs(invars, invals)
                luabase.emit_assignment(ectx, uins, uvals, true)

                -- Convert empty true arm to empty false arm
                -- if expr then else ... end -> if not expr then ... else end
                if #tstmts == 0 then
                    ifcond = luaexpr.lua["not"](ifcond)
                    tstmts, fstmts = fstmts, tstmts
                end

                ectx:emit_statement({
                    "if ", luabase.value_tostring_prec(ifcond), " then"
                })

                ectx:indent()
                for _, s in ipairs(tstmts) do ectx:emit_statement(s) end
                ectx:unindent()

                -- Convert "else end" into nothing
                if #fstmts > 0 then
                    ectx:emit_statement({"else"})
                    ectx:indent()
                    for _, s in ipairs(fstmts) do ectx:emit_statement(s) end
                    ectx:unindent()
                end

                ectx:emit_statement({"end"})

                -- leave outputs on stack
                while #ifargs > 0 do
                    i:stack_push(table.remove(ifargs))
                end

            end)
        end)
    end)
end

-- [ ... -> bool ] cil/lua/loop
-- keep doing body while its top value is true
function mod.emit_loop(i, body)
    EvalContext.expect(i, function(i, ectx)

        EvalContext.eval(i, body, List.new{}, function(i, outs, ins, stmts)
            local ilen = ins:length()
            local olen = outs:length()
            if olen ~= ilen + 1 then
                return i:error("in arity must be out arity - 1", ilen, olen)
            end

            local invars = ectx:expect_all(i, "loopv", ins)

            local ocont
            outs, ocont = outs:pop()
            local outvars = outs:to_table()

            local uargs, uouts = luabase.unique_pairs(invars, outvars)
            table.insert(stmts, luabase.assignment(uargs, uouts, false, ilen))

            ectx:emit_statement({"repeat"})

            ectx:indent()
            for _, s in ipairs(stmts) do ectx:emit_statement(s) end
            ectx:unindent()

            local continue = luaexpr.lua["not"](ocont)
            local condstr = luabase.value_tostring_prec(continue)
            ectx:emit_statement({"until ", condstr})

            while #outvars > 0 do
                i:stack_push(table.remove(outvars))
            end
        end)
    end)
end

function mod.emit_break(i)
    EvalContext.expect(i, function(i, ectx)
        ectx:emit_statement({"break"})
    end)
end

-- recursive functions (local function ...) are a different construct
-- body name cil/lua-function
-- [ body ] name cil/lua-function => function name() ... end
-- [ body ] #f cil/lua-function => local func1 = function() ... end
-- in either case, the function value itself is put on the stack after
function mod.emit_function(i)
    EvalContext.expect(i, function(i, ectx)
        local name = i:stack_pop()
        local body = i:stack_pop(List)
        EvalContext.eval(i, body, List.new{}, function(i, outs, ins, stmts)

            local fvar
            if base.Type.is("string", name) then
                fvar = S(name)
            else
                fvar = name or ectx:new_var("func")
                name = luabase.value_tostring_prec(fvar)
            end
            local head = {"function ", name, "("}
            luabase.csv_into(head, ins)
            table.insert(head, ")")

            ectx:emit_statement(head)
            ectx:indent()
            for _, s in ipairs(stmts) do ectx:emit_statement(s) end
            if outs:length() > 0 then
                local r = {"return "}
                luabase.csv_into(r, outs)
                ectx:emit_statement(r)
            end
            ectx:unindent()
            ectx:emit_statement({"end"})

            i:stack_push(fvar)

        end)
    end)
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




