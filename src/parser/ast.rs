use crate::lexer::lex::Token;
use crate::parser::parser_error::ParserError;
use crate::SyntaxMode;
use num_bigint::BigInt;
use std::fmt;
use std::fmt::Formatter;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ASTNode {
    Program(Vec<ASTNode>),
    Block(Block),
    Declaration(Declaration),
    Expression(Expression),
    Statement(Statement),
    Function(Function),
    Error(ParserError),
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<Statement>,
    pub syntax_mode: SyntaxMode,
    pub indent_level: Option<usize>, // Pour le mode Indentation
    pub braces: Option<(Token, Token)>, // Pour le mode Braces (ouverture, fermeture)
    // pub opening_brace: Option<Token>,  // pour le mode syntaxe Brace
    // pub closing_brace: Option<Token>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub position: Position,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}
#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Addition,
    Substraction,
    Multiplication,
    Division,
    Modulo,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    And,
    Or,
    LesshanOrEqual,
    GreaterThanOrEqual,
}

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Negate,
    Not,
    Increment,
    Decrement,
    Reference,
    Dereference,
    BitwiseNot,
    LogicalNot,
    Positive,
    Negative,
}


#[allow(dead_code)]
#[derive(Debug, Clone,PartialEq,Eq)]
pub struct GenericType{
    pub base: String,           // Nom du type
    pub parameters: Vec<Type>, //   Paramètres génériques
}

#[allow(dead_code)]
#[derive(Debug, Clone,PartialEq,Eq)]
pub enum Type {
    Int,
    Float,
    String,
    Bool,
    Char,
    Array(Box<Type>),
    Tuple(Vec<Type>),
    Custom(String),
    Generic(GenericType),
    Infer, // Type inféré déduire par le compilateur
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Declaration {
    Variable(VariableDeclaration),
    Function(FunctionDeclaration),
    Constante(ConstanteDeclaration),
    Structure(StructDeclaration),
    Class(ClassDeclaration),
    Enum(EnumDeclaration),
    Trait(TraitDeclaration),
    Impl(ImplDeclaration),
    Module(ModuleDeclaration),
    Macro(MacroDeclaration),
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct VariableDeclaration {
    pub name: String,
    pub variable_type: Option<Type>,
    pub value: Option<Expression>,
    pub mutable: bool,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FunctionDeclaration {
    pub name: String,
    pub parameters: Vec<(String, Type)>, // (nom, type)
    pub return_type: Option<Type>,
    pub body: Block,
    //pub annotations: Vec<Annotation>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ConstanteDeclaration {
    pub name: String,
    pub constant_type: Option<Type>,
    pub value: Expression,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct StructDeclaration {
    pub name: String,
    pub fields: Vec<Field>,

}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ClassDeclaration {
    pub name: String,
    pub parent_class: Option<String>,
    pub fields: Vec<Field>,
    pub methods: Vec<FunctionDeclaration>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EnumDeclaration {
    pub name: String,
    pub variants: Vec<EnumVariant>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TraitDeclaration {
    pub name: String,
    pub methods: Vec<FunctionSignature>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ImplDeclaration {
    pub trait_name: String,
    pub for_type: Type,
    pub methods: Vec<FunctionDeclaration>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ModuleDeclaration {
    pub name: String,
    pub statements: Vec<Statement>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MacroDeclaration {
    pub name: String,
    pub parameters: Vec<String>,
    pub body: Block,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Field{
    pub name: String,
    pub field_type: Type,
    pub mutable: bool, //  si neccessaire
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EnumVariant{
    pub name: String,
    pub associated_type: Option<Vec<Type>>, // None si pas de type associé
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FunctionSignature{
    pub name: String,
    pub parameters: Vec<(String,Type)>,
    pub return_type: Option<Type>,

}
// #[allow(dead_code)]
// #[derive(Debug, Clone)]
// pub struct Annotation{
//     pub name: String,
//     pub value: Option<Expression>,
// }

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal),
    Identifier(String),
    BinaryOperation(BinaryOperation),
    UnaryOperation(UnaryOperation),
    FunctionCall(FunctionCall),
    ArrayAccess(ArrayAccess),
    MemberAccess(MemberAccess),
    LambdaExpression(LambdaExpression),
    MatchExpression(MatchExpression),
    MatchArms(Box<MatchArms>),
    TypeCast(TypeCast),
    Conditional(Conditional),
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Literal {
    Integer { value: BigInt },
    Float { value: f64 },
    String(String),
    Boolean(bool),
    Array(Vec<Expression>),
}

//fonction parametre
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Parameters {
    pub name: String,
    pub parameter_type: Option<Type>,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct BinaryOperation {
    pub left: Box<Expression>,
    pub operator: Operator, ///////////////////// a changer
    pub right: Box<Expression>,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct UnaryOperation {
    pub operator: UnaryOperator,
    pub operand: Box<Expression>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: Vec<Expression>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ArrayAccess {
    pub array: Box<Expression>,
    pub index: Box<Expression>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MemberAccess {
    pub object: Box<Expression>,
    pub member: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TypeCast {
    pub expression: Box<Expression>,
    pub target_type: Type,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Conditional {
    pub condition: Box<Expression>,
    pub then_block: Box<Expression>,
    pub else_block: Box<Expression>,
}


#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum Statement {
    Expression(Expression),
    Return(ReturnStatement),
    Use(UseStatement),
    Import(ImportStatement),
    Raise(RaiseStatement),
    Del(DelStatement),
    If(IfStatement),
    Elif(ElifStatement),
    While(WhileStatement),
    For(ForStatement),
    Break,
    Continue,
    Try(TryStatement),
    With(WithStatement),
    Yield(YieldStatement),

    Declaration(Declaration),
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ReturnStatement {
    pub value: Option<Expression>,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct IfStatement {
    pub condition: Expression,
    pub block: Block,
    pub elif_blocks: Vec<(Expression, Block)>,
    pub else_block: Option<Block>,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ElifStatement {
    pub condition: Expression,
    pub block: Block,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct WhileStatement {
    pub condition: Expression,
    pub block: Block,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ForStatement {
    pub variable_iter: String,
    pub iterable: Expression,
    pub block: Block,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct UseStatement {
    pub module: String,
    pub alias: Option<String>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ImportStatement {
    pub module: String,
    pub alias: Option<String>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RaiseStatement {
    pub exception: Expression,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DelStatement {
    pub target: Expression,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TryStatement {
    pub block: Block,
    pub except: Vec<(Option<String>, Block)>,
    pub else_block: Option<Block>,
    pub finally_block: Option<Block>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct WithStatement {
    pub target: Expression,
    pub block: Block,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct YieldStatement {
    pub value: Option<Expression>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Function {
    pub declaration: FunctionDeclaration,
    pub block: Block,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Identifier {
    pub name: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LambdaExpression {
    pub parameters: Vec<Parameters>,
    pub block: Block,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MatchExpression {
    pub expression: Box<Expression>,
    pub arms: Vec<MatchArms>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MatchArms {
    pub pattern: Pattern,
    pub expression: Box<Expression>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Pattern {
    Literal(Literal),
    Identifier(String),
    Wildcard,
    EnumVariant(EnumVariant),
}

impl fmt::Display for ASTNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ASTNode::Program(statements) => {
                for statement in statements {
                    write!(f, "{}", statement)?;
                }
                Ok(())
            }
            ASTNode::Declaration(decl) => write!(f, "{:?}", decl),
            ASTNode::Expression(expr) => write!(f, "{:?}", expr),
            ASTNode::Statement(stmt) => write!(f, "{:?}", stmt),
            ASTNode::Function(func) => write!(f, "{:?}", func),
            ASTNode::Block(block) => write!(f, "{:?}", block),
            // ASTNode::IfStatement(ifstmt) => write!(f, "{}", ifstmt),
            // ASTNode::ForStatement(forstmt) => write!(f, "{}", forstmt),
            // ASTNode::WhileStatement(whilestmt) => write!(f, "{}", whilestmt),
            // ASTNode::ReturnStatement(retstmt) => write!(f, "{}", retstmt),
            // ASTNode::BinaryOperation(binop) => write!(f, "{}", binop),
            // ASTNode::UnaryOperation(unop) => write!(f, "{}", unop),
            // ASTNode::Identifier(ident) => write!(f, "{}", ident),
            // ASTNode::Literal(lit) => write!(f, "{}", lit),
            // ASTNode::Operator(op) => write!(f, "{}", op),
            ASTNode::Error(err) => write!(f, "{}", err),
        }
    }
}

impl Block {
    pub fn is_indentation_mode(&self) -> bool{
        matches!(self.syntax_mode, SyntaxMode::Indentation)
    }
    pub fn validate(&self) -> Result<(),String>{
        match self.syntax_mode {
            SyntaxMode::Indentation if self.indent_level.is_none() => {
                Err("Indentation level is missing".to_string())
            }
            SyntaxMode::Braces if self.braces.is_none() => {
                Err("Braces are missing".to_string())
            }
            _ => Ok(()),
        }
    }

}

impl ASTNode{
    pub fn program(statements: Vec<ASTNode>) -> Self{
        ASTNode::Program(statements)
    }
    pub fn block(block: Block) -> Self{
        ASTNode::Block(block)
    }
    pub fn declaration(declaration: Declaration) -> Self{
        ASTNode::Declaration(declaration)
    }
    pub fn expression(expression: Expression) -> Self{
        ASTNode::Expression(expression)
    }
    pub fn statement(statement: Statement) -> Self{
        ASTNode::Statement(statement)
    }
    pub fn function(function: Function) -> Self{
        ASTNode::Function(function)
    }
    pub fn error(error: ParserError) -> Self{
        ASTNode::Error(error)
    }
}

// by YmC

////////////////////////////////////////////////////////////////
