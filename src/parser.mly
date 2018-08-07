%{
  open Ast
  open Common
%}

%token EOF NEWLINE
%token KW_FN KW_LET KW_RETURN
%token SYM_COLON SYM_EQUALS SYM_LPAREN SYM_RPAREN SYM_SEMI SYM_UNIT

%token<int> NUMBER
%token<string> IDENT

%start prog

%type<expr> expr return_stmt
%type<const> const

%type<Ast.decl> decl decl_wrap
%type<Ast.stmt> stmt stmt_wrap
%type<Ast.prog> prog

%%

(* start *)

prog: decl_wrap+ EOF { $1 }
decl_wrap: decl NEWLINE* { $1 }
decl:
  | KW_FN name=IDENT SYM_EQUALS NEWLINE* stmts=stmt_wrap* return=return_stmt {
      FnDecl { name = name; body = stmts; return = return }
    }

(* exprs *)

expr: const { Const($1) }
const:
  | SYM_UNIT { Unit }
  | NUMBER { Int($1) }

(* stmts *)

stmt_wrap: stmt linesep { $1 }
stmt: let_stmt { $1 }
let_stmt: KW_LET name=IDENT SYM_EQUALS { Let(name, Const(Unit)) }
return_stmt: KW_RETURN expr=expr? { match expr with None -> Const(Unit) | Some v -> v }

(* util *)

linesep: NEWLINE | SYM_SEMI { }
