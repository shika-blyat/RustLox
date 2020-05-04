#[derive(Debug, PartialEq)]
pub enum Expr<'a> {
    Ident(&'a str),
    BinExpr {
        left: Box<Expr<'a>>,
        op: BinOp,
        right: Box<Expr<'a>>,
    },
    Instructions {
        left: Box<StmtExpr<'a>>,
        right: Box<StmtExpr<'a>>,
    },
    UnExpr {
        op: UnOp,
        val: Box<Expr<'a>>,
    },
    IfThenElse {
        condition: Box<Expr<'a>>,
        true_branch: Box<Expr<'a>>,
        false_branch: Box<Expr<'a>>,
    },
    While {
        cond: Box<Expr<'a>>,
        body: Box<Expr<'a>>,
    },
    Fun {
        param: &'a str,
        body: Box<Expr<'a>>,
    },
    Call {
        fun: Box<Expr<'a>>,
        param: Box<Expr<'a>>,
    },
    Lit(Lit),
}

#[derive(Debug, PartialEq)]
pub enum Statement<'a> {
    Return(Expr<'a>),
    Break(Expr<'a>),
    VarDecl(VarDecl<'a>),
    FunDecl(FunctionDecl<'a>),
}

#[derive(Debug, PartialEq)]
pub enum StmtExpr<'a> {
    Stmt(Statement<'a>),
    Expr(Expr<'a>),
}
#[derive(Debug, Clone, PartialEq)]
pub enum BinOp {
    Add,
    Mul,
    Div,
    Sub,
    EqEq,
    NotEq,
    And,
    Or,
    /// The semicolon is an operator, basically `a; b` is an expression evaluating to b, with the special case of
    /// `a; b;` where an implicit [`()`](enum.Lit.html#variant.Unit) is added
    Semicolon,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnOp {
    Not,
    Sub,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Lit {
    Num(isize),
    Bool(bool),
    /// The unit type, a.k.a `()`
    Unit,
}

#[derive(Debug, PartialEq)]
pub struct VarDecl<'a> {
    name: String,
    fun: Box<Expr<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct FunctionDecl<'a> {
    name: String,
    fun: Box<Expr<'a>>,
}
