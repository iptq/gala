open Lexer
open Lexing
open Map
open Parse

exception RuntimeError of string

module StringMap = Map.Make(String)
type environment = Ast.node StringMap.t

type state = environment list

let state_top_set state key value =
  match state with
  | [] -> raise (RuntimeError "wtf ran out of scopes?")
  | env :: rest -> (StringMap.add key value env) :: rest

let rec state_top_get state key =
  match state with
  | [] -> None
  | env :: rest -> match StringMap.find_opt key env with Some thing -> Some thing | None -> state_top_get rest key

let print_value value =
  match value with
  | Ast.ConstV (String s) -> print_endline s
  | Ast.ConstV (Int i) -> print_endline (string_of_int i)

let rec load_in (env:environment) (decls:Ast.decl list) =
  match decls with
  | [] -> env
  | decl :: rest ->
      let name = (match decl with Ast.ItemDecl (name, _) -> name | Ast.FnDecl { name; _ } -> name) in
      load_in (StringMap.add name (Ast.Decl decl) env) rest

let rec eval_expr (state:state) (expr:Ast.expr) =
  match expr with
  | SideEffect (stmt, expr) ->
      let state = eval_single state stmt in eval_expr state expr
  | Const const ->
      (state, Ast.ConstV const)
  | Var name ->
      begin match state_top_get state name with 
        | Some (Ast.Value v) -> (state, v)
        | Some _ -> raise (RuntimeError (Printf.sprintf "Name '%s' is not a value." name))
        | None -> raise (RuntimeError (Printf.sprintf "Unbounded name '%s'." name))
      end
  | Call (func, args) ->
      let state, closure = match eval_expr state func with
        | (state, Ast.Closure closure) -> (state, closure)
        | _ -> raise (RuntimeError "waht the fuc u doin bro")
      in (state, Ast.ConstV (Common.Unit))
  | If (cond, cont, alt) ->
      let state, cond = eval_expr state cond in
      begin match cond with
        | Ast.ConstV Common.True -> eval_expr state cont
        | Ast.ConstV Common.False -> eval_expr state alt
        | _ -> raise (RuntimeError "Condition is not a boolean")
      end
  | BinOp (left, op, right) ->
      let state, left = eval_expr state left in
      let state, right = eval_expr state right in
      match op with
      | Add -> begin match state_top_get state "int_add" with
            | Some(Ast.Value(Ast.Intrinsic fn)) -> (state, fn [left; right])
            | _ -> raise (RuntimeError "what")
          end

and eval_single (state:state) (stmt:Ast.stmt): state =
  match stmt with
  | Let (name, expr) ->
      let state, value = eval_expr state expr in
      state_top_set state name (Ast.Value value)
  | Print expr -> let (_, value) = eval_expr state expr in print_value value;
      state
  | Return expr -> print_endline "return"; state

let eval_in (state:state) (func:Ast.func): state =
  let all_stmts = func.body in
  let rec eval_rec (state:state) (stmts:Ast.stmt list): state =
    match stmts with
    | [] -> state
    | stmt :: rest -> eval_rec (eval_single state stmt) rest
  in eval_rec state all_stmts
