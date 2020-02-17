use crate::parser::ast::Expression;
use llvm::core::*;
use llvm::prelude::*;

mod error;

type Output<Out = ()> = Result<Out, error::CodegenError>;

pub struct Codegen {
    context: LLVMContextRef,
    module: LLVMModuleRef,
    builder: LLVMBuilderRef,
}

impl Codegen {
    fn new<T: Into<[u8]>>(module_name: T) -> Self {
        
    }

    fn new_with_context(context: LLVMContextRef) 
}

pub fn build_expression(context: LLVMContextRef, expr: Expression) -> Output {
    unimplemented!()
}
