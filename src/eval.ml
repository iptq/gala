open Lexer
open Lexing
open Parser

let _ = 
  if Array.length Sys.argv < 2 then begin
    print_endline "Usage: eval [file.g]";
    (exit 1)
  end else
    (* parse input *)
    let ic = open_in Sys.argv.(1) in
    let lexbuf = Lexing.from_channel ic in
    let prog = Parser.prog Lexer.token lexbuf in
    Ast.string_of_prog prog |> print_endline
;;
