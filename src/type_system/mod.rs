use crate::parser::ast::*;

mod error;

pub type Output<Out = ()> = Result<Out, error::TypeSystemError>;

pub struct TypeSystem {}

impl TypeSystem {
    pub fn analyze(_ast: &Program) {
        unimplemented!();
    }

    fn statement(&mut self, _statement: &Statement) -> Output<Type> {
        unimplemented!();
    }

    fn expression(&mut self, expr: &Expression) -> Output<Type> {
        match expr {
            Expression::Addition(left, right)
            | Expression::Subtraction(left, right)
            | Expression::Multiplication(left, right)
            | Expression::Division(left, right)
            | Expression::Modulus(left, right) => match self.expression(&left)? {
                left @ Type::Int | left @ Type::Float | left @ Type::Double => {
                    match self.expression(&right)? {
                        right @ Type::Int | right @ Type::Float | right @ Type::Double => {
                            if left == right {
                                Ok(left)
                            } else {
                                Err(error::TypeSystemError::error(format!(
                                    "Type mismatch. Left: {}, Right: {}",
                                    left, right
                                )))
                            }
                        }
                        right => Err(error::TypeSystemError::type_mismatch(
                            vec![Type::Int, Type::Float, Type::Double],
                            right,
                        )),
                    }
                }
                left => Err(error::TypeSystemError::type_mismatch(
                    vec![Type::Int, Type::Float, Type::Double],
                    left,
                )),
            },
            Expression::Equality(left, right)
            | Expression::NotEq(left, right)
            | Expression::LessThan(left, right)
            | Expression::GreaterThan(left, right)
            | Expression::LessEq(left, right)
            | Expression::GreaterEq(left, right)
            | Expression::And(left, right)
            | Expression::Or(left, right) => {
                // This is only for leveraging lazy evaluation
                if {
                    match self.expression(&left)? {
                        Type::Boolean => true,
                        left_type => {
                            return Err(error::TypeSystemError::error(format!(
                                "Type mismatch left hand side. Expected boolean, found: {}",
                                left_type
                            )))
                        }
                    }
                } && {
                    match self.expression(&right)? {
                        Type::Boolean => true,
                        right_type => {
                            return Err(error::TypeSystemError::error(format!(
                                "Type mismatch right hand side. Expected boolean, found: {}",
                                right_type
                            )))
                        }
                    }
                } {
                    Ok(Type::Boolean)
                } else {
                    // This should never happen.
                    // Only here to satisfy compiler
                    Err(error::TypeSystemError::error(
                        "Something went horribly wrong",
                    ))
                }
            }
            Expression::Not(not) => {
                let not_type = self.expression(not)?;
                if let Type::Boolean = not_type {
                    Ok(Type::Boolean)
                } else {
                    Err(error::TypeSystemError::error(format!(
                        "Type mismatch. Expected boolean, found: {}",
                        not_type
                    )))
                }
            }
            Expression::If(if_expr) => self.if_expression(if_expr),
            Expression::Block(block) => self.block(block),
            Expression::Value(value) => self.value(value),
        }
    }

    fn if_expression(&mut self, if_expr: &IfExpression) -> Output<Type> {
        unimplemented!();
    }

    fn block(&mut self, block: &Block) -> Output<Type> {
        unimplemented!();
    }

    fn value(&mut self, value: &Value) -> Output<Type> {
        match value {
            Value::Literal(literal) => match literal {
                Literal::Number(Number::Int(_)) => Ok(Type::Int),
                Literal::Number(Number::Float(_)) => Ok(Type::Float),
                Literal::Number(Number::Double(_)) => Ok(Type::Double),
                Literal::Boolean(_) => Ok(Type::Boolean),
                Literal::String(_) => Ok(Type::String),
            },
            Value::Variable(name) => self.variable(name),
            Value::FunctionCall(func_call) => self.function_call(func_call),
        }
    }

    fn variable(&mut self, var: &str) -> Output<Type> {
        unimplemented!();
    }

    fn function_call(&mut self, func_call: &FunctionCall) -> Output<Type> {
        unimplemented!();
    }
}
