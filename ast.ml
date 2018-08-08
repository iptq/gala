(* Abstract Syntax Tree *)

open Sexplib
open Sexplib.Std

type expr =
  | Const of Common.const
  | Var of string
[@@deriving sexp]

and stmt =
  | Let of string * expr
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

type item =
  | StructItem
[@@deriving sexp]

type decl =
  | ItemDecl of string * item
  | FnDecl of func
[@@deriving sexp]

type prog = decl list
[@@deriving sexp]

let string_of_expr (e: expr) = e |> sexp_of_expr |> Sexp.to_string_hum ~indent:4
let string_of_prog (p: prog) = p |> sexp_of_prog |> Sexp.to_string_hum ~indent:4
