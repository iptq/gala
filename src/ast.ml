open Core

type expr =
  | Const of Common.const
[@@deriving sexp]

type fn_decl = {
  name: string;
}
[@@deriving sexp]

type decl =
  | FnDecl of fn_decl
[@@deriving sexp]

type prog = decl list
[@@deriving sexp]

let string_of_expr (e: expr) = e |> sexp_of_expr |> Sexp.to_string_hum ~indent:4
let string_of_prog (p: prog) = p |> sexp_of_prog |> Sexp.to_string_hum ~indent:4
