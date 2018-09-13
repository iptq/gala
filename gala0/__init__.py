from gala0.parser import parser

def compile(data):
    raw_tree = parser.parse(data)
    print(raw_tree.pretty())
