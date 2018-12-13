use std::collections::BTreeMap;

use common::{self, next_int, Typed};
use mir;

fn letter_of_number(mut n: u32) -> String {
    let mut result = String::new();
    while n > 0 {
        result.push(((n % 26) as u8 + 97) as char);
        n /= 26;
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
    names: BTreeMap<String, u32>,
    items: Vec<Item>,
}

impl Scope {
    pub fn push_subscope(&mut self, scope: Scope) {
        self.items.push(Item::Inner(Box::new(scope)));
    }
    pub fn prepend_line(&mut self, line: impl AsRef<str>) {
        self.items.insert(0, Item::Line(line.as_ref().to_owned()));
    }
    pub fn push_line(&mut self, line: impl AsRef<str>) {
        self.items.push(Item::Line(line.as_ref().to_owned()));
    }
    pub fn new_variable(&mut self, name: impl AsRef<str>, id: u32) {
        self.names.insert(name.as_ref().to_owned(), id);
    }
    pub fn lookup_name(&self, name: impl AsRef<str>) -> Option<u32> {
        self.names.get(name.as_ref()).cloned()
    }
    pub fn as_string(&self) -> String {
        self.items
            .iter()
            .map(|item| item.as_string())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[derive(Debug)]
pub struct Emitter {
    scope_stack: Vec<Scope>,
}

impl Emitter {
    pub fn new() -> Self {
        Emitter {
            scope_stack: vec![Scope::default()],
        }
    }
    pub fn scope(&mut self) {
        self.scope_stack.push(Scope::default());
    }
    pub fn pop(&mut self) {
        if let Some(scope) = self.scope_stack.pop() {
            if let Some(scope2) = self.scope_stack.last_mut() {
                scope2.push_subscope(scope)
            }
        }
    }
    pub fn next_int(&mut self) -> u32 {
        next_int()
    }
    pub fn new_variable(&mut self, name: impl AsRef<str>, id: u32) {
        if let Some(scope) = self.scope_stack.last_mut() {
            scope.new_variable(name, id)
        }
    }
    pub fn lookup_name(&self, name: impl AsRef<str>) -> Option<u32> {
        for v in self.scope_stack.iter().rev() {
            if let Some(n) = v.lookup_name(&name) {
                return Some(n);
            }
        }
        None
    }
    pub fn push_global_line(&mut self, line: impl AsRef<str>) {
        if let Some(scope) = self.scope_stack.first_mut() {
            scope.prepend_line(line)
        }
    }
    pub fn push_line(&mut self, line: impl AsRef<str>) {
        if let Some(scope) = self.scope_stack.last_mut() {
            scope.push_line(line)
        }
    }
    pub fn as_string(&self) -> String {
        self.scope_stack[0].as_string()
    }
}

pub trait Codegen<T = ()> {
    fn generate(&self, &mut Emitter) -> T;
}

impl Codegen for mir::Program {
    fn generate(&self, emitter: &mut Emitter) {
        for decl in self.0.iter() {
            decl.generate(emitter);
        }
    }
}

impl Codegen for mir::TopDecl {
    fn generate(&self, emitter: &mut Emitter) {
        use mir::TopDecl;
        match self {
            TopDecl::Extern(name, _ty) => {
                emitter.push_line(format!("declare i32 @{}(i8* nocapture) nounwind", name));
            }
            TopDecl::Fn(name, args, _ty, stmts) => {
                let mut args_s = Vec::new();
                let mut args_a = Vec::new();
                for arg in args {
                    let argn = emitter.next_int();
                    args_s.push(format!("{} %i{}", arg.1.ir_repr().as_ref(), argn));

                    let tmp = emitter.next_int();
                    args_a.push(format!("%i{} = alloca i32", tmp));
                    args_a.push(format!("store i32 %i{}, i32* %i{}", argn, tmp));
                    emitter.new_variable(&arg.0, tmp);
                }
                for _arg in args {}
                emitter.push_line(format!("define i32 @{} ({}) {{", name, args_s.join(", ")));
                emitter.push_line("entry:");
                emitter.scope();
                for arg in args_a {
                    emitter.push_line(arg);
                }
                for stmt in stmts {
                    stmt.generate(emitter);
                }
                emitter.pop();
                emitter.push_line("ret i32 0");
                emitter.push_line("}");
            }
            TopDecl::Struct(name, fields) => {
                let fields_s = fields
                    .iter()
                    .map(|field| field.get_type().ir_repr().as_ref().to_owned())
                    .collect::<Vec<_>>()
                    .join(", ");
                emitter.push_line(format!("%st.{} = type {{ {} }}", name, fields_s));
            }
        }
    }
}

impl Codegen for mir::Stmt {
    fn generate(&self, emitter: &mut Emitter) {
        use mir::Stmt;
        match self {
            Stmt::Assign(re, name, expr) => {
                let assigned = expr.generate(emitter);
                let result = if !*re {
                    let result = emitter.next_int();
                    emitter.new_variable(name, result);
                    emitter.push_line(format!("%i{} = alloca i32", result));
                    result
                } else {
                    emitter
                        .lookup_name(name)
                        .unwrap_or_else(|| panic!("Name '{}' not found.", name))
                };
                let tmp = emitter.next_int();
                emitter.push_line(format!("%i{} = add i32 %i{}, 0", tmp, assigned));
                emitter.push_line(format!("store i32 %i{}, i32* %i{}", tmp, result));
            }
            Stmt::Expr(expr) => {
                expr.generate(emitter);
            }
            Stmt::If(cond, body1, body2) => {
                let cond_ty = cond.get_type().ir_repr().as_ref().to_owned();
                let cond = cond.generate(emitter);
                let cmp = emitter.next_int();
                let succ_label = letter_of_number(emitter.next_int());
                let fail_label = letter_of_number(emitter.next_int());
                let done_label = letter_of_number(emitter.next_int());
                emitter.push_line(format!("%i{} = icmp ne {} %i{}, 0", cmp, cond_ty, cond));
                emitter.push_line(format!(
                    "br i1 %i{}, label %L{}, label %L{}",
                    cmp,
                    succ_label,
                    match &body2 {
                        Some(_) => fail_label.clone(),
                        None => done_label.clone(),
                    }
                ));
                emitter.push_line(format!("L{}:", succ_label));
                emitter.scope();
                for stmt in body1 {
                    stmt.generate(emitter);
                }
                emitter.pop();
                emitter.push_line(format!("br label %L{}", done_label));
                if let Some(body) = body2 {
                    emitter.push_line(format!("L{}:", fail_label));
                    emitter.scope();
                    for stmt in body {
                        stmt.generate(emitter);
                    }
                    emitter.pop();
                    emitter.push_line(format!("br label %L{}", done_label));
                }
                emitter.push_line(format!("L{}:", done_label));
            }
            Stmt::While(cond, body) => {
                let begin_label = letter_of_number(emitter.next_int());
                let check_label = letter_of_number(emitter.next_int());
                let done_label = letter_of_number(emitter.next_int());
                emitter.push_line(format!("br label %L{}", check_label));
                emitter.push_line(format!("L{}:", begin_label));
                emitter.scope();
                for stmt in body {
                    stmt.generate(emitter);
                }
                emitter.pop();

                emitter.push_line(format!("br label %L{}", check_label));
                emitter.push_line(format!("L{}:", check_label));

                let cond_ty = cond.get_type().ir_repr().as_ref().to_owned();
                let cond = cond.generate(emitter);
                let cmp = emitter.next_int();
                emitter.push_line(format!("%i{} = icmp ne {} %i{}, 0", cmp, cond_ty, cond));
                emitter.push_line(format!(
                    "br i1 %i{}, label %L{}, label %L{}",
                    cmp, begin_label, done_label
                ));
                emitter.push_line(format!("L{}:", done_label));
            }
            Stmt::Return(expr) => match expr {
                Some(expr) => {
                    let expr = expr.generate(emitter);
                    emitter.push_line(format!("ret i32 %i{}", expr));
                }
                None => {
                    emitter.push_line("ret void".to_owned());
                }
            },
        }
    }
}

impl Codegen<u32> for mir::Expr {
    fn generate(&self, emitter: &mut Emitter) -> u32 {
        use mir::Expr;
        match self {
            Expr::Call(func, args, _ty) => {
                let result = emitter.next_int();
                let args = args
                    .iter()
                    .map(|expr| {
                        let ename = expr.generate(emitter);
                        format!("{} %i{}", expr.get_type().ir_repr().as_ref(), ename)
                    }).collect::<Vec<_>>()
                    .join(", ");
                emitter.push_line(format!("%i{} = call i32 @{}({})", result, func, args));
                result
            }
            Expr::Literal(lit, _ty) => lit.generate(emitter),
            Expr::Name(name, _ty) => match emitter.lookup_name(name) {
                Some(val) => {
                    let result = emitter.next_int();
                    emitter.push_line(format!("%i{} = load i32, i32* %i{}", result, val));
                    result
                }
                None => panic!("Could not find name '{}'", name),
            },
            Expr::NotEquals(left, right, _ty)
            | Expr::Equals(left, right, _ty)
            | Expr::Plus(left, right, _ty)
            | Expr::Minus(left, right, _ty)
            | Expr::Times(left, right, _ty) => {
                let left = left.generate(emitter);
                let right = right.generate(emitter);
                let result = emitter.next_int();
                emitter.push_line(format!(
                    "%i{} = {} i32 %i{}, %i{}",
                    result,
                    match self {
                        Expr::NotEquals(_, _, _) => "icmp ne",
                        Expr::Equals(_, _, _) => "icmp eq",
                        Expr::Plus(_, _, _) => "add",
                        Expr::Minus(_, _, _) => "sub",
                        Expr::Times(_, _, _) => "mul",
                        _ => unreachable!(),
                    },
                    left,
                    right,
                ));
                result
            }
        }
    }
}

impl Codegen<u32> for common::Literal {
    fn generate(&self, emitter: &mut Emitter) -> u32 {
        use common::Literal;
        match self {
            Literal::Int(n) => {
                let result = emitter.next_int();
                // wtf?
                emitter.push_line(format!("%i{} = add i32 {}, 0", result, n));
                result
            }
            Literal::String(s) => {
                let litname = emitter.next_int();
                let tmp = emitter.next_int();
                let result = emitter.next_int();
                emitter.push_global_line(format!(
                    "@ss{} = private unnamed_addr constant [{} x i8] c\"{}\"",
                    litname,
                    s.len(),
                    s
                ));
                emitter.push_line(format!(
                    "%i{} = getelementptr [{} x i8], [{} x i8]* @ss{}",
                    tmp,
                    s.len(),
                    s.len(),
                    litname
                ));
                emitter.push_line(format!(
                    "%i{} = bitcast [{} x i8]* %i{} to i8*",
                    result,
                    s.len(),
                    tmp
                ));
                result
            }
        }
    }
}
