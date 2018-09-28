open Map

module StringMap = Map.Make (String)

(* ast *)

type func
  = {
    args: string list;
    body: stmt list;
  }

and stmt
  = Assign of string * expr
  | Return of expr

and expr
  = Name of string
  | Call of string * expr list
  | Add of expr * expr
  | Mul of expr * expr

(* ssa *)

type block = {
  highest: int ref;
  mutable varmap: int StringMap.t;
  mutable lines: line list;
}

and line
  = AddAssign of int * int * int
  | MulAssign of int * int * int
  | CallAssign of int * string * int list
  | Reassign of int * int
  | Ret of int

let into_ssa (f:func): block =
  let b = {
    highest = ref 0;
    varmap = StringMap.empty;
    lines = [];
  }
  in let next (): int =
    incr b.highest; !(b.highest)
  in let lookup (n:string): int = begin
    match StringMap.find_opt n b.varmap with
    | Some i -> i
    | None -> (let h = next () in (b.varmap <- StringMap.add n h b.varmap); h)
  end
  in let _ = List.iter (fun x -> lookup x; ()) f.args
  in let rec convert_expr (e:expr): int = (
    match e with
    | Name s -> lookup s
    | Call (s, exprs) -> begin
        let t = List.map (fun f ->
          let h = next () in
          let i = convert_expr f in
          b.lines <- Reassign (h, i) :: b.lines; h
        ) exprs in
        let r = next () in
        b.lines <- CallAssign (r, s, t) :: b.lines;
        r
      end
    | Add (p, q) -> begin
        let a1 = next () in
        let b1 = next () in
        let c1 = next () in
        let e1 = Reassign (a1, convert_expr p) in
        let e2 = Reassign (b1, convert_expr q) in
        let e3 = AddAssign (c1, a1, b1) in
        (b.lines <- e3 :: e2 :: e1 :: b.lines);
        c1
      end
    | Mul (p, q) -> begin
        let a1 = next () in
        let b1 = next () in
        let c1 = next () in
        let e1 = Reassign (a1, convert_expr p) in
        let e2 = Reassign (b1, convert_expr q) in
        let e3 = MulAssign (c1, a1, b1) in
        (b.lines <- e3 :: e2 :: e1 :: b.lines);
        c1
      end
  )
  in let rec convert_stmt (s:stmt): line = begin
    match s with
    | Assign (n, e) -> Reassign (lookup n, (convert_expr e))
    | Return e -> Ret (convert_expr e)
  end
  in List.iter (fun s -> let p = convert_stmt s in b.lines <- p :: b.lines; ()) f.body;
  b.lines = List.rev b.lines;
  b

let dist = {
  args = ["x"; "y"];
  body = [
    Assign ("x", Mul (Name "x", Name "x"));
    Assign ("y", Mul (Name "y", Name "y"));
    Return (Call ("isqrt", [Add (Name "x", Name "y")]))
  ];
}

let res = into_ssa dist
