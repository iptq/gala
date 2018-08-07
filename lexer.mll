{
  open Parser
  exception SyntaxError of string
}

rule token = parse
  | [' ' '\r' '\t'] { token lexbuf }
  | ['\n'] { NEWLINE }

  (* symbols *)
  | '(' ')' { SYM_UNIT }
  | '"' { str (Buffer.create 40) lexbuf }

  | ':' { SYM_COLON }
  | '=' { SYM_EQUALS }
  | '(' { SYM_LPAREN }
  | ')' { SYM_RPAREN }
  | ';' { SYM_SEMI }

  (* keywords *)
  | "fn" { KW_FN }
  | "int" { KW_INT }
  | "let" { KW_LET }
  | "match" { KW_MATCH }
  | "print" { KW_PRINT }
  | "return" { KW_RETURN }
  | "struct" { KW_STRUCT }
  | "type" { KW_TYPE }

  | '-'? ['0'-'9']+ as n { NUMBER(int_of_string n) }
  | ['A'-'Z' 'a'-'z' '_'] ['A'-'Z' 'a'-'z' '0'-'9' '_']* as s { IDENT(s) }

  | _ { raise (SyntaxError ("Unexpected char: " ^ Lexing.lexeme lexbuf)) }
  | eof { EOF }

and str buf = parse
  | '\\' '"' { Buffer.add_char buf '"'; str buf lexbuf }
  | '"' { STRING(Buffer.contents buf) }
  | [^ '"' '\\']+ { Buffer.add_string buf (Lexing.lexeme lexbuf); str buf lexbuf }
  | _ { raise (SyntaxError ("Illegal string character: " ^ Lexing.lexeme lexbuf)) }
  | eof { raise (SyntaxError "Unterminated string literal.") }

