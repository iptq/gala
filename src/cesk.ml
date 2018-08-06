open Ast
open Anf

module StringMap = Map.Make(String)
type environment = string StringMap.t
