open Eval
open Intrinsics

let _ = 
  if Array.length Sys.argv < 2 then begin
    print_endline "Usage: eval [file.g]";
    (exit 1)
  end else
    (* parse input *)
    let lexbuf = Parse.lexbuf_of_file Sys.argv.(1) in
    match Parse.parse Parser.Incremental.prog lexbuf with
    | `Success prog -> begin
        (* evaluate *)
        Ast.string_of_prog prog |> print_endline;
        let intrinsics = generate_intrinsics () in
        let global = Eval.load_in intrinsics prog in
        let main = (match StringMap.find_opt "main" global with
          | Some (Ast.Decl (FnDecl v)) -> v
          | _ -> raise (Failure "No main function found.")
        ) in
        let _ = Eval.eval_in (global::[]) main in
        ()
      end
    | `Error (line, message) -> print_endline ("Error " ^ message ^ " on line " ^ (string_of_int line))
;;
