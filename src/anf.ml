open Core

type expr =
  | Const of Common.const
[@@deriving sexp]

let anf_of_expr (expr: Ast.expr) =
  match expr with
  | Ast.Const c -> Const c

let string_of_anf_expr (e: expr) = e |> sexp_of_expr |> Sexp.to_string_hum ~indent:4
