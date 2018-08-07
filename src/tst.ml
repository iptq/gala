(* Typed Syntax Tree *)

open Core

type expr =
  | Const of Common.const
  | Var of string
[@@deriving sexp]

and stmt =
  | Let of string * expr
[@@deriving sexp]

and fn_decl = {
  name: string;
  body: stmt list;
  return: expr;
}
[@@deriving sexp]

type decl =
  | StructDecl
  | FnDecl of fn_decl
[@@deriving sexp]

type prog = decl list
[@@deriving sexp]

let string_of_expr (e: expr) = e |> sexp_of_expr |> Sexp.to_string_hum ~indent:4
let string_of_prog (p: prog) = p |> sexp_of_prog |> Sexp.to_string_hum ~indent:4
