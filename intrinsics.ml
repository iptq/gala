open Eval

let int_arithmetic fn args =
  assert (List.length args = 2);
  let args = Array.of_list args in
  match args.(0), args.(1) with
    | ((Ast.ConstV (Common.Int i)), (Ast.ConstV (Common.Int j))) -> Ast.ConstV (Common.Int (i + j))
    | _ -> raise (RuntimeError "Calling (+) on non-ints.")

let generate_intrinsics () =
  let intrinsics = StringMap.empty in

  (* TODO: actually turn these into intrinsic traits *)
  StringMap.add "int_add" (Ast.Value (Ast.Intrinsic (int_arithmetic (+)))) intrinsics
