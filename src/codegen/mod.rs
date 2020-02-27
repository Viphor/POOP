use crate::parser::ast::*;
#[cfg(feature = "codegen-debug")]
use llvm::analysis::*;
use llvm::core::*;
use llvm::prelude::*;
use std::ffi::CString;
use std::os::raw::c_uint;

pub mod error;
pub mod module;
mod traits;
pub mod types;

use traits::Builder;
use types::Types;

pub struct Codegen {
    context: LLVMContextRef,
    module: module::Module,
    builder: LLVMBuilderRef,
}

impl Codegen {
    pub fn new(module_name: &str) -> Self {
        unsafe {
            let context = LLVMContextCreate();
            Self::new_with_context(context, module_name)
        }
    }

    pub fn new_with_context(context: LLVMContextRef, module_name: &str) -> Self {
        unsafe {
            let module = module::Module::new(context, module_name);
            let builder = LLVMCreateBuilderInContext(context);

            Self {
                context,
                module,
                builder,
            }
        }
    }

    pub fn module(&self) -> &module::Module {
        &self.module
    }

    pub fn module_mut(&mut self) -> &mut module::Module {
        &mut self.module
    }

    pub fn build_expression(&mut self, expr: &Expression) -> LLVMValueRef {
        unsafe {
            match expr {
                Expression::Addition(left, right) => LLVMBuildAdd(
                    self.builder,
                    self.build_expression(left),
                    self.build_expression(right),
                    self.module.empty_string(),
                ),
                Expression::Subtraction(left, right) => LLVMBuildSub(
                    self.builder,
                    self.build_expression(left),
                    self.build_expression(right),
                    self.module.empty_string(),
                ),
                Expression::Multiplication(left, right) => LLVMBuildMul(
                    self.builder,
                    self.build_expression(left),
                    self.build_expression(right),
                    self.module.empty_string(),
                ),
                Expression::Division(left, right) => LLVMBuildSDiv(
                    self.builder,
                    self.build_expression(left),
                    self.build_expression(right),
                    self.module.empty_string(),
                ),
                Expression::Modulus(_left, _right) => unimplemented!(),
                Expression::Equality(_left, _right) => unimplemented!(),
                Expression::Value(value) => match value {
                    Value::Literal(Literal::Number(Number::Int(int))) => {
                        LLVMConstInt(Types::Int.to_llvm(self.context), *int as u64, 1)
                    }
                    Value::Literal(Literal::Boolean(boolean)) => LLVMConstInt(
                        Types::Bool.to_llvm(self.context),
                        if *boolean { 1 } else { 0 },
                        0,
                    ),
                    _ => panic!("Not yet implemented!"),
                },
            }
        }
    }

    /// TEMPORARY! Please delete!
    pub fn build_program(&mut self, expr: Expression) -> LLVMValueRef {
        self.build_function("printf", Types::printf(), &|_, _| {});
        let ft = Types::Func(Box::new(Types::Int), Vec::new(), false);
        self.build_function("expr", ft, &|codegen, func: &LLVMValueRef| {
            let bb = codegen.build_basic_block("entry", *func);
            codegen.position_at_block(bb);
            let value = codegen.build_expression(&expr);
            codegen.build_ret(value);
        });

        let main = self.build_function("main", Types::main(), &|codegen, func| {
            let bb = codegen.build_basic_block("entry", *func);
            codegen.position_at_block(bb);
            let format = codegen.build_heap_string("Result of expr: %d\n");
            let output = codegen.build_function_call("expr", &mut [], "");
            codegen.build_function_call("printf", &mut [format, output], "bar");
            let ret = codegen.build_const_int(0);
            codegen.build_ret(ret);
        });

        #[cfg(feature = "codegen-debug")]
        {
            unsafe {
                LLVMDumpModule(self.module.module);
            }
        }
        main
    }
}

impl Drop for Codegen {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeBuilder(self.builder);
            LLVMContextDispose(self.context);
        }
    }
}

fn to_cstring(input: &str) -> CString {
    CString::new(input).expect("CString::new failed")
}

fn to_ptr(input: &str) -> *const i8 {
    to_cstring(input).as_ptr() as *const _
}

#[cfg(feature = "codegen-debug")]
fn verify_fn(func: LLVMValueRef) -> bool {
    unsafe { LLVMVerifyFunction(func, LLVMVerifierFailureAction::LLVMPrintMessageAction) != 0 }
}
