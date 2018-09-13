"""
The Gala parser.
"""

from lark import Lark
from lark.lexer import Lexer, Token
from lark.indenter import Indenter

class _Indenter(Indenter):
    NL_type = '_NL'
    OPEN_PAREN_types = []
    CLOSE_PAREN_types = []
    INDENT_type = '_INDENT'
    DEDENT_type = '_DEDENT'
    tab_len = 4

# == Main Grammar ==
parser = Lark(r"""
    %import common.WS_INLINE
    %ignore WS_INLINE

    ?start: _NL* program

    %declare _INDENT _DEDENT
    _NL: /(\n[\t ]*)+/

    name: /[A-Za-z_][A-Za-z0-9_]*/
    path: name ("::" name)*
    number: /[0-9]+(\.[0-9]*)?/
    string: /\"([^"\\]|\\n)*\"/
    unit: /\(\)/

    program: (decl _NL*)+
    decl: use_decl | fn_decl | type_decl

    use_decl: "use" name
    type_decl: "type" name decl_type_annot "=" type_decl_body
    type_decl_body: _NL [_INDENT type_decl_line+ _DEDENT]
    type_decl_line: name ":" type_literal _NL*
                  | "fn" name "(" fn_args ")" type_annot _NL*

    type_annot: ":" type_literal
    type_literal: "int"
                | name
                | unit
                | param_type
    param_type: type_literal type_literal

    decl_type_annot: ":" decl_type_literal
    decl_type_literal: "struct" | "enum" | "trait"

    fn_decl: "fn" name "(" fn_args ")" type_annot "=" body
    fn_args: (name type_annot ("," name type_annot)* ","?)?
    body: _NL* _INDENT (stmt _NL*)+ _DEDENT

    stmt: expr | if_stmt | else_stmt | return_stmt
    if_stmt: "if" expr body
    else_stmt: "else" body
             | "else" if_stmt

    return_stmt: "return" expr
    expr: path
        | literal
        | expr op expr
        | expr "(" fn_call_args ")"
        | expr "." expr
    fn_call_args: (expr ("," expr)* ","?)?

    literal: number
           | string

    op: "<=" | "*" | "-"
""", parser="lalr", postlex=_Indenter())
