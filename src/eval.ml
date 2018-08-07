open Lexer
open Lexing
open Map
open Parser

module StringMap = Map.Make(String)
type environment = Ast.decl StringMap.t

let rec load_in (env:environment) (decls:Ast.decl list) =
  match decls with
  | [] -> env
  | decl :: rest ->
      let name = (match decl with Ast.ItemDecl (name, _) -> name | Ast.FnDecl { name; _ } -> name) in
      load_in (StringMap.add name decl env) rest

let _ = 
  if Array.length Sys.argv < 2 then begin
    print_endline "Usage: eval [file.g]";
    (exit 1)
  end else
    (* parse input *)
    let ic = open_in Sys.argv.(1) in
    let lexbuf = Lexing.from_channel ic in
    let prog = Parser.prog Lexer.token lexbuf in

    (* evaluate *)
    Ast.string_of_prog prog |> print_endline;
    let root = load_in StringMap.empty prog in
    let main = (match StringMap.find_opt "main" root with
      | Some v -> v
      | None -> raise (Failure "No main function found.")
    ) in
    ();
;;
