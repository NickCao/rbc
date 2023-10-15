use lalrpop_util::lalrpop_mod;

#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
    Term(char),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Not(Box<Expr>),
}

lalrpop_mod!(pub calculator1); // synthesized by LALRPOP

#[test]
fn calculator1() {
    assert_eq!(
        calculator1::ExprParser::new().parse("A").unwrap(),
        Box::new(Expr::Term('A'))
    );
    assert_eq!(
        calculator1::ExprParser::new().parse("(A+B)").unwrap(),
        Box::new(Expr::Or(
            Box::new(Expr::Term('A')),
            Box::new(Expr::Term('B'))
        ))
    );
    assert_eq!(
        calculator1::ExprParser::new().parse("(A*B)").unwrap(),
        Box::new(Expr::And(
            Box::new(Expr::Term('A')),
            Box::new(Expr::Term('B'))
        ))
    );
}
