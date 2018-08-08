open Intrinsics

let rec repl (state:Eval.state) =
  print_string ">> ";
  let input = read_line () in
  if input = "" then repl state
  else
    try
      let lexbuf = Lexing.from_string input in
      match Parse.parse Parser.Incremental.line lexbuf with
      | `Success stmt ->
          let state = Eval.eval_single state stmt in
          repl state
      | `Error (line, message) ->
          let lexbuf = Lexing.from_string input in
          begin match Parse.parse Parser.Incremental.exprline lexbuf with
          | `Success expr ->
              let (state, value) = Eval.eval_expr state expr in
              Eval.print_value value;
              repl state
          | `Error _ ->
              print_endline ("Error " ^ message ^ " on line " ^ (string_of_int line)); 
              repl state
          end
    with
    | Failure msg ->
        print_endline ("Failure: " ^ msg);
        repl state
    | Eval.RuntimeError msg ->
        print_endline ("RuntimeError: " ^ msg);
        repl state

let _ = repl [generate_intrinsics ()]
