
open Worst_base;;

Builtin.define_type (module Type.In_channel);;
Builtin.define_type (module Type.Out_channel);;

module InS = Builtin.Stack(Type.In_channel);;
module OutS = Builtin.Stack(Type.Out_channel);;

module InPortS = Builtin.StackPop(Base.ValInputPort);;
module OutPortS = Builtin.StackPop(Base.ValOutputPort);;

Builtin.(define "stdin-port" (S.In_channel.push stdin));;
Builtin.(define "stdout-port" (S.Out_channel.push stdout));;
Builtin.(define "stderr-port" (S.Out_channel.push stderr));;

Builtin.(define "port-read-all->string" @@ begin
    let* p = S.InputPort.pop in
    let buf = Buffer.create 1000 in
    let _len = Base.ValInputPort.read_into_buffer p buf in
    let s = Buffer.contents buf in
    S.String.push s
end);;

Builtin.(define "port-write-string" @@ begin
    let* s = S.String.pop in
    let* p = S.OutputPort.pop in
    S.Int.push (Base.ValOutputPort.write_string p s)
end);;

Builtin.(define "port-flush" @@ begin
    let* p = S.OutputPort.pop in
    Base.ValOutputPort.flush p;
    ok
end);;

