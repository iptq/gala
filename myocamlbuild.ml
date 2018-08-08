(* OASIS_START *)
(* OASIS_STOP *)

open Ocamlbuild_plugin

module Menhir = struct
  let dispatcher = function
    | After_rules ->
        flag ["ocamlbuild"] (A "-use-ocamlfind");
        flag ["menhir"; "parser"; "menhir_trace"] (A "--trace");
        flag ["menhir"; "parser"; "menhir_table"] (A "--table");
        flag ["menhir"; "parser"; "menhir_canonical"] (A "--canonical");
    | _ -> ()
end

let _ =
  dispatch begin fun hook ->
    dispatch_default hook;
    Menhir.dispatcher hook;
  end

