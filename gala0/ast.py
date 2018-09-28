__all__ = [
    "Function",
    "Program",
]

class ASTMeta(type):
    rules = []
    def __new__(self, name, bases, attrs):
        c = type.__new__(self, name, bases, attrs)
        ASTMeta.rules.append(attrs.get("__doc__", ""))
        return c

class Convert(object):
    def convert(self, *args, **kwargs):
        raise NotImplementedError("convert not implemented for {}".format(self.__class__.__name__))

class ASTNode(Convert, metaclass=ASTMeta):
    def __repr__(self):
        return "{}{}".format(self.__class__.__name__, repr(vars(self)))

# ========================================================

class Body(ASTNode):
    """
    body: _NL* _INDENT (stmt _NL*)+ _DEDENT
    """
    def __init__(self, tree):
        self.children = []
        for child in tree.children:
            self.children.append(Stmt(child).into())

    def into(self):
        return self.children

class Decl(ASTNode):
    """ decl: use_decl | fn_decl | type_decl """

    def __init__(self, tree):
        assert tree.data == "decl"
        assert len(tree.children) == 1
        self.child = tree.children[0]

    def into(self):
        if self.child.data == "fn_decl":
            return Function(self.child)

class Expr(ASTNode):
    """
    expr: path
        | literal
        | expr op expr
        | expr "[" expr "]"
        | expr "(" fn_call_args ")"
        | expr "." expr
    """
    def __init__(self, tree):
        print(tree)

class Function(ASTNode):
    """
    fn_decl: "fn" name "(" fn_args ")" type_annot? "=" body
    """
    def __init__(self, tree):
        assert tree.data == "fn_decl"
        for child in tree.children:
            if child.data == "name":
                self.name = child.children[0].value
            if child.data == "body":
                self.body = Body(child).into()
        # print("\n\n".join([str(child) for child in tree.children]))

    def convert(self, globals):
        print(globals)
        return """
        define i32 @{name} () {{
            ret i32 5
        }}
        """.format(name=self.name)

class Program(ASTNode):
    """ program: (decl _NL*)+ """

    def __init__(self, tree):
        assert tree.data == "program"
        self.decls = []
        for child in tree.children:
            self.decls.append(Decl(child).into())

    def convert(self):
        globals = []
        ir = ""
        for decl in self.decls:
            ir += decl.convert(globals=globals) + "\n"
        return ir

class ReturnStmt(ASTNode):
    """
    return_stmt: "return" expr
    """
    def __init__(self, tree):
        assert len(tree.children) == 1
        self.ret = Expr(tree.children[0])

    def convert(self):
        return "ret"

class Stmt(ASTNode):
    """
    stmt: expr | assign_stmt | if_stmt | else_stmt | return_stmt
    """
    def __init__(self, tree):
        assert tree.data == "stmt"
        assert len(tree.children) == 1
        self.child = tree.children[0]

    def into(self):
        if self.child.data == "return_stmt":
            return ReturnStmt(self.child)
