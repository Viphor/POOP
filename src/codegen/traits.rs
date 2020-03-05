use super::*;

pub trait Builder {
    //fn build_function(
    //    &mut self,
    //    function_name: &str,
    //    function_type: Types,
    //    body: &dyn Fn(&mut Codegen, &LLVMValueRef) -> (),
    //) -> LLVMValueRef;
    //fn build_basic_block(&mut self, block_name: &str, function: LLVMValueRef) -> LLVMBasicBlockRef;
    fn build_const_int(&mut self, value: u64) -> LLVMValueRef;
    fn build_const_string(&mut self, value: &str) -> LLVMValueRef;
    fn build_function_call(
        &mut self,
        function_name: &str,
        args: &mut [LLVMValueRef],
        name: &str,
    ) -> LLVMValueRef;
    fn build_store(&mut self, value: LLVMValueRef, ptr: LLVMValueRef) -> LLVMValueRef;
    fn build_load(&mut self, ptr: LLVMValueRef) -> LLVMValueRef;
    fn build_bit_cast(&mut self, value: LLVMValueRef, dest_type: Types) -> LLVMValueRef;
    fn build_stack_ptr(&mut self, stack_type: Types, name: &str) -> LLVMValueRef;
    fn build_stack_str(&mut self, value: &str) -> LLVMValueRef;
    fn build_ret_void(&mut self) -> LLVMValueRef;
    fn build_ret(&mut self, value: LLVMValueRef) -> LLVMValueRef;
    fn position_at_block(&mut self, basic_block: LLVMBasicBlockRef);
    fn build_const_pointer(&mut self, value: LLVMValueRef, to_type: Types) -> LLVMValueRef;
    fn build_null(&mut self, null_type: Types) -> LLVMValueRef;
    fn build_global_string(&mut self, value: &str) -> LLVMValueRef;
}

impl Builder for Codegen {
    //fn build_function(
    //    &mut self,
    //    function_name: &str,
    //    function_type: Types,
    //    body: &dyn Fn(&mut Codegen, &LLVMValueRef) -> (),
    //) -> LLVMValueRef {
    //    let func;
    //    let module;
    //    {
    //        module = self.module.borrow().module
    //    }
    //    unsafe {
    //        func = LLVMAddFunction(
    //            module,
    //            self.module.borrow_mut().new_string_ptr(function_name),
    //            function_type.to_llvm(self.context),
    //        );
    //    }
    //    body(self, &func);

    //    #[cfg(feature = "codegen-debug")]
    //    println!("Error in {}: {}", function_name, verify_fn(func));

    //    func
    //}

    //fn build_basic_block(&mut self, block_name: &str, function: LLVMValueRef) -> LLVMBasicBlockRef {
    //    unsafe {
    //        LLVMAppendBasicBlockInContext(
    //            self.context,
    //            function,
    //            self.module.borrow_mut().new_string_ptr(block_name),
    //        )
    //    }
    //}

    fn build_const_int(&mut self, value: u64) -> LLVMValueRef {
        unsafe { LLVMConstInt(Types::Int.to_llvm(self.context), value, 1) }
    }

    fn build_const_string(&mut self, value: &str) -> LLVMValueRef {
        let value_ptr = self.module.borrow_mut().new_string_ptr(value);
        unsafe { LLVMConstStringInContext(self.context, value_ptr, value.len() as u32, 0) }
    }

    fn build_function_call(
        &mut self,
        function_name: &str,
        args: &mut [LLVMValueRef],
        name: &str,
    ) -> LLVMValueRef {
        unsafe {
            let function = LLVMGetNamedFunction(
                self.module.borrow().module,
                self.module.borrow_mut().new_string_ptr(function_name),
            );
            //println!("{}: {:?}", function_name, function);

            LLVMBuildCall(
                self.builder,
                function,
                args.as_mut_ptr(),
                args.len() as c_uint,
                to_ptr(name),
            )
        }
    }

    fn build_store(&mut self, value: LLVMValueRef, ptr: LLVMValueRef) -> LLVMValueRef {
        unsafe { LLVMBuildStore(self.builder, value, ptr) }
    }

    fn build_load(&mut self, ptr: LLVMValueRef) -> LLVMValueRef {
        unsafe { LLVMBuildLoad(self.builder, ptr, self.module.borrow().empty_string()) }
    }

    fn build_bit_cast(&mut self, value: LLVMValueRef, dest_type: Types) -> LLVMValueRef {
        unsafe {
            LLVMBuildBitCast(
                self.builder,
                value,
                dest_type.to_llvm(self.context),
                self.module.borrow().empty_string(),
            )
        }
    }

    fn build_stack_ptr(&mut self, stack_type: Types, name: &str) -> LLVMValueRef {
        unsafe {
            LLVMBuildAlloca(
                self.builder,
                stack_type.to_llvm(self.context),
                self.module.borrow_mut().new_string_ptr(name),
            )
        }
    }

    fn build_const_pointer(&mut self, value: LLVMValueRef, to_type: Types) -> LLVMValueRef {
        unsafe { LLVMConstPointerCast(value, to_type.to_llvm(self.context)) }
    }

    fn build_null(&mut self, null_type: Types) -> LLVMValueRef {
        unsafe { LLVMConstNull(null_type.to_llvm(self.context)) }
    }

    fn build_stack_str(&mut self, value: &str) -> LLVMValueRef {
        let heap = self.build_stack_ptr(
            Types::Array(Box::new(Types::Char), (value.len() + 1) as c_uint),
            "",
        );
        let string = self.build_const_string(value);
        self.build_store(string, heap);
        self.build_bit_cast(heap, Types::String)
    }

    fn build_global_string(&mut self, value: &str) -> LLVMValueRef {
        let string;
        {
            let mut module = self.module.borrow_mut();
            unsafe {
                string = LLVMBuildGlobalString(
                    self.builder,
                    module.new_string_ptr(value),
                    module.empty_string(),
                )
            }
        }
        self.build_bit_cast(string, Types::String)
    }

    fn build_ret_void(&mut self) -> LLVMValueRef {
        unsafe { LLVMBuildRetVoid(self.builder) }
    }

    fn build_ret(&mut self, value: LLVMValueRef) -> LLVMValueRef {
        unsafe { LLVMBuildRet(self.builder, value) }
    }

    fn position_at_block(&mut self, basic_block: LLVMBasicBlockRef) {
        unsafe {
            LLVMPositionBuilderAtEnd(self.builder, basic_block);
        }
    }
}

impl Builder for function::Function {
    fn build_const_int(&mut self, value: u64) -> LLVMValueRef {
        unsafe { LLVMConstInt(Types::Int.to_llvm(self.context), value, 1) }
    }

    fn build_const_string(&mut self, value: &str) -> LLVMValueRef {
        let value_ptr = self.module.borrow_mut().new_string_ptr(value);
        unsafe { LLVMConstStringInContext(self.context, value_ptr, value.len() as u32, 0) }
    }

    fn build_function_call(
        &mut self,
        function_name: &str,
        args: &mut [LLVMValueRef],
        name: &str,
    ) -> LLVMValueRef {
        unsafe {
            let function = LLVMGetNamedFunction(
                self.module.borrow().module,
                self.module.borrow_mut().new_string_ptr(function_name),
            );
            //println!("{}: {:?}", function_name, function);

            LLVMBuildCall(
                self.builder,
                function,
                args.as_mut_ptr(),
                args.len() as c_uint,
                to_ptr(name),
            )
        }
    }

    fn build_store(&mut self, value: LLVMValueRef, ptr: LLVMValueRef) -> LLVMValueRef {
        unsafe { LLVMBuildStore(self.builder, value, ptr) }
    }

    fn build_load(&mut self, ptr: LLVMValueRef) -> LLVMValueRef {
        unsafe { LLVMBuildLoad(self.builder, ptr, self.module.borrow().empty_string()) }
    }

    fn build_bit_cast(&mut self, value: LLVMValueRef, dest_type: Types) -> LLVMValueRef {
        unsafe {
            LLVMBuildBitCast(
                self.builder,
                value,
                dest_type.to_llvm(self.context),
                self.module.borrow().empty_string(),
            )
        }
    }

    fn build_stack_ptr(&mut self, stack_type: Types, name: &str) -> LLVMValueRef {
        unsafe {
            LLVMBuildAlloca(
                self.builder,
                stack_type.to_llvm(self.context),
                self.module.borrow_mut().new_string_ptr(name),
            )
        }
    }

    fn build_const_pointer(&mut self, value: LLVMValueRef, to_type: Types) -> LLVMValueRef {
        unsafe { LLVMConstPointerCast(value, to_type.to_llvm(self.context)) }
    }

    fn build_null(&mut self, null_type: Types) -> LLVMValueRef {
        unsafe { LLVMConstNull(null_type.to_llvm(self.context)) }
    }

    fn build_stack_str(&mut self, value: &str) -> LLVMValueRef {
        let heap = self.build_stack_ptr(
            Types::Array(Box::new(Types::Char), (value.len() + 1) as c_uint),
            "",
        );
        let string = self.build_const_string(value);
        self.build_store(string, heap);
        self.build_bit_cast(heap, Types::String)
    }

    fn build_global_string(&mut self, value: &str) -> LLVMValueRef {
        let string;
        {
            let mut module = self.module.borrow_mut();
            unsafe {
                string = LLVMBuildGlobalString(
                    self.builder,
                    module.new_string_ptr(value),
                    module.empty_string(),
                )
            }
        }
        self.build_bit_cast(string, Types::String)
    }

    fn build_ret_void(&mut self) -> LLVMValueRef {
        unsafe { LLVMBuildRetVoid(self.builder) }
    }

    fn build_ret(&mut self, value: LLVMValueRef) -> LLVMValueRef {
        unsafe { LLVMBuildRet(self.builder, value) }
    }

    fn position_at_block(&mut self, basic_block: LLVMBasicBlockRef) {
        unsafe {
            LLVMPositionBuilderAtEnd(self.builder, basic_block);
        }
    }
}
