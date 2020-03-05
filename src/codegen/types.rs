use llvm::core::*;
use llvm::prelude::{LLVMContextRef, LLVMTypeRef};
use std::os::raw::c_uint;

pub enum Types {
    Int,
    Bool,
    Char,
    String,
    Void,
    Array(Box<Types>, c_uint),
    Func(Box<Types>, Vec<Types>, bool),
}

impl Types {
    pub fn to_llvm(&self, context: LLVMContextRef) -> LLVMTypeRef {
        unsafe {
            match self {
                Types::Int => LLVMInt64TypeInContext(context),
                Types::Bool => LLVMInt8TypeInContext(context),
                Types::Char => LLVMInt8TypeInContext(context),
                Types::String => LLVMPointerType(Types::Char.to_llvm(context), 0),
                Types::Void => LLVMVoidType(),
                Types::Array(element_type, count) => {
                    LLVMArrayType(element_type.to_llvm(context), *count)
                }
                Types::Func(ret, args, variadic) => LLVMFunctionType(
                    ret.to_llvm(context),
                    args.iter()
                        .map(|arg| arg.to_llvm(context))
                        .collect::<Vec<LLVMTypeRef>>()
                        .as_mut_ptr(),
                    args.len() as u32,
                    if *variadic { 1 } else { 0 },
                ),
            }
        }
    }

    pub fn printf() -> Self {
        Types::Func(Box::new(Types::Int), vec![Types::String], true)
    }

    pub fn main() -> Self {
        Types::Func(Box::new(Types::Int), Vec::new(), false)
    }
}
