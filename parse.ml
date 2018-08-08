type parse_error =
  | SyntaxError of string * Lexing.position
  | ParseError of message option * Lexing.position * Lexing.position
and message = string

exception Error of parse_error

let position {Lexing.pos_fname; pos_lnum; pos_cnum; pos_bol} =
  let file = pos_fname in
  let line = pos_lnum in
  let character = pos_cnum - pos_bol in
  (file, line, character)
;;

let report_error errors =
  List.iter prerr_endline errors;
  prerr_endline "while parsing."
;;

let parse fn lexbuf =
  let open MenhirLib.General in
  let module Interp = Parser.MenhirInterpreter in
  let input = Interp.lexer_lexbuf_to_supplier Lexer.token lexbuf in
  let success result = `Success result in
  let failure checkpoint = begin try
    let env = match checkpoint with
    | Interp.HandlingError env -> env
    | _ -> assert false in
    match Interp.stack env with
    | lazy Nil -> assert false
    | lazy (Cons (Interp.Element (state, _, start_pos, end_pos), _)) ->
            (* todo: the fucking .messages thing *)
            let message = Some("u done fucked") in
            raise (Error (ParseError (message, start_pos, end_pos)))
  with
    | Error err -> begin match err with
        | SyntaxError (invalid_input, err_pos) -> `Error (0, "lol")
        | ParseError (message, start_pos, end_pos) -> begin
            let _, start_line, start_char = position start_pos in
            let _, curr_line, curr_char = position end_pos in
            let lines =
              if curr_line = start_line then Printf.sprintf "line %d" curr_line
              else Printf.sprintf "lines %d-%d" start_line curr_line in
            let chars =
              if curr_line = start_line then Printf.sprintf "chars %d-%d" start_char curr_char
              else Printf.sprintf "char %d" start_char in
            let buf = Buffer.create 128 in
            Printf.bprintf buf "Parsing error: %s, %s:\n" lines chars;
            begin match message with
              | None -> ()
              | Some msg -> Printf.bprintf buf "\n%s\n" msg
            end;
            `Error (start_line, Buffer.contents buf)
        end
      end
  end in
  try
    Interp.loop_handle success failure input
      (fn lexbuf.Lexing.lex_curr_p)
  with
    Lexer.Error (input, pos) -> raise (Error (SyntaxError (input, pos)))

