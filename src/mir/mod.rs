//! This module contains all the representations for the Middle Intermediate Representation
//! or MIR for short.
//!
//! This represents the structure that all of our analysis and transformations is based on.
//!
//! All language frontends should transform into this representation.

use std::fmt;
use std::ops::Deref;

mod from;

/// This is the top level node in the MIR
#[derive(Debug, PartialEq)]
pub struct Program {
    /// List of declarations in the program
    pub declarations: Vec<Decl>,
}

/// The different types of declarations
#[derive(Debug, PartialEq)]
pub enum Decl {
    /// Variable declaration
    VarDecl(VarDecl),
    /// Function declaration
    FuncDecl(FuncDecl),
}

/// Function declaration
#[derive(Debug, PartialEq)]
pub struct FuncDecl {
    /// Function name
    pub name: String,
    /// Function arguments
    pub args: Vec<ArgDecl>,
    return_type: Type,
    /// Function body
    pub body: Block,
}

impl FuncDecl {
    /// Creates a new function declaration
    pub fn new(name: &str, args: Vec<ArgDecl>, return_type: Type, body: Block) -> Self {
        Self {
            name: name.to_string(),
            args,
            return_type,
            body,
        }
    }
}

impl HasType for FuncDecl {
    fn return_type(&self) -> Option<Type> {
        Some(self.return_type.clone())
    }
}

/// Argument declaration
#[derive(Debug, PartialEq)]
pub struct ArgDecl {
    /// Argument name
    pub name: String,
    /// Type of argument
    pub arg_type: Type,
}

impl ArgDecl {
    /// Creates a new argument declaration
    pub fn new(name: &str, arg_type: Type) -> Self {
        Self {
            name: name.to_string(),
            arg_type,
        }
    }
}

impl HasType for ArgDecl {
    fn return_type(&self) -> Option<Type> {
        Some(self.arg_type.clone())
    }
}

/// Basic types
#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    /// Integer
    Int,
    /// Float
    Float,
    /// Double
    Double,
    /// Boolean
    Boolean,
    /// String
    String,
    /// Void
    Void,
    /// User defined type. Currently not supporting inheritance
    UserDefined(String),
    /// Used internally for when no type has been inferred yet
    NotYetInferred(Vec<Type>),
}

/// This trait defines which constructs actually has a "return" type,
/// i.e. a type which can be used for further type inferrence.
pub trait HasType {
    /// Returns the type if any.
    fn return_type(&self) -> Option<Type>;
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Int => write!(f, "int"),
            Type::Float => write!(f, "float"),
            Type::Double => write!(f, "double"),
            Type::Boolean => write!(f, "boolean"),
            Type::String => write!(f, "string"),
            Type::Void => write!(f, "void"),
            Type::UserDefined(name) => write!(f, "{}", name),
            Type::NotYetInferred(possibilities) => {
                write!(f, "Type is not inferred yet. Possible types:")?;
                for possiblility in possibilities.iter() {
                    write!(f, " '{}'", possiblility)?;
                }
                Ok(())
            }
        }
    }
}

/// Statements
#[derive(Debug, PartialEq)]
pub enum Statement {
    /// Variable declaration statement
    VarDecl(VarDecl),
    /// Expression statement
    Expression(Expression),
    /// Empty statement
    Empty,
}

impl HasType for Statement {
    fn return_type(&self) -> Option<Type> {
        match self {
            Self::VarDecl(_) => None,
            Self::Expression(expr) => expr.return_type(),
            Self::Empty => Some(Type::Void),
        }
    }
}

/// Code block
#[derive(Debug, PartialEq)]
pub struct Block(Vec<Statement>);

impl Block {
    /// Creates a new code block
    pub fn new(statements: Vec<Statement>) -> Self {
        Self(statements)
    }
}

impl HasType for Block {
    fn return_type(&self) -> Option<Type> {
        self.0.last().and_then(|last| last.return_type())
    }
}

impl Deref for Block {
    type Target = Vec<Statement>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Variable declaration
#[derive(Debug, PartialEq)]
pub struct VarDecl {
    /// Name of the variable
    pub identifier: String,
    var_type: Type,
    /// Expression assigned to the variable
    pub expression: Expression,
}

impl VarDecl {
    /// Creates a new variable declaration
    pub fn new(identifier: String, var_type: Type, expression: Expression) -> Self {
        Self {
            identifier,
            var_type,
            expression,
        }
    }
}

impl HasType for VarDecl {
    fn return_type(&self) -> Option<Type> {
        Some(self.var_type.clone())
    }
}

/// Expression
#[derive(Debug, PartialEq)]
pub enum Expression {
    /// Binary operation expression
    BinaryOp(BinaryOpContainer),
    /// Unary operation expression
    UnaryOp(UnaryOpContainer),
    /// If expression
    If(IfExpressionContainer),
    /// Block expression
    Block(Block),
    /// Value expression
    Value(Value),
}

impl HasType for Expression {
    fn return_type(&self) -> Option<Type> {
        unimplemented!()
    }
}

/// Container used to go around the circular nature
pub type BinaryOpContainer = Box<BinaryOp>;

/// Binary operation
#[derive(Debug, PartialEq)]
pub struct BinaryOp {
    left: Expression,
    right: Expression,
    op: Operator,
}

impl BinaryOp {
    /// Creates a new BinaryOp
    pub fn new(left: Expression, right: Expression, op: Operator) -> Self {
        Self { left, right, op }
    }
}

impl HasType for BinaryOp {
    fn return_type(&self) -> Option<Type> {
        unimplemented!()
    }
}

/// Container used to go around the circular nature
pub type UnaryOpContainer = Box<UnaryOp>;

/// Unary operation
#[derive(Debug, PartialEq)]
pub struct UnaryOp {
    expression: Expression,
    op: Operator,
}

impl UnaryOp {
    /// Creates a new UnaryOp
    pub fn new(expression: Expression, op: Operator) -> Self {
        Self { expression, op }
    }
}

impl HasType for UnaryOp {
    fn return_type(&self) -> Option<Type> {
        unimplemented!()
    }
}

/// Types of basic operators
#[derive(Debug, PartialEq)]
pub enum Operator {
    /// Plus `+`
    Plus,
    /// Minus `-`
    Minus,
    /// Star `*`
    Star,
    /// Slash `/`
    Slash,
    /// Percent `%`
    Percent,
    /// Equality `==`
    Equality,
    /// Not equal `!=`
    NotEq,
    /// Less than `<`
    LessThan,
    /// Greater than `>`
    GreaterThan,
    /// Less than or equal `<=`
    LessEq,
    /// Greater than or equal `>=`
    GreaterEq,
    /// And `&&`
    And,
    /// Or `||`
    Or,
    /// Not `!`
    Not,
}

/// Container used to go around the circular nature
pub type IfExpressionContainer = Box<IfExpression>;

/// If expression
#[derive(Debug, PartialEq)]
pub struct IfExpression {
    /// Condition expression
    pub condition: Expression,
    /// Body block
    pub body: Block,
    /// Else expression
    pub else_expression: ElseExpression,
}

impl IfExpression {
    /// Creates a new if expression
    pub fn new(condition: Expression, body: Block, else_expression: ElseExpression) -> Self {
        Self {
            condition,
            body,
            else_expression,
        }
    }
}

/// Else expression can either be an else block, another if expression or nothing
#[derive(Debug, PartialEq)]
pub enum ElseExpression {
    /// Else block expression
    Block(Block),
    /// Else if expression
    IfExpression(IfExpressionContainer),
    /// No else block
    None,
}

/// Value
#[derive(Debug, PartialEq)]
pub enum Value {
    /// Literal value
    Literal(Literal),
    /// Variable, contains the identifier of the variable
    Variable(String),
    /// Function call value
    FunctionCall(FunctionCall),
}

/// Literal types
#[derive(Debug, PartialEq)]
pub enum Literal {
    /// Unsigned integer literal
    Integer(isize),
    /// Float literal
    Float(f32),
    /// Double literal
    Double(f64),
    /// Boolean literal
    Boolean(bool),
    /// String literal
    String(String),
}

impl HasType for Literal {
    fn return_type(&self) -> Option<Type> {
        Some(match self {
            Self::Integer(_) => Type::Int,
            Self::Float(_) => Type::Float,
            Self::Double(_) => Type::Double,
            Self::Boolean(_) => Type::Boolean,
            Self::String(_) => Type::String,
        })
    }
}

/// Function call
#[derive(Debug, PartialEq)]
pub struct FunctionCall {
    /// Name of the function
    pub name: String,
    /// Arguments of the function
    pub arguments: Vec<Expression>,
}

impl FunctionCall {
    /// Creates a new function call
    pub fn new(name: &str, arguments: Vec<Expression>) -> Self {
        Self {
            name: String::from(name),
            arguments,
        }
    }
}

impl HasType for FunctionCall {
    fn return_type(&self) -> Option<Type> {
        unimplemented!()
    }
}
