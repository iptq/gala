open Ast
open Anf
open Lexer
open Lexing
open Parser

let rec repl () =
  print_string ">> ";
  let input = read_line () in
  if input = "" then repl () else
    try
      let lb = Lexing.from_string input in
      let expr = Parser.line Lexer.token lb in
      let anf = Anf.anf_of_expr expr in
      print_endline ("result : " ^ (string_of_anf_expr anf));
      repl ()
    with
    | Failure msg -> print_endline msg; repl () 
      (* if msg = "lexing: empty token" then repl ()
      else print_endline msg; repl () *)
;;

repl () ;;
