"""
The Gala parser.
"""

import os

from lark import Lark
from lark.lexer import Lexer, Token
from lark.indenter import Indenter

from gala0.ast import ASTMeta

class _Indenter(Indenter):
    NL_type = '_NL'
    OPEN_PAREN_types = []
    CLOSE_PAREN_types = []
    INDENT_type = '_INDENT'
    DEDENT_type = '_DEDENT'
    tab_len = 4

with open(os.path.join(os.path.dirname(__file__), "Grammar"), "r") as f:
    base_grammar = f.read()
    grammar = base_grammar + "\n" + "\n".join(ASTMeta.rules)
    # print(grammar)
    parser = Lark(grammar, parser="lalr", postlex=_Indenter())
