use anyhow::bail;
use evelin::{
    ast::{FnDecl, StructDecl, Token},
    emitter::{Emitter, qbee::QBEEmitter},
    lexer::Lexer,
    parser::Parser,
    passes, type_sys,
};

pub fn tokenize<T: Into<String>>(input: T) -> Vec<evelin::lexer::Token> {
    let l = input.into();
    let mut lexer = Lexer::from(&l);
    lexer.start().unwrap();
    lexer.tokens().to_owned()
}

#[allow(dead_code)]
pub fn lex(tokens: Vec<Token>) -> (Vec<FnDecl>, Vec<StructDecl>, usize) {
    let mut parser = Parser::from(&tokens);
    parser.parse();
    (parser.fn_decls, parser.struct_decls, parser.errors_count)
}

#[allow(dead_code)]
pub fn parse_fn(source: &str) -> Vec<FnDecl> {
    let tokens = tokenize(source.to_string());
    let mut parser = Parser::from(&tokens);
    parser.parse();
    parser.fn_decls
}

#[allow(dead_code)]
pub fn parser_struct(source: &str) -> Vec<StructDecl> {
    let tokens = tokenize(source.to_string());
    let mut parser = Parser::from(&tokens);
    parser.parse();
    parser.struct_decls
}

#[allow(dead_code)]
pub fn compile<T: Into<String>>(input: T) -> Result<String, anyhow::Error> {
    let tokens = tokenize(&input.into());

    let (fns, sts, errors_count) = lex(tokens);
    if errors_count != 0 {
        bail!("Failed to compile due to {} parsing error(s)", errors_count);
    }

    let (mut fn_decls, mut struct_decls) = match passes::run_passes(fns, sts) {
        Ok((fn_, st)) => (fn_, st),
        Err(errs) => {
            bail!("Failed to compile due to {} error(s)", &errs.len());
        }
    };

    let type_sys = type_sys::TypeSystem::new(&mut fn_decls, &mut struct_decls);
    let (type_error_count, fn_decls) = type_sys.check();
    if type_error_count != 0 {
        bail!(
            "Failed to compile due to {} type error(s)",
            type_error_count
        );
    }

    let mut qbe_generator = QBEEmitter::from((&fn_decls, &struct_decls));
    let ir = qbe_generator.emit_ir()?;

    Ok(ir)
}
