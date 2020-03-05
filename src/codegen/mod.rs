use crate::parser::ast::*;
#[cfg(feature = "codegen-debug")]
use llvm::analysis::*;
use llvm::core::*;
use llvm::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::CString;
use std::os::raw::c_uint;
use std::rc::Rc;

pub mod error;
pub mod function;
pub mod module;
mod traits;
pub mod types;

use traits::Builder;
use types::Types;

pub struct Codegen {
    context: LLVMContextRef,
    module: Rc<RefCell<module::Module>>,
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
            let module = Rc::new(RefCell::new(module::Module::new(context, module_name)));
            let builder = LLVMCreateBuilderInContext(context);

            Self {
                context,
                module,
                builder,
            }
        }
    }

    pub fn module(&self) -> Rc<RefCell<module::Module>> {
        self.module.clone()
    }

    /// TEMPORARY! Please delete!
    pub fn build_program(&mut self, statement: Statement) -> function::Function {
        let printf = function::Function::new(self, Types::printf(), "printf");
        //self.build_function("printf", Types::printf(), &|_, _| {});
        let ft = Types::Func(Box::new(Types::Int), Vec::new(), false);
        let mut statement_fn = function::Function::new(self, ft, "statement");
        //self.build_function("statement", ft, &|codegen, func: &LLVMValueRef| {
        statement_fn.build(&|func| {
            func.basic_block("entry");
            func.position_at_block("entry");
            let value = func.build_statement(&statement);
            func.build_ret(value);
        });

        //let main = self.build_function("main", Types::main(), &|codegen, func| {
        let mut main = function::Function::new(self, Types::main(), "main");
        main.build(&|func| {
            func.basic_block("entry");
            func.position_at_block("entry");
            let format = func.build_global_string("Result of statement: %d\n");
            let output = statement_fn.call(&mut [], "");
            printf.call(&mut [format, output], "bar");
            let ret = func.build_const_int(0);
            func.build_ret(ret);
        });

        #[cfg(feature = "codegen-debug")]
        {
            unsafe {
                LLVMDumpModule(self.module.borrow().module);
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
