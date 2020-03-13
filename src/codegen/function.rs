use super::*;
use llvm::LLVMIntPredicate;

struct FunctionEnvironment {
    variables: HashMap<String, LLVMValueRef>,
    basic_blocks: HashMap<String, LLVMBasicBlockRef>,
    global: Environment,
}

impl FunctionEnvironment {
    pub fn new(global: Environment) -> Self {
        Self {
            global,
            ..Default::default()
        }
    }
}

impl Default for FunctionEnvironment {
    fn default() -> Self {
        Self {
            variables: HashMap::new(),
            basic_blocks: HashMap::new(),
            global: Environment::default(),
        }
    }
}

pub struct Function {
    pub module: Rc<RefCell<module::Module>>,
    pub builder: LLVMBuilderRef,
    pub context: LLVMContextRef,
    environment: FunctionEnvironment,
    value: LLVMValueRef,
    name: String,
    current_basic_block: Option<LLVMBasicBlockRef>,
}

impl Function {
    pub fn new(codegen: &mut Codegen, function_type: Types, name: &str) -> Self {
        let value = build_function(codegen, function_type, name);
        Self {
            module: codegen.module.clone(),
            builder: codegen.builder,
            context: codegen.context,
            environment: FunctionEnvironment::new(codegen.environment.clone()),
            value,
            name: String::from(name),
            current_basic_block: None,
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

    //pub fn position_at_block(&mut self, name: &str) {
    //    self.position_at_block_ref(*self.environment.basic_blocks.get_mut(name).unwrap())
    //    //unsafe {
    //    //    LLVMPositionBuilderAtEnd(
    //    //        self.builder,
    //    //        *self.environment.basic_blocks.get_mut(name).unwrap(),
    //    //    )
    //    //}
    //}

    pub fn position_at_block_ref(&mut self, block_ref: LLVMBasicBlockRef) {
        unsafe { LLVMPositionBuilderAtEnd(self.builder, block_ref) }
        self.current_basic_block = Some(block_ref);
    }

    fn build_cond_br(
        &mut self,
        condition: LLVMValueRef,
        then: LLVMBasicBlockRef,
        else_block: LLVMBasicBlockRef,
    ) -> LLVMValueRef {
        unsafe { LLVMBuildCondBr(self.builder, condition, then, else_block) }
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
                Expression::Modulus(left, right) => LLVMBuildOr(
                    self.builder,
                    self.build_expression(left),
                    self.build_expression(right),
                    self.module.borrow().empty_string(),
                ),
                Expression::Equality(left, right) => {
                    let left = self.build_expression(left);
                    let right = self.build_expression(right);
                    self.build_icmp(LLVMIntPredicate::LLVMIntEQ, left, right)
                }
                Expression::NotEq(left, right) => {
                    let left = self.build_expression(left);
                    let right = self.build_expression(right);
                    self.build_icmp(LLVMIntPredicate::LLVMIntNE, left, right)
                }
                Expression::LessThan(left, right) => {
                    let left = self.build_expression(left);
                    let right = self.build_expression(right);
                    self.build_icmp(LLVMIntPredicate::LLVMIntSLT, left, right)
                }
                Expression::GreaterThan(left, right) => {
                    let left = self.build_expression(left);
                    let right = self.build_expression(right);
                    self.build_icmp(LLVMIntPredicate::LLVMIntSGT, left, right)
                }
                Expression::LessEq(left, right) => {
                    let left = self.build_expression(left);
                    let right = self.build_expression(right);
                    self.build_icmp(LLVMIntPredicate::LLVMIntSLE, left, right)
                }
                Expression::GreaterEq(left, right) => {
                    let left = self.build_expression(left);
                    let right = self.build_expression(right);
                    self.build_icmp(LLVMIntPredicate::LLVMIntSGE, left, right)
                }
                Expression::And(left, right) => LLVMBuildAnd(
                    self.builder,
                    self.build_expression(left),
                    self.build_expression(right),
                    self.module.borrow().empty_string(),
                ),
                Expression::Or(left, right) => LLVMBuildOr(
                    self.builder,
                    self.build_expression(left),
                    self.build_expression(right),
                    self.module.borrow().empty_string(),
                ),
                Expression::Not(expr) => LLVMBuildNot(
                    self.builder,
                    self.build_expression(expr),
                    self.module.borrow().empty_string(),
                ),
                Expression::If(if_expression) => self.build_if_expression(if_expression),
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
                    Value::Literal(Literal::String(string)) => self.build_global_string(&string),
                    Value::Variable(name) => {
                        if let Some(var) = self.var(name) {
                            self.build_load(var)
                        } else {
                            panic!("Variable '{}' has not been declared yet", name);
                        }
                    }
                    Value::FunctionCall(func) => {
                        let mut args = Vec::new();
                        for arg in func.arguments.iter() {
                            args.push(self.build_expression(&arg));
                        }
                        self.call_other(&func.name, &mut args, "")
                    }
                    _ => panic!("Not yet implemented!"),
                },
            }
        }
    }

    unsafe fn build_icmp(
        &mut self,
        op: LLVMIntPredicate,
        left: LLVMValueRef,
        right: LLVMValueRef,
    ) -> LLVMValueRef {
        LLVMBuildICmp(
            self.builder,
            op,
            left,
            right,
            self.module.borrow().empty_string(),
        )
    }

    pub fn build_block(&mut self, block: &Block) -> LLVMValueRef {
        for statement in block.iter().take(block.len() - 1) {
            self.build_statement(statement);
        }
        self.build_statement(block.last().unwrap())
    }

    pub fn build_if_expression(&mut self, if_expression: &IfExpression) -> LLVMValueRef {
        let current_basic_block = self.current_basic_block.unwrap();
        let after = self.basic_block("after");
        let condition = self.build_expression(&if_expression.condition);
        let if_block = self.basic_block("if");
        self.position_at_block_ref(if_block);
        let if_body = self.build_block(&if_expression.body);
        let mut incoming = vec![(self.current_basic_block.unwrap(), if_body)];
        self.build_br(after);
        let else_block = match &if_expression.else_expression {
            ElseExpression::Block(block) => {
                let basic_block = self.basic_block("else");
                self.position_at_block_ref(basic_block);
                incoming.push((basic_block, self.build_block(&block)));
                self.build_br(after);
                basic_block
            }
            ElseExpression::IfExpression(expr) => {
                let val = self.build_if_expression(&expr);
                self.build_br(after);
                incoming.push((self.current_basic_block.unwrap(), val));
                self.current_basic_block.unwrap()
            }
            ElseExpression::None => {
                let noop_block = self.basic_block("noop");
                incoming.push((noop_block, self.build_const_int(0)));
                self.position_at_block_ref(noop_block);
                self.build_br(after);
                noop_block
            }
        };
        self.position_at_block_ref(current_basic_block);
        self.build_cond_br(condition, if_block, else_block);
        self.position_at_block_ref(after);
        self.build_phi(Types::Int, incoming)
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

    pub fn call_other(
        &self,
        function_name: &str,
        args: &mut [LLVMValueRef],
        name: &str,
    ) -> LLVMValueRef {
        unsafe {
            println!("function_name: {}, self.name: {}", function_name, self.name);
            let other = if function_name == self.name {
                self.value
            } else {
                self.environment
                    .global
                    .borrow()
                    .get(function_name)
                    .unwrap()
                    .value
            };
            LLVMBuildCall(
                self.builder,
                other,
                args.as_mut_ptr(),
                args.len() as c_uint,
                self.module.borrow_mut().new_string_ptr(name),
            )
        }
    }

    pub fn get_param(&mut self, index: c_uint) -> LLVMValueRef {
        unsafe { LLVMGetParam(self.value, index) }
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
