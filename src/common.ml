open Core

type fn_type = {
  args: field list;
  returns: field;
}
[@@deriving sexp]

and field = {
  name: string;
  ty: ty;
}
[@@deriving sexp]

and ty =
  | FnT of fn_type
[@@deriving sexp]

type const =
  | Int of int
[@@deriving sexp]
