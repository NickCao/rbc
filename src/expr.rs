use lalrpop_util::lalrpop_mod;

pub enum Expr {
    Term(String),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Not(Box<Expr>),
}

lalrpop_mod!(pub calculator1); // synthesized by LALRPOP

#[test]
fn calculator1() {
    assert!(calculator1::ExprParser::new().parse("A").is_ok());
    assert!(calculator1::ExprParser::new().parse("(A+B)").is_ok());
    assert!(calculator1::ExprParser::new().parse("((A*B)+C)").is_ok());
}
