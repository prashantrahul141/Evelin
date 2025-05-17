mod expr;
mod stmt;

use std::collections::HashMap;

use log::{debug, trace};

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
        debug!("creating new type checker");
        Self {
            fn_decls,
            st_decls,
            errors_count: 0,
            env: HashMap::new(),
        }
    }

    pub fn check(mut self) -> (usize, Vec<FnDecl>) {
        debug!("running type check");
        let mut fns = vec![];
        for mut fn_decl in self.fn_decls.clone() {
            trace!("checking function : '{}'", &fn_decl.name);
            self.env.clear();

            if let Some(p) = &fn_decl.parameter {
                self.def_env(p.field_name.clone(), p.field_type.clone());
            }

            for stmt in &mut fn_decl.body {
                if let Err(e) = self.check_stmt(stmt) {
                    self.errors_count += 1;
                    Self::report_msg(e.to_string());
                }
            }
            trace!("checked function : '{}'", &fn_decl.name);
            fns.push(fn_decl);
        }

        (self.errors_count, fns)
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
        trace!("defining var: {} of type : {}", &name, &ty);
        let l = self.env.insert(name, ty);
        trace!("env: {:?}", self.env);
        l
    }

    pub(super) fn get_env(&self, name: &String) -> Option<&DType> {
        trace!("retrieve var: {}", &name);
        let l = self.env.get(name);
        trace!("env: {:?}", self.env);
        l
    }

    fn report_msg<M: Into<String>>(msg: M) {
        report_message(msg.into(), MessageType::Error(ErrorType::TypeError))
    }
}
