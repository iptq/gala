open Core

type fn_type = {
  args: field list;
  returns: ty;
}
[@@deriving sexp]

and field = {
  name: string;
  ty: ty;
}
[@@deriving sexp]

and ty =
  | UnitT
  | FnT of fn_type
[@@deriving sexp]

type const =
  | Unit
  | Int of int
[@@deriving sexp]
