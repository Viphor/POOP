//! This module takes care of converting the AST into llvm IR

use crate::parser::ast::*;
#[cfg(feature = "codegen-debug")]
use llvm::analysis::*;
use llvm::core::*;
use llvm::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::CString;
use std::ops::Deref;
use std::os::raw::c_uint;
use std::rc::Rc;

pub mod error;
pub mod function;
pub mod module;
mod traits;
pub mod types;

use traits::Builder;
use types::Types;

#[derive(Clone)]
pub struct Environment(Rc<RefCell<HashMap<String, function::Function>>>);

impl Default for Environment {
    fn default() -> Self {
        Self(Rc::new(RefCell::new(HashMap::new())))
    }
}

impl Deref for Environment {
    type Target = Rc<RefCell<HashMap<String, function::Function>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Codegen {
    context: LLVMContextRef,
    module: Rc<RefCell<module::Module>>,
    builder: LLVMBuilderRef,
    environment: Environment,
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
                environment: Environment::default(),
            }
        }
    }

    pub fn module(&self) -> Rc<RefCell<module::Module>> {
        self.module.clone()
    }

    fn build_function(&mut self, function: FuncDecl) {
        let ft = Types::Func(
            Box::new(function.return_type.clone().into()),
            function
                .args
                .iter()
                .map(|arg| arg.arg_type.clone().into())
                .collect(),
            false,
        );

        let mut func = function::Function::new(self, ft, &function.name);
        func.build(&|func| {
            // Function setup
            let entry = func.basic_block("entry");
            func.position_at_block_ref(entry);
            for (i, arg) in function.args.iter().enumerate() {
                println!("Setting param: {} to name: {}", i, &arg.name);
                let ptr = func.build_stack_ptr(arg.arg_type.clone().into(), &arg.name);
                let val = func.get_param(i as u32);
                func.assign(&arg.name, ptr);
                func.build_store(val, ptr);
            }

            // Actual body
            let block = func.basic_block("block");
            // Transition to new bb
            func.build_br(block);
            func.position_at_block_ref(block);
            let ret = func.build_block(&function.body);
            if let Type::Void = function.return_type {
                func.build_ret_void();
            } else {
                func.build_ret(ret);
            }
        });

        self.environment.borrow_mut().insert(function.name, func);
    }

    fn declare_function(&mut self, function_type: Types, name: &str) {
        let func = function::Function::new(self, function_type, name);
        self.environment
            .borrow_mut()
            .insert(String::from(name), func);
    }

    fn build_program_internal(&mut self, program: Program) {
        match program {
            Program::Decl(Decl::FuncDecl(func_decl), next) => {
                self.build_function(func_decl);
                self.build_program_internal(*next);
            }
            Program::Decl(_, _) => panic!("Global variables are not yet supported"),
            Program::Empty => (),
        }
    }

    /// TEMPORARY! Please delete!
    pub fn build_program(&mut self, program: Program) -> LLVMValueRef {
        //self.declare_function(Types::printf(), "printf");
        ////self.build_function("printf", Types::printf(), &|_, _| {});
        //let ft = Types::Func(Box::new(Types::Int), Vec::new(), false);
        //let mut statement_fn = function::Function::new(self, ft, "statement");
        ////self.build_function("statement", ft, &|codegen, func: &LLVMValueRef| {
        //statement_fn.build(&|func| {
        //    func.basic_block("entry");
        //    func.position_at_block("entry");
        //    //let value = func.build_statement(&statement);
        //    //func.build_ret(value);
        //});

        ////let main = self.build_function("main", Types::main(), &|codegen, func| {
        //let mut main = function::Function::new(self, Types::main(), "main");
        //main.build(&|func| {
        //    func.basic_block("entry");
        //    func.position_at_block("entry");
        //    let format = func.build_global_string("Result of statement: %d\n");
        //    let output = statement_fn.call(&mut [], "");
        //    printf.call(&mut [format, output], "bar");
        //    let ret = func.build_const_int(0);
        //    func.build_ret(ret);
        //});

        self.declare_function(Types::printf(), "printf");

        self.build_program_internal(program);

        #[cfg(feature = "codegen-debug")]
        {
            unsafe {
                LLVMDumpModule(self.module.borrow().module);
            }
        }
        self.environment
            .borrow()
            .get("main")
            .expect("There must be a main function")
            .value()
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
