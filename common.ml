open Sexplib.Std

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
  | BoolT
  | IntT
  | StringT
  | FnT of fn_type
[@@deriving sexp]

type const =
  | Unit
  | True
  | False
  | Int of int
  | String of string
[@@deriving sexp]
