#![allow(unused_variables, unreachable_code)]
#[allow(unused_imports)]
use crate::{
    ast::{BinOp, Expr, Lit, UnOp},
    utils::*,
};

#[test]
fn simple_expr() {
    use crate::ast::{Expr, Lit};

    let simple_addition = Expr::BinExpr {
        left: box_(Expr::Lit(Lit::Num(5))),
        op: BinOp::Add,
        right: box_(Expr::BinExpr {
            left: box_(Expr::Lit(Lit::Num(158))),
            op: BinOp::Mul,
            right: box_(Expr::Lit(Lit::Num(85))),
        }),
    };
    let ast = todo!(); // "5 + 158 * 85"
    assert_eq!(simple_addition, ast);

    let parenthesis = Expr::BinExpr {
        left: box_(Expr::BinExpr {
            left: box_(Expr::Lit(Lit::Num(5))),
            op: BinOp::Add,
            right: box_(Expr::Lit(Lit::Num(158))),
        }),
        op: BinOp::Mul,
        right: box_(Expr::Lit(Lit::Num(85))),
    };
    let ast = todo!(); // "(5 + 158) * 85"
    assert_eq!(parenthesis, ast);
}

#[test]
fn func_calls() {
    let empty_call = Expr::Ident("abc");
    let ast = todo!(); // "abc"
    assert_eq!(empty_call, ast);
    let another_call = Expr::Call {
        fun: box_(Expr::Call {
            fun: box_(Expr::Ident("def")),
            param: box_(Expr::Lit(Lit::Bool(true))),
        }),
        param: box_(Expr::BinExpr {
            left: box_(Expr::Lit(Lit::Num(1))),
            op: BinOp::Add,
            right: box_(Expr::Lit(Lit::Num(2))),
        }),
    };
    let ast = todo!(); // "def true (1 + 2)"
    assert_eq!(another_call, ast);
}

#[test]
fn var_decls() {
    let simple_addition = ();
    let ast = todo!();
    assert_eq!(simple_addition, ast)
}

#[test]
fn func_decls() {
    let simple_addition = ();
    let ast = todo!();
    assert_eq!(simple_addition, ast)
}
