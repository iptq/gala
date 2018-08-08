open Common

module StringMap = Map.Make(String)
type environment = ty StringMap.t

let annotate_const env const =
  match const with
  | Unit -> (const, UnitT)
  | Int _ -> (const, IntT)
  | String _ -> (const, StringT)

let annotate_value env astvalue =
  match astvalue with
  | Ast.ConstV const -> let (c, t) = annotate_const env const in Tst.ConstV (c, t)

let annotate env astnode =
  match astnode with
  | Ast.Value value -> Tst.Value (annotate_value env value)

let infer env astnode =
  annotate env astnode
