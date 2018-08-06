{
  open Parser
}

rule token = parse
  | [' ' '\r' '\t'] { token lexbuf }
  | ['\n'] { EOL }

  | '-'? ['0'-'9']+ as n { NUMBER(int_of_string n) }
  | eof { EOL }
