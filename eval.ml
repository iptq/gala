open Lexer
open Lexing
open Map
open Parse

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
  let all_stmts = func.body in
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

