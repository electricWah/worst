
open Ty

type state =
    | Space
    | Comment
    | Hash
    | Atom
    | String of { escaping: bool }

type list_state = {
    opening: char;
    closing: char;
    data: Val.t Queue.t;
}

type t = {
    list_state: list_state list;
    state: state;
    parsing: Buffer.t;
    output: Val.t Queue.t;
}

let empty () = {
    list_state = [];
    state = Space;
    parsing = Buffer.create 60;
    output = Queue.create ();
}

let is_whitespace = function
    | ' ' | '\x0C' | '\n' | '\r' | '\t' -> true
    | _ -> false

let is_nonatomic = function
    | ';' | '"' | '(' | ')' | '[' | ']' | '{' | '}' -> true
    | _ -> false

let string_escape_char = function
    | 'e' -> '\x1b'
    | 'n' -> '\n'
    | 'r' -> '\r'
    | 't' -> '\t'
    | c -> c

exception Unmatched of char
exception Unknown_hash of char

let emit r v =
    let q = match r.list_state with { data; _ } :: _ -> data | [] -> r.output in
    Queue.push v q

let start_state r c =
    let start_list opening closing =
        { r with list_state = { opening; closing; data = Queue.create() } :: r.list_state }
    in
    let end_list c =
        match r.list_state with
        | { closing; data; _ } :: list_state ->
            if c = closing then
                let r' = { r with list_state } in
                emit r' (V.List.to_val (List.of_seq (Queue.to_seq data)));
                r'
            else raise (Unmatched c)
        | [] -> raise (Unmatched c)
    in
    match c with
    | ';' -> { r with state = Comment }
    | '"' -> { r with state = String { escaping = false; } }
    | '#' -> { r with state = Hash }
    | '(' -> start_list '(' ')'
    | '[' -> start_list '[' ']'
    | '{' -> start_list '{' '}'
    | ')' | ']' | '}' -> end_list c
    | _ ->
        Buffer.add_char r.parsing c;
        { r with state = Atom }

let parse_atom s =
    (* use sscanf_opt in ocaml 5.0+ *)
    try Scanf.sscanf s "%i!" V.Int.to_val with
    Scanf.Scan_failure _ | Failure _ ->
    try Scanf.sscanf s "%f!" V.Float.to_val with
    Scanf.Scan_failure _ | Failure _ ->
        V.Symbol.to_val s

let rec read_seq r (nc: unit -> char option) =
    match r.state with
    | Space ->
        let rec spaces r =
            match nc() with
            | None -> r
            | Some c ->
                if is_whitespace c
                then spaces r
                else read_seq (start_state r c) nc
        in spaces r
    | Comment ->
        let rec comment r =
            match nc() with
            | None -> r
            | Some c ->
                if c = '\n'
                then read_seq { r with state = Space } nc
                else comment r
        in comment r
    | Hash -> begin
        match nc() with
        | None -> r
        | Some '!' -> read_seq { r with state = Comment } nc
        | Some (('t'|'f') as c) ->
            emit r (V.Bool.to_val (c = 't'));
            read_seq { r with state = Space } nc
        | Some c -> raise (Unknown_hash c)
    end
    | Atom ->
        let rec atom r =
            match nc() with
            | None -> r
            | Some c ->
                let na = is_nonatomic c in
                if na || is_whitespace c then begin
                    let contents = Buffer.contents r.parsing in
                    Buffer.reset r.parsing;
                    emit r (parse_atom contents);
                    let r' = { r with state = Space } in
                    let r' = if na then start_state r' c else r' in
                    read_seq r' nc
                end else begin
                    Buffer.add_char r.parsing c;
                    atom r
                end
        in atom r
    | String { escaping } ->
        let rec stringy escaping r =
            match nc() with
            | None -> { r with state = String { escaping } }
            | Some c ->
                if escaping then begin
                    Buffer.add_char r.parsing (string_escape_char c);
                    stringy false r
                end else if c = '"' then begin
                    let contents = Buffer.contents r.parsing in
                    Buffer.reset r.parsing;
                    emit r (V.String.to_val contents);
                    read_seq { r with state = Space } nc
                end else if c = '\\' then begin
                    stringy true r
                end else begin
                    Buffer.add_char r.parsing c;
                    stringy false r
                end
        in stringy escaping r

let next_val ?(eof=false) r =
    match Queue.take_opt r.output with
    | Some o -> Some o
    | None ->
        if eof then
            match r.list_state with
            | { opening; _ } :: _ -> raise (Unmatched opening)
            | [] ->
                match r.state with
                | Atom -> begin
                    let contents = Buffer.contents r.parsing in
                    Buffer.reset r.parsing;
                    Some (parse_atom contents)
                end
                | Hash -> raise (Unmatched '#')
                | String _ -> raise (Unmatched '"')
                | _ -> None
        else None

let all_vals ?eof r =
    let unfolder () =
        match next_val ?eof r with
        | Some v -> Some (v, ())
        | None -> None
    in
    Seq.unfold unfolder ()

