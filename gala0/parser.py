"""
The Gala parser.
"""

import os

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
with open(os.path.join(os.path.dirname(__file__), "Grammar"), "r") as f:
    grammar = f.read()
    parser = Lark(grammar, parser="lalr", postlex=_Indenter())
