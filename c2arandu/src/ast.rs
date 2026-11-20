#[derive(Debug, Clone)]
pub struct CField {
    pub name: String,
    pub ty: String,
    pub bit_width: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct CStruct {
    pub name: String,
    pub fields: Vec<CField>,
    pub is_union: bool,
}

#[derive(Debug, Clone)]
pub struct CParam {
    pub name: String,
    pub ty: String,
    pub is_mut: bool,
}

#[derive(Debug, Clone)]
pub enum CExpr {
    IntLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    BoolLiteral(bool),
    Ident(String),
    BinaryOp {
        op: String,
        left: Box<CExpr>,
        right: Box<CExpr>,
    },
    UnaryOp {
        op: String,
        operand: Box<CExpr>,
        is_postfix: bool,
    },
    Call {
        callee: Box<CExpr>,
        args: Vec<CExpr>,
    },
    MemberAccess {
        base: Box<CExpr>,
        member: String,
        is_arrow: bool,
    },
    Index {
        base: Box<CExpr>,
        index: Box<CExpr>,
    },
    Cast {
        target_ty: String,
        expr: Box<CExpr>,
    },
    Paren(Box<CExpr>),
    Ternary {
        cond: Box<CExpr>,
        then_expr: Box<CExpr>,
        else_expr: Box<CExpr>,
    },
    Raw(String),
}

#[derive(Debug, Clone)]
pub struct CMatchArm {
    pub pattern: String,
    pub body: Vec<CStmt>,
}

#[derive(Debug, Clone)]
pub enum CStmt {
    VarDecl {
        name: String,
        ty: String,
        init: Option<CExpr>,
        is_mut: bool,
    },
    Assign {
        target: CExpr,
        op: String,
        value: CExpr,
    },
    Expr(CExpr),
    Return(Option<CExpr>),
    If {
        cond: CExpr,
        then_block: Vec<CStmt>,
        else_block: Option<Vec<CStmt>>,
    },
    While {
        cond: CExpr,
        body: Vec<CStmt>,
    },
    DoWhile {
        body: Vec<CStmt>,
        cond: CExpr,
    },
    For {
        init: Option<Box<CStmt>>,
        cond: Option<CExpr>,
        step: Option<CExpr>,
        body: Vec<CStmt>,
    },
    Switch {
        expr: CExpr,
        arms: Vec<CMatchArm>,
    },
    Break,
    Continue,
    Goto(String),
    Label(String),
    Compound(Vec<CStmt>),
}

#[derive(Debug, Clone)]
pub struct CFunction {
    pub name: String,
    pub return_ty: String,
    pub params: Vec<CParam>,
    pub body: Option<Vec<CStmt>>,
    pub is_definition: bool,
    pub is_variadic: bool,
}

#[derive(Debug, Clone)]
pub struct CEnumVariant {
    pub name: String,
    pub value: i64,
}

#[derive(Debug, Clone)]
pub struct CEnum {
    pub name: String,
    pub variants: Vec<CEnumVariant>,
}

#[derive(Debug, Clone)]
pub struct CTypedef {
    pub name: String,
    pub underlying_ty: String,
}

#[derive(Debug, Clone)]
pub struct CGlobal {
    pub name: String,
    pub ty: String,
    pub init: Option<CExpr>,
    pub is_const: bool,
}

#[derive(Debug, Default)]
pub struct CProgram {
    pub typedefs: Vec<CTypedef>,
    pub enums: Vec<CEnum>,
    pub structs: Vec<CStruct>,
    pub globals: Vec<CGlobal>,
    pub functions: Vec<CFunction>,
}
