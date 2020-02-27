use super::*;

pub struct Module {
    pub module: LLVMModuleRef,
    strings: Vec<CString>,
    empty_string: CString,
}

impl Module {
    pub fn new(context: LLVMContextRef, name: &str) -> Self {
        let c_name = to_cstring(name);

        let llvm_module;
        unsafe {
            llvm_module = LLVMModuleCreateWithNameInContext(c_name.as_ptr(), context);
        }

        Self {
            module: llvm_module,
            strings: vec![c_name],
            empty_string: to_cstring(""),
        }
    }

    pub fn new_string_ptr(&mut self, value: &str) -> *const i8 {
        if value.is_empty() {
            return self.empty_string();
        }

        let c_string = to_cstring(value);
        let ptr = c_string.as_ptr();
        self.strings.push(c_string);
        ptr
    }

    pub fn empty_string(&self) -> *const i8 {
        self.empty_string.as_ptr()
    }
}

//impl Drop for Module {
//    fn drop(&mut self) {
//        unsafe {
//            //LLVMDisposeModule(self.module);
//        }
//    }
//}
