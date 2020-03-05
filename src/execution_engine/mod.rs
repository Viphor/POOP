use llvm::execution_engine::*;
//use llvm::prelude::*;
use llvm::target::*;
use std::cell::RefCell;
use std::mem;
use std::os::raw::{c_char, c_int, c_uint};
use std::rc::Rc;

use super::codegen::{function::Function, module::Module, Codegen};

pub struct ExecutionEngine {
    _codegen: Codegen,
    module: Rc<RefCell<Module>>,
    ee: LLVMExecutionEngineRef,
}

impl ExecutionEngine {
    pub fn new(codegen: Codegen) -> Self {
        let mut ee;
        let module = codegen.module();
        unsafe {
            ee = mem::MaybeUninit::uninit().assume_init();
            let mut out = mem::zeroed();
            LLVMLinkInMCJIT();
            LLVM_InitializeNativeTarget();
            LLVM_InitializeNativeAsmPrinter();

            //println!("Out before: {:?}", out);
            //println!("EE before: {:?}", ee);
            LLVMCreateExecutionEngineForModule(&mut ee, module.borrow().module, &mut out);
            //println!("Out after: {:?}", out);
            //println!("EE after: {:?}", ee);
        }

        Self {
            _codegen: codegen,
            module,
            ee,
        }
    }

    pub fn get_function(&mut self, function_name: &str) -> extern "C" fn() -> () {
        let function_name_ptr = self.module.borrow_mut().new_string_ptr(function_name);
        unsafe {
            //println!("Function name: {}", function_name);
            let addr = LLVMGetFunctionAddress(self.ee, function_name_ptr);
            //println!("{:?}", addr);
            mem::transmute(addr)
        }
    }

    pub fn run_as_main(&mut self, main_fn: Function, argv: &[&str]) -> c_int {
        unsafe {
            LLVMRunFunctionAsMain(
                self.ee,
                main_fn.value(),
                argv.len() as c_uint,
                argv.iter()
                    .map(|arg| self.module.borrow_mut().new_string_ptr(arg))
                    .collect::<Vec<*const c_char>>()
                    .as_mut_ptr(),
                mem::zeroed(),
            )
        }
    }
}

impl Drop for ExecutionEngine {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeExecutionEngine(self.ee);
        }
    }
}
