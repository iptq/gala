%{
  open Ast
%}

%token EOL

%token<int> NUMBER

%start line expr
%type<Ast.expr> line expr

%%

line:
  | expr EOL? { $1 }
;

expr:
  | const { Const($1) }
;

const:
  | NUMBER { Int($1) }
;
