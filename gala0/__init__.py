from gala0.parser import parser

def compile(data):
    tree = parser.parse(data)
    print(tree.pretty())
