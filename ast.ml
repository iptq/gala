(* Abstract Syntax Tree *)

open Sexplib
open Sexplib.Std

type node =
  | Value of value
  | Expr of expr
  | Decl of decl
  | Stmt of stmt
[@@deriving sexp]

and expr =
  | SideEffect of stmt * expr
  | Const of Common.const
  | Var of string
  | Call of expr * expr list
  | If of expr * expr * expr
  | BinOp of expr * op * expr
[@@deriving sexp]

and value =
  | Intrinsic of (value list -> value)
  | Closure of stmt list (* TODO: actually use a closure *)
  | ConstV of Common.const
[@@deriving sexp]

and op =
  | Add
  | Sub
  | Mul
  | Div
[@@deriving sexp]

and stmt =
  | ExprS of expr
  | Let of string * expr
  (* TODO: move print to intrinsics *)
  | Print of expr
  | Return of expr
[@@deriving sexp]

and field = {
  name: string;
  ty: Common.ty option;
}
[@@deriving sexp]

and func_args = field list
[@@deriving sexp]

and func = {
  name: string;
  args: func_args;
  type_hint: Common.ty option;
  body: stmt list;
}
[@@deriving sexp]

and item =
  | StructItem
[@@deriving sexp]

and decl =
  | ItemDecl of string * item
  | FnDecl of func
[@@deriving sexp]

type prog = decl list
[@@deriving sexp]

let string_of_expr (e: expr) = e |> sexp_of_expr |> Sexp.to_string_hum ~indent:4
let string_of_prog (p: prog) = p |> sexp_of_prog |> Sexp.to_string_hum ~indent:4
