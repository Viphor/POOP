//! Contains the implementations for conversion from the AST to the MIR

use super::*;
use crate::parser::ast;

impl From<ast::Program> for Program {
    fn from(program: ast::Program) -> Self {
        let mut current_decl = program;
        let mut declarations: Vec<Decl> = Vec::new();

        while let ast::Program::Decl(decl, rest) = current_decl {
            declarations.push(decl.into());
            current_decl = *rest;
        }

        Self { declarations }
    }
}

impl From<ast::Decl> for Decl {
    fn from(decl: ast::Decl) -> Self {
        match decl {
            ast::Decl::FuncDecl(func_decl) => Decl::FuncDecl(func_decl.into()),
            ast::Decl::VarDecl(var_decl) => Decl::VarDecl((&var_decl).into()),
        }
    }
}

impl From<ast::FuncDecl> for FuncDecl {
    fn from(func_decl: ast::FuncDecl) -> Self {
        FuncDecl::new(
            &func_decl.name,
            func_decl.args.iter().map(|arg| arg.into()).collect(),
            func_decl.return_type.into(),
            func_decl.body.into(),
        )
    }
}

impl From<&ast::ArgDecl> for ArgDecl {
    fn from(arg_decl: &ast::ArgDecl) -> Self {
        ArgDecl::new(&arg_decl.name, arg_decl.arg_type.clone().into())
    }
}

impl From<ast::Type> for Type {
    fn from(other: ast::Type) -> Self {
        match other {
            ast::Type::Boolean => Self::Boolean,
            ast::Type::Double => Self::Double,
            ast::Type::Float => Self::Float,
            ast::Type::Int => Self::Int,
            ast::Type::String => Self::String,
            ast::Type::Void => Self::Void,
            ast::Type::UserDefined(name) => Self::UserDefined(name),
        }
    }
}

impl From<&ast::Statement> for Statement {
    fn from(statement: &ast::Statement) -> Self {
        match statement {
            ast::Statement::VarDecl(var_decl) => Statement::VarDecl(var_decl.into()),
            ast::Statement::Expression(expr) => Statement::Expression(expr.into()),
            ast::Statement::Empty => Statement::Empty,
        }
    }
}

impl From<&ast::Block> for Block {
    fn from(block: &ast::Block) -> Self {
        Block::new(block.iter().map(|statement| statement.into()).collect())
    }
}

impl From<ast::Block> for Block {
    fn from(block: ast::Block) -> Self {
        Self::from(&block)
    }
}

impl From<&ast::VarDecl> for VarDecl {
    fn from(var_decl: &ast::VarDecl) -> Self {
        Self::new(
            var_decl.identifier.clone(),
            Type::NotYetInferred(Vec::new()),
            var_decl.expression.clone().into(),
        )
    }
}

impl From<&ast::Expression> for Expression {
    fn from(expr: &ast::Expression) -> Self {
        match expr {
            ast::Expression::Addition(lhs, rhs) => {
                Self::BinaryOp(BinaryOpContainer::new(BinaryOp::new(
                    (**lhs).clone().into(),
                    (**rhs).clone().into(),
                    Operator::Plus,
                )))
            }
            ast::Expression::Subtraction(lhs, rhs) => {
                Self::BinaryOp(BinaryOpContainer::new(BinaryOp::new(
                    (**lhs).clone().into(),
                    (**rhs).clone().into(),
                    Operator::Minus,
                )))
            }
            ast::Expression::Multiplication(lhs, rhs) => {
                Self::BinaryOp(BinaryOpContainer::new(BinaryOp::new(
                    (**lhs).clone().into(),
                    (**rhs).clone().into(),
                    Operator::Star,
                )))
            }
            ast::Expression::Division(lhs, rhs) => {
                Self::BinaryOp(BinaryOpContainer::new(BinaryOp::new(
                    (**lhs).clone().into(),
                    (**rhs).clone().into(),
                    Operator::Slash,
                )))
            }
            ast::Expression::Modulus(lhs, rhs) => {
                Self::BinaryOp(BinaryOpContainer::new(BinaryOp::new(
                    (**lhs).clone().into(),
                    (**rhs).clone().into(),
                    Operator::Percent,
                )))
            }
            ast::Expression::Equality(lhs, rhs) => {
                Self::BinaryOp(BinaryOpContainer::new(BinaryOp::new(
                    (**lhs).clone().into(),
                    (**rhs).clone().into(),
                    Operator::Equality,
                )))
            }
            ast::Expression::NotEq(lhs, rhs) => {
                Self::BinaryOp(BinaryOpContainer::new(BinaryOp::new(
                    (**lhs).clone().into(),
                    (**rhs).clone().into(),
                    Operator::NotEq,
                )))
            }
            ast::Expression::LessThan(lhs, rhs) => {
                Self::BinaryOp(BinaryOpContainer::new(BinaryOp::new(
                    (**lhs).clone().into(),
                    (**rhs).clone().into(),
                    Operator::LessThan,
                )))
            }
            ast::Expression::GreaterThan(lhs, rhs) => {
                Self::BinaryOp(BinaryOpContainer::new(BinaryOp::new(
                    (**lhs).clone().into(),
                    (**rhs).clone().into(),
                    Operator::GreaterThan,
                )))
            }
            ast::Expression::LessEq(lhs, rhs) => {
                Self::BinaryOp(BinaryOpContainer::new(BinaryOp::new(
                    (**lhs).clone().into(),
                    (**rhs).clone().into(),
                    Operator::LessEq,
                )))
            }
            ast::Expression::GreaterEq(lhs, rhs) => {
                Self::BinaryOp(BinaryOpContainer::new(BinaryOp::new(
                    (**lhs).clone().into(),
                    (**rhs).clone().into(),
                    Operator::GreaterEq,
                )))
            }
            ast::Expression::And(lhs, rhs) => {
                Self::BinaryOp(BinaryOpContainer::new(BinaryOp::new(
                    (**lhs).clone().into(),
                    (**rhs).clone().into(),
                    Operator::And,
                )))
            }
            ast::Expression::Or(lhs, rhs) => Self::BinaryOp(BinaryOpContainer::new(BinaryOp::new(
                (**lhs).clone().into(),
                (**rhs).clone().into(),
                Operator::Or,
            ))),
            ast::Expression::Not(expr) => Self::UnaryOp(UnaryOpContainer::new(UnaryOp::new(
                (**expr).clone().into(),
                Operator::Or,
            ))),
            ast::Expression::If(expr) => Self::If(expr.into()),
            ast::Expression::Block(block) => Self::Block(block.into()),
            ast::Expression::Value(value) => Self::Value((*value).clone().into()),
        }
    }
}

impl From<ast::Expression> for Expression {
    fn from(expr: ast::Expression) -> Self {
        Self::from(&expr)
    }
}

impl From<&ast::IfExpressionContainer> for IfExpressionContainer {
    fn from(if_expr: &ast::IfExpressionContainer) -> Self {
        let if_expr = if_expr.clone();
        IfExpressionContainer::new(IfExpression::new(
            if_expr.condition.into(),
            if_expr.body.into(),
            if_expr.else_expression.into(),
        ))
    }
}

impl From<ast::ElseExpression> for ElseExpression {
    fn from(else_expression: ast::ElseExpression) -> Self {
        match else_expression {
            ast::ElseExpression::Block(block) => ElseExpression::Block(block.into()),
            ast::ElseExpression::IfExpression(if_expr) => {
                ElseExpression::IfExpression((&if_expr).into())
            }
            ast::ElseExpression::None => ElseExpression::None,
        }
    }
}

impl From<ast::Value> for Value {
    fn from(value: ast::Value) -> Self {
        match value {
            ast::Value::Literal(lit) => Self::Literal(lit.into()),
            ast::Value::Variable(name) => Self::Variable(name),
            ast::Value::FunctionCall(func) => Self::FunctionCall(func.into()),
        }
    }
}

impl From<ast::Literal> for Literal {
    fn from(lit: ast::Literal) -> Self {
        match lit {
            ast::Literal::Number(num) => num.into(),
            ast::Literal::Boolean(boolean) => Self::Boolean(boolean),
            ast::Literal::String(string) => Self::String(string),
        }
    }
}

impl From<ast::Number> for Literal {
    fn from(num: ast::Number) -> Self {
        match num {
            ast::Number::Int(int) => Self::Integer(int),
            ast::Number::Float(float) => Self::Float(float),
            ast::Number::Double(double) => Self::Double(double),
        }
    }
}

impl From<ast::FunctionCall> for FunctionCall {
    fn from(call: ast::FunctionCall) -> Self {
        Self::new(
            &call.name,
            call.arguments.iter().map(|expr| expr.into()).collect(),
        )
    }
}
