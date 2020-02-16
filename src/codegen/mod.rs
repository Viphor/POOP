use crate::parser::ast::Expression;
use llvm::core::*;
use llvm::prelude::*;

mod error;

type Output<Out = ()> = Result<Out, error::CodegenError>;

pub fn build_expression(context: LLVMContextRef, expr: Expression) -> Output {
    unimplemented!()
}
