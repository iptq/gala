%{
  open Ast
  open Common
%}

%token EOF NEWLINE
%token KW_FN KW_INT KW_LET KW_MATCH KW_PRINT KW_RETURN KW_STRUCT KW_TYPE
%token SYM_COLON SYM_EQUALS SYM_LPAREN SYM_RPAREN SYM_SEMI SYM_UNIT

%token<int> NUMBER
%token<string> IDENT STRING

%start prog

%type<expr> expr return_stmt
%type<const> const
%type<ty> fn_type_hint type_literal

%type<Ast.func> func
%type<Ast.item> item
%type<Ast.decl> decl decl_wrap
%type<Ast.stmt> stmt stmt_wrap let_stmt print_stmt
%type<Ast.prog> prog

%%

(* start *)

prog: decl_wrap+ EOF { $1 }
decl_wrap: decl NEWLINE* { $1 }
decl:
  | item_decl { $1 }
  | func_decl { $1 }
item_decl: KW_TYPE name=IDENT SYM_EQUALS item=item { ItemDecl(name, item) }
item:
  | KW_STRUCT { StructItem }
func_decl: func=func { FnDecl(func) }
func:
  | KW_FN name=IDENT SYM_LPAREN SYM_RPAREN type_hint=fn_type_hint? SYM_EQUALS NEWLINE* stmts=stmt_wrap* return=return_stmt {
      { name = name; type_hint = type_hint; body = stmts; return = return }
    }
fn_type_hint: SYM_COLON type_literal { $2 }
type_literal:
  | KW_INT { IntT }

(* exprs *)

expr:
  | const { Const($1) }
  | IDENT { Var($1) }
const:
  | SYM_UNIT { Unit }
  | NUMBER { Int($1) }
  | STRING { String($1) }

(* stmts *)

stmt_wrap: stmt linesep { $1 }
stmt:
  | let_stmt { $1 }
  | print_stmt { $1 }
  | return_stmt { Return($1) }
let_stmt: KW_LET name=IDENT SYM_EQUALS expr=expr { Let(name, expr) }
print_stmt: KW_PRINT expr=expr { Print(expr) }
return_stmt: KW_RETURN expr=expr? { match expr with None -> Const(Unit) | Some v -> v }

(* util *)

linesep: NEWLINE | SYM_SEMI { }

