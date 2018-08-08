{
  open Parser

  exception Error of string * Lexing.position
  let lexing_error lexbuf =
	let invalid_input = String.make 1 (Lexing.lexeme_char lexbuf 0) in
	raise (Error (invalid_input, lexbuf.Lexing.lex_curr_p))
}

rule token = parse
  | [' ' '\r' '\t'] { token lexbuf }
  | '\\' '\n' { token lexbuf (* escape newline *) }
  | ['\n'] { Lexing.new_line lexbuf; NEWLINE }
  | '/' '/' { line_comment lexbuf }

  (* symbols *)
  | '(' ')' { SYM_UNIT }
  | '"' { str (Buffer.create 40) lexbuf }
  | ':' { SYM_COLON }
  | ',' { SYM_COMMA }
  | '-' { SYM_DASH }
  | '=' { SYM_EQUALS }
  | '(' { SYM_LPAREN }
  | '+' { SYM_PLUS }
  | ')' { SYM_RPAREN }
  | ';' { SYM_SEMI }
  | '/' { SYM_SLASH }
  | '*' { SYM_STAR }

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

  | _ { lexing_error lexbuf }
  | eof { EOF }

and str buf = parse
  | '\\' '"' { Buffer.add_char buf '"'; str buf lexbuf }
  | '"' { STRING(Buffer.contents buf) }
  | [^ '"' '\\']+ { Buffer.add_string buf (Lexing.lexeme lexbuf); str buf lexbuf }
  | _ { lexing_error lexbuf }
  | eof { lexing_error lexbuf }

and line_comment = parse
  | '\n' { token lexbuf }
  | _ { line_comment lexbuf }
