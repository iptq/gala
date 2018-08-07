open Lexer
open Lexing
open Map
open Parser

module StringMap = Map.Make(String)
type environment = Ast.decl StringMap.t

type state = environment list

let rec load_in (env:environment) (decls:Ast.decl list) =
  match decls with
  | [] -> env
  | decl :: rest ->
      let name = (match decl with Ast.ItemDecl (name, _) -> name | Ast.FnDecl { name; _ } -> name) in
      load_in (StringMap.add name decl env) rest

let eval_in (state:state) (func:Ast.func): state =
  let all_stmts = func.body @ [Return(func.return)] in
  let rec eval_single (state:state) (stmt:Ast.stmt): state =
    match stmt with
    | Let (name, expr) -> print_endline "let"; state
    | Print expr -> print_endline "print"; state
    | Return expr -> print_endline "return"; state
  in let rec eval_rec (state:state) (stmts:Ast.stmt list): state =
    match stmts with
    | [] -> state
    | stmt :: rest -> eval_rec (eval_single state stmt) rest
  in eval_rec state all_stmts

let _ = 
  if Array.length Sys.argv < 2 then begin
    print_endline "Usage: eval [file.g]";
    (exit 1)
  end else
    (* prepare *)
    let open MenhirLib.General in
    let module Interp = Parser.MenhirInterpreter in

    (* parse input *)
    let ic = open_in Sys.argv.(1) in
    let lexbuf = Lexing.from_channel ic in
    let prog = Parser.prog Lexer.token lexbuf in

    (* evaluate *)
    (* Ast.string_of_prog prog |> print_endline; *)
    let global = load_in StringMap.empty prog in
    let main = (match StringMap.find_opt "main" global with
      | Some (FnDecl v) -> v
      | None -> raise (Failure "No main function found.")
    ) in
    eval_in (global::[]) main;
;;
