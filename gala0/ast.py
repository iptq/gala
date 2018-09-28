import gala0.anf

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

class BinOp(ASTNode):
    """
    binop: expr op expr
    """
    def __init__(self, tree):
        assert tree.data == "binop"
        assert len(tree.children) == 3
        self.left = Expr(tree.children[0]).into()
        self.op = Op(tree.children[1])
        self.right = Expr(tree.children[2]).into()

    def convert(self, locals, globals):
        return "{} {} {}".format(
            self.left.convert(locals, globals),
            self.op.convert(locals, globals),
            self.right.convert(locals, globals),
        )

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
    expr: binop
        | path
        | literal
        | expr "[" expr "]"
        | expr "(" fn_call_args ")"
        | expr "." expr
    """
    def __init__(self, tree):
        self.tree = tree

    def into(self):
        if len(self.tree.children) == 1:
            child = self.tree.children[0]
            if child.data == "binop":
                return BinOp(child)
            elif child.data == "literal":
                return Literal(child).into()

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

    def convert(self, globals):
        locals = []
        stmts = []
        for stmt in self.body:
            stmts.append(stmt.convert(locals=locals, globals=globals))
        return """
        define i32 @{name} () {{
            {stmts}
        }}
        """.format(name=self.name, stmts="\n".join(stmts))

class Literal(ASTNode):
    """
    literal: number
        | string
    """
    def __init__(self, tree):
        assert tree.data == "literal"
        assert len(tree.children) == 1
        self.child = tree.children[0]

    def into(self):
        if self.child.data == "number":
            return Number(self.child)
        else:
            return String(self.child)

class Number(ASTNode):
    """
    number: /[1-9][0-9]*/
    """
    def __init__(self, tree):
        assert tree.data == "number"
        assert len(tree.children) == 1
        self.value = tree.children[0]

    def convert(self, locals, globals):
        return str(self.value)

class Op(ASTNode):
    """
    op: /==/ | /\<=/ | /\*/ | /-/ | /\+/
    """
    def __init__(self, tree):
        assert tree.data == "op"
        assert len(tree.children) == 1
        self.op = tree.children[0]

    def convert(self, locals, globals):
        return self.op

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
        self.expr = Expr(tree.children[0]).into()

    def convert(self, locals, globals):
        return "ret i32 {}".format(self.expr.convert(locals, globals))

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
