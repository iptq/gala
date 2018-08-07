%{
  open Ast
  open Common
%}

%token EOF NEWLINE
%token KW_FN KW_RETURN
%token SYM_COLON SYM_EQUALS SYM_LPAREN SYM_RPAREN SYM_SEMI SYM_UNIT

%token<int> NUMBER
%token<string> IDENT

%start prog

%type<expr> expr return_stmt
%type<const> const

%type<Ast.decl> decl
%type<Ast.prog> prog

%%

(* start *)

prog: decl+ EOF { $1 }
decl:
  | KW_FN IDENT SYM_EQUALS NEWLINE* return=return_stmt NEWLINE* {
      FnDecl { name = $2; body = []; return = return }
    }

(* exprs *)

expr: const { Const($1) }
const:
  | SYM_UNIT { Unit }
  | NUMBER { Int($1) }

(* stmts *)

stmt: return_stmt linesep { }
return_stmt: KW_RETURN expr=expr? { match expr with None -> Const(Unit) | Some v -> v }

(* util *)

linesep: NEWLINE | SYM_SEMI { }
