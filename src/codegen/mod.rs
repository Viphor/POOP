use crate::parser::ast::*;
use llvm::core::*;
use llvm::prelude::*;
use std::ffi::CString;

pub mod error;
pub mod types;

use types::Types;

//type Output<Out = ()> = Result<Out, error::CodegenError>;

pub struct Codegen {
    context: LLVMContextRef,
    module: LLVMModuleRef,
    builder: LLVMBuilderRef,
    counter: usize,
}

impl Codegen {
    pub fn new<T: Into<CString>>(module_name: T) -> Self {
        unsafe {
            let context = LLVMContextCreate();
            Self::new_with_context(context, module_name)
        }
    }

    pub fn new_with_context<T: Into<CString>>(context: LLVMContextRef, module_name: T) -> Self {
        unsafe {
            let module = LLVMModuleCreateWithNameInContext(
                // This might break, as the string goes out of scope, and gets deallocated
                // when this scope ends, and might need to be used inside LLVM.
                module_name.into().as_ptr() as *const _,
                context,
            );
            let builder = LLVMCreateBuilderInContext(context);

            Self {
                context,
                module,
                builder,
                counter: 0,
            }
        }
    }

    fn build_function<T: Into<CString>>(
        &mut self,
        function_name: T,
        function_type: &Types,
    ) -> LLVMValueRef {
        unsafe {
            LLVMAddFunction(
                self.module,
                function_name.into().as_ptr() as *const _,
                function_type.to_llvm(self.context),
            )
        }
    }

    fn build_basic_block<T: Into<CString>>(
        &mut self,
        block_name: T,
        function: LLVMValueRef,
    ) -> LLVMBasicBlockRef {
        unsafe {
            LLVMAppendBasicBlockInContext(
                self.context,
                function,
                block_name.into().as_ptr() as *const _,
            )
        }
    }

    fn position_at_block(&mut self, basic_block: LLVMBasicBlockRef) {
        unsafe {
            LLVMPositionBuilderAtEnd(self.builder, basic_block);
        }
    }

    pub fn build_main(&mut self) {
        let ft = Types::Func(Box::new(Types::Int), Vec::new());
        let func = self.build_function(to_cstring("main"), &ft);
        let bb = self.build_basic_block(to_cstring("entry"), func);
        self.position_at_block(bb);
    }

    pub fn build_expression(&mut self, expr: Expression) -> LLVMValueRef {
        unsafe {
            match expr {
                Expression::Addition(left, right) => LLVMBuildAdd(
                    self.builder,
                    self.build_expression(*left),
                    self.build_expression(*right),
                    to_cstring(&format!("sum.{}", self.counter)).as_ptr() as *const _,
                ),
                Expression::Subtraction(left, right) => LLVMBuildSub(
                    self.builder,
                    self.build_expression(*left),
                    self.build_expression(*right),
                    to_cstring(&format!("sub.{}", self.counter)).as_ptr() as *const _,
                ),
                Expression::Multiplication(left, right) => LLVMBuildMul(
                    self.builder,
                    self.build_expression(*left),
                    self.build_expression(*right),
                    to_cstring(&format!("sub.{}", self.counter)).as_ptr() as *const _,
                ),
                Expression::Division(left, right) => LLVMBuildSDiv(
                    self.builder,
                    self.build_expression(*left),
                    self.build_expression(*right),
                    to_cstring(&format!("sub.{}", self.counter)).as_ptr() as *const _,
                ),
                Expression::Modulus(_left, _right) => unimplemented!(),
                Expression::Equality(_left, _right) => unimplemented!(),
                Expression::Value(value) => match value {
                    Value::Literal(Literal::Number(Number::Int(int))) => {
                        LLVMConstInt(Types::Int.to_llvm(self.context), int as u64, 1)
                    }
                    Value::Literal(Literal::Boolean(boolean)) => LLVMConstInt(
                        Types::Bool.to_llvm(self.context),
                        if boolean { 1 } else { 0 },
                        0,
                    ),
                    _ => panic!("Not yet implemented!"),
                },
            }
        }
    }

    /// TEMPORARY! Please delete!
    pub fn build_program(&mut self, expr: Expression) {
        self.build_main();
        let value = self.build_expression(expr);
        unsafe {
            LLVMBuildRet(self.builder, value);
            LLVMDisposeBuilder(self.builder);
            LLVMDumpModule(self.module);
        }
    }
}

fn to_cstring(input: &str) -> CString {
    CString::new(input).expect("CString::new failed")
}
