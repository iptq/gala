type parse_error =
  | SyntaxError of string * Lexing.position
  | ParseError of string * Lexing.position * Lexing.position
  | Generic of string

exception Error of parse_error

let lexbuf_of_file (filename:string): Lexing.lexbuf =
  let mutate_position { Lexing.pos_fname; pos_lnum; pos_bol; pos_cnum } =
    { Lexing.pos_fname = filename; pos_lnum = pos_lnum; pos_bol = pos_bol; pos_cnum = pos_cnum; } in
  let ic = open_in filename in
  let lexbuf = Lexing.from_channel ic in
  lexbuf.lex_start_p <- mutate_position lexbuf.lex_start_p;
  lexbuf.lex_curr_p <- mutate_position lexbuf.lex_curr_p;
  lexbuf

let position {Lexing.pos_fname; pos_lnum; pos_cnum; pos_bol} =
  let file = pos_fname in
  let line = pos_lnum in
  let character = pos_cnum - pos_bol in
  (file, line, character)

let report_error errors =
  List.iter prerr_endline errors;
  prerr_endline "while parsing."

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
      | lazy Nil -> raise (Error (ParseError ("Unknown error", lexbuf.lex_start_p, lexbuf.lex_curr_p)))
      | lazy (Cons (Interp.Element (state, _, _, _), _)) ->
          let buf = Buffer.create 128 in
          Printf.bprintf buf "current state: %d\n" (Interp.current_state_number env);
          begin try
            let message = Parser_messages.message (Interp.number state) in
            Printf.bprintf buf "%s\n" message;
          with
            | Not_found -> ()
          end;
          raise (Error (ParseError (Buffer.contents buf, lexbuf.lex_start_p, lexbuf.lex_curr_p)))
  with
    | Error err -> begin match err with
        | Generic message -> `Error(0, message)
        | SyntaxError (invalid_input, err_pos) -> `Error (0, "lol")
        | ParseError (message, start_pos, end_pos) -> begin
            let buf = Buffer.create 128 in
            let file, start_line, start_char = position start_pos in
            let _, curr_line, curr_char = position end_pos in

            (* print preview *)
            begin try
              let ic = open_in file in
              let line_counter = ref 1 in
              let hl = ref false in
              let preview_start_line = start_line in
              let preview_end_line = curr_line in
              let start_hl = "\x1b[31;1;4m" in
              let end_hl = "\x1b[0m" in
              print_endline "\n";
              begin try
                while true; do
                  let line = input_line ic in
                  let len = String.length line in
                  if !line_counter >= preview_start_line && !line_counter <= preview_end_line then
                    begin
                      if !line_counter = start_line then
                        if !line_counter <> curr_line then (!hl = true; print_string ("  | " ^ (String.sub line 0 start_char) ^ start_hl ^ (String.sub line start_char (len - start_char)) ^ end_hl ^ "\n"))
                        else print_string ("  | " ^ (String.sub line 0 start_char) ^ start_hl ^ (String.sub line start_char (curr_char - start_char)) ^ end_hl ^ (String.sub line curr_char (len - curr_char)))
                      else
                        if !line_counter = curr_line then (!hl = false; print_string ("  | " ^ start_hl ^ (String.sub line 0 curr_char) ^ end_hl ^ (String.sub line curr_char (len - curr_char)) ^ "\n"))
                        else print_endline ("  | " ^ (if !hl then start_hl else "") ^ line ^ end_hl)
                    end;
                  incr line_counter;
                done;
              with
                End_of_file -> close_in ic;
              end;
              print_endline "\n";
            with
              _ -> () (* whatever *)
            end;
            
            (* line numbers *)
            let lines =
              if curr_line = start_line then Printf.sprintf "line %d" curr_line
              else Printf.sprintf "lines %d-%d" start_line curr_line in
            let chars =
              if curr_line = start_line then Printf.sprintf "chars %d-%d" start_char curr_char
              else Printf.sprintf "char %d" start_char in
            Printf.bprintf buf "Parsing error in '%s': %s, %s:\n" file lines chars;
            Printf.bprintf buf "(%d, %d) to (%d, %d)\n" start_line start_char curr_line curr_char;
            Printf.bprintf buf "\n%s\n" message;
            `Error (start_line, Buffer.contents buf)
        end
      end
  end in
  try
    Interp.loop_handle success failure input
      (fn lexbuf.Lexing.lex_curr_p)
  with
    Lexer.Error (input, pos) -> raise (Error (SyntaxError (input, pos)))

