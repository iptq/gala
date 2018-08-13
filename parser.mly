%{
  open Ast
  open Common
%}

%token EOF NEWLINE
%token KW_ELSE KW_FALSE KW_FN KW_IF KW_INT KW_LET KW_MATCH KW_PRINT KW_RETURN KW_STRUCT KW_THEN KW_TRUE KW_TYPE
%token SYM_COLON SYM_COMMA SYM_DASH SYM_EQUALS SYM_LPAREN SYM_PLUS SYM_RPAREN SYM_SEMI SYM_SLASH SYM_STAR SYM_UNIT

%token<int> NUMBER
%token<string> IDENT STRING

%start prog line exprline

%type<const> const
%type<ty> fn_type_hint type_literal

%type<Ast.func_args> func_args
%type<Ast.func> func
%type<Ast.expr> exprline expr return_stmt
%type<Ast.item> item
%type<Ast.decl> decl decl_wrap
%type<Ast.stmt list> block
%type<Ast.stmt> line stmt stmt_wrap let_stmt print_stmt
%type<Ast.prog> prog
%type<Ast.op> op

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
func_arg: name=IDENT ty=fn_type_hint? { { name = name; ty = ty } }
func_args:
  | SYM_UNIT { [] }
  | SYM_LPAREN args=func_arg+ SYM_RPAREN { args }
func_call:
  | SYM_UNIT { [] }
  | SYM_LPAREN SYM_RPAREN { [] }
  | SYM_LPAREN first=expr rest=func_call_rest SYM_COMMA? SYM_RPAREN { first :: rest }
func_call_rest:
  | { [] }
  | SYM_COMMA expr func_call_rest { $2 :: $3 }
func:
  | KW_FN name=IDENT args=func_args type_hint=fn_type_hint? SYM_EQUALS NEWLINE* stmts=block {
      { name = name; args = args; type_hint = type_hint; body = stmts; }
    }
fn_type_hint: SYM_COLON type_literal { $2 }
type_literal:
  | KW_INT { IntT }

(* exprs *)

exprline: expr EOF { $1 }
expr:
  | SYM_LPAREN expr SYM_RPAREN { $2 }
  | stmt=stmt SYM_SEMI expr=expr { SideEffect(stmt, expr) }
  | const { Const($1) }
  | IDENT { Var($1) }
  | expr func_call { Call($1, $2) }
  | KW_IF cond=expr KW_THEN cont=expr KW_ELSE alt=expr { If(cond, cont, alt) }
  | expr op expr { BinOp($1, $2, $3) }
const:
  | SYM_UNIT { Unit }
  | KW_TRUE { True }
  | KW_FALSE { False }
  | NUMBER { Int($1) }
  | STRING { String($1) }
op:
  | SYM_PLUS { Add }
  | SYM_DASH { Sub }
  | SYM_STAR { Mul }
  | SYM_SLASH { Div }

(* stmts *)

line: stmt EOF { $1 }
block: stmt_wrap+ { $1 }
stmt_wrap: stmt linesep { $1 }
stmt:
  | let_stmt { $1 }
  | print_stmt { $1 }
  | return_stmt { Return($1) }
let_stmt: KW_LET name=IDENT SYM_EQUALS expr=expr { Let(name, expr) }
print_stmt: KW_PRINT expr=expr { Print(expr) }
return_stmt: KW_RETURN expr=expr? { match expr with Some v -> v | None -> Const(Unit) }

(* util *)

linesep: NEWLINE | SYM_SEMI { }

