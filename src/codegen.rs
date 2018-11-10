use ast;
use std::collections::BTreeMap;

fn letter_of_number(n: i32) -> String {
    let mut result = String::new();
    let mut n = n;
    while n > 0 {
        result.push(((n % 26) as u8 + 97) as char);
        n = n / 26;
    }
    result.chars().rev().collect::<String>()
}

#[derive(Debug)]
pub enum Item {
    Line(String),
    Inner(Box<Scope>),
}

impl Item {
    pub fn as_string(&self) -> String {
        match self {
            Item::Line(s) => s.clone(),
            Item::Inner(scope) => scope.as_string(),
        }
    }
}

impl Into<String> for Item {
    fn into(self) -> String {
        self.as_string()
    }
}

#[derive(Debug, Default)]
pub struct Scope {
    names: BTreeMap<String, i32>,
    items: Vec<Item>,
}

impl Scope {
    pub fn push_line(&mut self, line: impl AsRef<str>) {
        self.items.push(Item::Line(line.as_ref().to_owned()));
    }
    pub fn new_variable(&mut self, name: impl AsRef<str>, id: i32) {
        self.names.insert(name.as_ref().to_owned(), id);
    }
    pub fn lookup_name(&self, name: impl AsRef<str>) -> Option<i32> {
        self.names.get(name.as_ref()).map(|v| v.clone())
    }
    pub fn lines(self) -> Vec<Item> {
        self.items
    }
    pub fn as_string(&self) -> String {
        let mut s = String::new();
        for item in self.items.iter() {
            s += &item.as_string();
            s += "\n";
        }
        s
    }
}

#[derive(Debug)]
pub struct Emitter {
    inc: i32,
    scope_stack: Vec<Scope>,
}

impl Emitter {
    pub fn new() -> Self {
        Emitter {
            inc: 1,
            scope_stack: vec![Scope::default()],
        }
    }
    pub fn scope(&mut self) {
        self.scope_stack.push(Scope::default());
    }
    pub fn pop(&mut self) {
        if let Some(scope) = self.scope_stack.pop() {
            self.push_lines(scope.lines());
        }
    }
    pub fn next_int(&mut self) -> i32 {
        let r = self.inc;
        self.inc += 1;
        r
    }
    pub fn new_variable(&mut self, name: impl AsRef<str>, id: i32) {
        self.scope_stack
            .last_mut()
            .map(|scope| scope.new_variable(name, id));
    }
    pub fn lookup_name(&self, name: impl AsRef<str>) -> Option<i32> {
        for v in self.scope_stack.iter().rev() {
            if let Some(n) = v.lookup_name(&name) {
                return Some(n);
            }
        }
        None
    }
    pub fn push_lines(&mut self, lines: Vec<impl Into<String>>) {
        for line in lines {
            self.push_line(line.into());
        }
    }
    pub fn push_line(&mut self, line: impl AsRef<str>) {
        self.scope_stack
            .last_mut()
            .map(|scope| scope.push_line(line));
    }
    pub fn as_string(&self) -> String {
        self.scope_stack[0].as_string()
    }
}

pub trait Codegen<T = ()> {
    fn generate(&self, &mut Emitter) -> T;
}

impl Codegen for ast::Program {
    fn generate(&self, emitter: &mut Emitter) {
        for decl in self.0.iter() {
            decl.generate(emitter);
        }
    }
}

impl Codegen for ast::TopDecl {
    fn generate(&self, emitter: &mut Emitter) {
        use ast::TopDecl;
        match self {
            TopDecl::Fn(name, stmts) => {
                emitter.push_line(format!("define i32 @{}() {{", name));
                emitter.push_line("entry:");
                emitter.scope();
                for stmt in stmts {
                    stmt.generate(emitter);
                }
                emitter.pop();
                emitter.push_line("ret i32 0");
                emitter.push_line("}");
            }
        }
    }
}

impl Codegen for ast::Stmt {
    fn generate(&self, emitter: &mut Emitter) {
        use ast::Stmt;
        match self {
            Stmt::Assign(name, expr) => {
                let assigned = expr.generate(emitter);
                let result = emitter.next_int();
                emitter.new_variable(name, result);
                emitter.push_line(format!("%i{} = add i32 %i{}, 0", result, assigned));
            }
            Stmt::Expr(expr) => {
                expr.generate(emitter);
            }
            Stmt::If(cond, body1) => {
                let cond = cond.generate(emitter);
                let cmp = emitter.next_int();
                let succ_label = emitter.next_int();
                let done_label = emitter.next_int();
                emitter.push_line(format!("%i{} = icmp ne i32 %i{}, 0", cmp, cond));
                emitter.push_line(format!(
                    "br i1 %i{}, label %L{}, label %L{}",
                    cmp,
                    letter_of_number(succ_label),
                    letter_of_number(done_label)
                ));
                emitter.push_line(format!("L{}:", letter_of_number(succ_label)));
                emitter.scope();
                for stmt in body1 {
                    stmt.generate(emitter);
                }
                emitter.pop();
                emitter.push_line(format!("br label %L{}", letter_of_number(done_label)));
                emitter.push_line(format!("L{}:", letter_of_number(done_label)));
            }
        }
    }
}

impl Codegen<i32> for ast::Expr {
    fn generate(&self, emitter: &mut Emitter) -> i32 {
        use ast::Expr;
        match self {
            Expr::Literal(lit) => lit.generate(emitter),
            Expr::Name(name) => match emitter.lookup_name(name) {
                Some(val) => val,
                None => panic!("Could not find name '{}'", name),
            },
            Expr::Plus(left, right) | Expr::Minus(left, right) => {
                let left = left.generate(emitter);
                let right = right.generate(emitter);
                let result = emitter.next_int();
                emitter.push_line(format!(
                    "%i{} = {} i32 %i{}, %i{}",
                    result,
                    match self {
                        Expr::Plus(_, _) => "add",
                        Expr::Minus(_, _) => "sub",
                        _ => unreachable!(),
                    },
                    left,
                    right
                ));
                result
            }
        }
    }
}

impl Codegen<i32> for ast::Literal {
    fn generate(&self, emitter: &mut Emitter) -> i32 {
        use ast::Literal;
        match self {
            Literal::Int(n) => {
                let result = emitter.next_int();
                // wtf?
                emitter.push_line(format!("%i{} = add i32 {}, 0", result, n));
                result
            }
        }
    }
}
