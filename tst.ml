(* Typed Syntax Tree *)

open Common
open Sexplib
open Sexplib.Std

type node =
  | Value of value
  | Expr of expr
  | Stmt of stmt
[@@deriving sexp]

and expr =
  | Const of Common.const * ty
  | Var of string * ty
[@@deriving sexp]

and value =
  | ConstV of Common.const * ty
[@@deriving sexp]

and stmt =
  | Let of string * expr * ty
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

let string_of_expr (e: expr) = e |> sexp_of_expr |> Sexp.to_string_hum ~indent:4
