mod expr;
mod stmt;

use std::collections::HashMap;

use crate::{
    ast::{DType, FnDecl, Stmt, StructDecl},
    utils::{ErrorType, MessageType, report_message},
};

/// Anotate ast with types and check them.
pub struct TypeSystem<'a> {
    fn_decls: &'a mut Vec<FnDecl>,
    st_decls: &'a mut Vec<StructDecl>,
    pub errors_count: usize,
    pub env: HashMap<String, DType>,
}

impl<'a> TypeSystem<'a> {
    pub fn new(fn_decls: &'a mut Vec<FnDecl>, st_decls: &'a mut Vec<StructDecl>) -> Self {
        Self {
            fn_decls,
            st_decls,
            errors_count: 0,
            env: HashMap::new(),
        }
    }

    pub fn check(&mut self) {
        let mut local_fn_decls: Vec<_> = std::mem::take(self.fn_decls);
        for i in &mut local_fn_decls {
            self.check_fn(i)
        }
        *self.fn_decls = local_fn_decls;
    }

    fn check_fn(&mut self, fn_decl: &mut FnDecl) {
        self.env.clear();
        for stmt in &mut fn_decl.body {
            if let Err(e) = self.check_stmt(stmt) {
                self.errors_count += 1;
                Self::report_msg(e.to_string());
            }
        }
    }

    pub(super) fn check_stmt(&mut self, stmt: &mut Stmt) -> Result<DType, anyhow::Error> {
        match stmt {
            Stmt::Block(block) => self.check_block(block),
            Stmt::Let(le) => self.check_let(le),
            Stmt::StructInit(st_init) => self.check_stinit(st_init),
            Stmt::If(ifst) => self.check_if(ifst),
            Stmt::Loop(lop) => self.check_loop(lop),
            Stmt::Break(bre) => self.check_break(bre),
            Stmt::Print(p) => self.check_print(p),
            Stmt::Return(ret) => self.check_return(ret),
            Stmt::Expression(expr) => self.check_expr(expr),
        }
    }

    pub(super) fn def_env(&mut self, name: String, ty: DType) -> Option<DType> {
        self.env.insert(name, ty)
    }

    pub(super) fn get_env(&self, name: &String) -> Option<&DType> {
        self.env.get(name)
    }

    fn report_msg<M: Into<String>>(msg: M) {
        report_message(msg.into(), MessageType::Error(ErrorType::TypeError))
    }

    pub fn fn_decls(&self) -> &Vec<FnDecl> {
        self.fn_decls
    }
    pub fn st_decls(&self) -> &Vec<StructDecl> {
        self.st_decls
    }
}
