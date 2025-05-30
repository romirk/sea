//! The High-Level Intermediate Representation.

/// A C program (technically a "translation unit").
pub struct Program {
    /// The top-level declarations and definitions in the program.
    pub decls: Vec<TopDefn>,
}

/// A top-level declaration or definition.
pub enum TopDefn {
    /// A function declaration or definition.
    Fn(FnDefn),

    /// A variable declaration or definition.
    Var(VarDefn),

    /// A type definition.
    Type(TypeDefn),
}

/// A function declaration or definition.
pub struct FnDefn {
    /// Attributes on the function.
    pub attrs: (), // TODO

    /// The return type of the function.
    pub rett: Type,

    /// The name of the function.
    pub name: String,

    /// The parameters of the function.
    pub params: Vec<MonoDecl>,

    /// The function body, if any.
    pub body: Option<Block>,
}

/// A variable declaration or definition.
pub struct VarDefn {
    /// Attributes on the function.
    pub attrs: (), // TODO

    /// The base type of the declaration.
    pub base: Type,

    /// The bindings in the declaration, with optional definitions.
    pub bindings: Vec<(Binding, Option<Expr>)>,
}

/// A type definition.
pub struct TypeDefn {
    /// Attributes on the function.
    pub attrs: (), // TODO

    /// The base type of the declaration.
    pub base: Type,

    /// The type bindings in the declaration.
    pub bindings: Vec<Binding>,
}

/// A declaration.
pub struct Decl {
    /// Attributes on the declaration.
    pub attrs: (), // TODO

    /// The base type of the declaration.
    pub base: Type,

    /// The bindings in the declaration.
    pub bindings: Vec<Binding>,
}

/// A declaration with a single binding.
pub struct MonoDecl {
    /// Attributes on the declaration.
    pub attrs: (), // TODO

    /// The base type of the declaration.
    pub base: Type,

    /// The declaration's binding.
    pub binding: Binding,
}

/// A binding.
pub enum Binding {
    /// A function binding.
    Fn {
        /// The inner binding.
        ///
        /// This elaborates the return type.
        inner: Box<Binding>,

        /// Parameters to the function.
        params: Vec<MonoDecl>,
    },

    /// An array binding.
    Array {
        /// The inner binding.
        ///
        /// This elaborates the element type.
        inner: Box<Binding>,

        /// The array size, if specified.
        size: Option<Expr>,
    },

    /// A pointer binding.
    Pointer {
        /// The inner binding.
        ///
        /// This elaborates the element type.
        inner: Box<Binding>,
    },

    /// A parenthesized binding.
    Paren(Box<Binding>),

    /// A variable binding.
    Ident {
        /// The name of the binding.
        name: String,
    },

    /// An anonymous binding.
    ///
    /// This is only allowed in some contexts.
    Anonymous,
}

/// A type.
pub enum Type {
    /// A character type.
    Char {
        /// The signedness of the type, if explicit.
        signed: Option<bool>,
    },

    /// A short integer.
    Short {
        /// The signedness of the type, if explicit.
        signed: Option<bool>,
    },

    /// An integer.
    Int {
        /// The signedness of the type, if explicit.
        signed: Option<bool>,
    },

    /// A long integer.
    Long {
        /// The signedness of the type, if explicit.
        signed: Option<bool>,
    },

    /// A very long integer.
    LongLong {
        /// The signedness of the type, if explicit.
        signed: Option<bool>,
    },

    /// A floating-point number.
    Float,

    /// A double-precision floating-point number.
    Double,

    /// A long double-precision floating-point number.
    LongDouble,

    /// A structure.
    Struct {
        /// Attributes on the struct.
        attrs: (), // TODO

        /// The name of the struct, if specified.
        name: Option<String>,

        /// The fields of the struct.
        fields: Vec<Decl>,
    },

    /// An identifier.
    Ident {
        /// The name of the type.
        name: String,
    },
}

/// A block.
pub struct Block {
    /// The statements making up the block.
    pub stmts: Vec<Stmt>,
}

/// A statement.
pub enum Stmt {
    /// A standalone semicolon.
    Empty,

    /// A block.
    Block {
        /// The statements making up the block.
        stmts: Vec<Stmt>,
    },

    /// A variable declaration/definition.
    VarDefn(VarDefn),

    /// A type definition.
    Type(TypeDefn),

    /// An expression.
    Expr(Expr),

    /// An if statement.
    If {
        /// The condition.
        cond: Expr,

        /// The success case.
        then: Box<Stmt>,

        /// The failure case.
        r#else: Box<Stmt>,
    },

    /// A for loop.
    For {
        /// The initialization expression.
        init: Option<Expr>,

        /// The loop condition, if any.
        cond: Option<Expr>,

        /// The repetition statement.
        rept: Option<Expr>,

        /// The loop body.
        body: Box<Stmt>,
    },

    /// A while loop.
    While {
        /// The loop condition.
        cond: Expr,

        /// The loop body.
        body: Box<Stmt>,
    },

    /// A do-while loop.
    DoWhile {
        /// The loop body.
        body: Box<Stmt>,

        /// The loop condition.
        cond: Expr,
    },

    /// A break statement.
    Break,

    /// A continue statement.
    Continue,

    /// A return statement
    Return { expr: Option<Expr> },
}

/// An expression.
pub enum Expr {
    /// A debug expression
    /// Equivalent to reading `expr` in the source
    Debug,

    /// A reference expression.
    Ref(Box<Expr>),

    /// A binary expression.
    Bin(BinOp, Box<Expr>, Box<Expr>),

    /// A unary expression.
    Una(UnaOp, Box<Expr>),

    /// An identifier.
    Ident(String),
}

/// A binary operation.
pub enum BinOp {
    /// Addition.
    Add,

    /// Subtraction.
    Sub,
}

/// A unary operation.
pub enum UnaOp {
    /// Negation.
    Neg,

    /// Logical inversion.
    Not,

    /// Bitwise inversion.
    Inv,
}
