use llvm::core::{LLVMFunctionType, LLVMInt64TypeInContext, LLVMInt8TypeInContext};
use llvm::prelude::{LLVMContextRef, LLVMTypeRef};

pub enum Types {
    Int,
    Bool,
    Func(Box<Types>, Vec<Types>),
}

impl Types {
    pub fn to_llvm(&self, context: LLVMContextRef) -> LLVMTypeRef {
        unsafe {
            match self {
                Types::Int => LLVMInt64TypeInContext(context),
                Types::Bool => LLVMInt8TypeInContext(context),
                Types::Func(ret, args) => LLVMFunctionType(
                    ret.to_llvm(context),
                    args.iter()
                        .map(|arg| arg.to_llvm(context))
                        .collect::<Vec<LLVMTypeRef>>()
                        .as_mut_ptr(),
                    args.len() as u32,
                    0,
                ),
            }
        }
    }
}
