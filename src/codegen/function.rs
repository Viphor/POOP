use super::*;

struct Environment {
    variables: HashMap<String, LLVMValueRef>,
    basic_blocks: HashMap<String, LLVMBasicBlockRef>,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            variables: HashMap::new(),
            basic_blocks: HashMap::new(),
        }
    }
}

pub struct Function {
    pub module: Rc<RefCell<module::Module>>,
    pub builder: LLVMBuilderRef,
    pub context: LLVMContextRef,
    environment: Environment,
    value: LLVMValueRef,
    name: String,
}

impl Function {
    pub fn new(codegen: &mut Codegen, function_type: Types, name: &str) -> Self {
        let value = build_function(codegen, function_type, name);
        Self {
            module: codegen.module.clone(),
            builder: codegen.builder,
            context: codegen.context,
            environment: Environment::default(),
            value,
            name: String::from(name),
        }
    }

    pub fn call(&self, args: &mut [LLVMValueRef], name: &str) -> LLVMValueRef {
        unsafe {
            LLVMBuildCall(
                self.builder,
                self.value,
                args.as_mut_ptr(),
                args.len() as c_uint,
                self.module.borrow_mut().new_string_ptr(name),
            )
        }
    }

    pub fn basic_block(&mut self, name: &str) -> LLVMBasicBlockRef {
        unsafe {
            self.environment.basic_blocks.insert(
                name.to_string(),
                LLVMAppendBasicBlockInContext(
                    self.context,
                    self.value,
                    self.module.borrow_mut().new_string_ptr(name),
                ),
            );
            *self.environment.basic_blocks.get_mut(name).unwrap()
        }
    }

    pub fn position_at_block(&mut self, name: &str) {
        unsafe {
            LLVMPositionBuilderAtEnd(
                self.builder,
                *self.environment.basic_blocks.get_mut(name).unwrap(),
            )
        }
    }

    pub fn build_statement(&mut self, statement: &Statement) -> LLVMValueRef {
        match statement {
            Statement::VarDecl(var_decl) => {
                let ptr = self.build_stack_ptr(Types::Int, &var_decl.identifier);
                let value = self.build_expression(&var_decl.expression);
                self.assign(&var_decl.identifier, ptr);
                self.build_store(value, ptr)
            }
            Statement::Expression(expr) => self.build_expression(expr),
            Statement::Empty => self.build_null(Types::Void),
        }
    }

    pub fn build_expression(&mut self, expr: &Expression) -> LLVMValueRef {
        unsafe {
            match expr {
                Expression::Addition(left, right) => LLVMBuildAdd(
                    self.builder,
                    self.build_expression(left),
                    self.build_expression(right),
                    self.module.borrow().empty_string(),
                ),
                Expression::Subtraction(left, right) => LLVMBuildSub(
                    self.builder,
                    self.build_expression(left),
                    self.build_expression(right),
                    self.module.borrow().empty_string(),
                ),
                Expression::Multiplication(left, right) => LLVMBuildMul(
                    self.builder,
                    self.build_expression(left),
                    self.build_expression(right),
                    self.module.borrow().empty_string(),
                ),
                Expression::Division(left, right) => LLVMBuildSDiv(
                    self.builder,
                    self.build_expression(left),
                    self.build_expression(right),
                    self.module.borrow().empty_string(),
                ),
                Expression::Modulus(_left, _right) => unimplemented!(),
                Expression::Equality(_left, _right) => unimplemented!(),
                Expression::Block(block) => self.build_block(block),
                Expression::Value(value) => match value {
                    Value::Literal(Literal::Number(Number::Int(int))) => {
                        LLVMConstInt(Types::Int.to_llvm(self.context), *int as u64, 1)
                    }
                    Value::Literal(Literal::Boolean(boolean)) => LLVMConstInt(
                        Types::Bool.to_llvm(self.context),
                        if *boolean { 1 } else { 0 },
                        0,
                    ),
                    Value::Variable(name) => {
                        if let Some(var) = self.var(name) {
                            self.build_load(var)
                        } else {
                            panic!("Variable '{}' has not been declared yet", name);
                        }
                    }
                    _ => panic!("Not yet implemented!"),
                },
            }
        }
    }

    pub fn build_block(&mut self, block: &Block) -> LLVMValueRef {
        for statement in block.iter().take(block.len() - 1) {
            self.build_statement(statement);
        }
        self.build_statement(block.last().unwrap())
    }

    pub fn build(&mut self, builder: &dyn Fn(&mut Function) -> ()) {
        builder(self);

        #[cfg(feature = "codegen-debug")]
        println!("Error in {}: {}", self.name(), self.verify());
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn assign(&mut self, var: &str, value: LLVMValueRef) {
        self.environment.variables.insert(var.to_string(), value);
    }

    pub fn var(&self, var: &str) -> Option<LLVMValueRef> {
        match self.environment.variables.get(var) {
            Some(val) => Some(*val),
            None => None,
        }
    }

    pub fn value(&self) -> LLVMValueRef {
        self.value
    }

    #[cfg(feature = "codegen-debug")]
    pub fn verify(&self) -> bool {
        unsafe {
            LLVMVerifyFunction(
                self.value,
                LLVMVerifierFailureAction::LLVMPrintMessageAction,
            ) != 0
        }
    }
}

fn build_function(codegen: &mut Codegen, function_type: Types, name: &str) -> LLVMValueRef {
    let mut module = codegen.module.borrow_mut();
    unsafe {
        LLVMAddFunction(
            module.module,
            module.new_string_ptr(name),
            function_type.to_llvm(codegen.context),
        )
    }
}
