use crate::aig;
use lalrpop_util::lalrpop_mod;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expr {
    Term(char),
    Not(Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Xor(Box<Expr>, Box<Expr>),
}

impl From<Box<Expr>> for Box<aig::AIG> {
    fn from(val: Box<Expr>) -> Self {
        match *val {
            Expr::Term(v) => (v as usize - 'A' as usize).into(),
            Expr::Not(v) => {
                let v: Box<aig::AIG> = v.into();
                !v
            }
            Expr::And(l, r) => {
                let l: Box<aig::AIG> = l.into();
                let r: Box<aig::AIG> = r.into();
                l & r
            }
            Expr::Or(l, r) => {
                let l: Box<aig::AIG> = l.into();
                let r: Box<aig::AIG> = r.into();
                !((!l) & (!r))
            }
            Expr::Xor(l, r) => Box::new(Expr::Or(
                Box::new(Expr::And(l.clone(), Box::new(Expr::Not(r.clone())))),
                Box::new(Expr::And(r.clone(), Box::new(Expr::Not(l.clone())))),
            ))
            .into(),
        }
    }
}

lalrpop_mod!(pub calculator1); // synthesized by LALRPOP

#[test]
fn calculator1() {
    assert_eq!(
        calculator1::ExprParser::new().parse("A").unwrap(),
        Box::new(Expr::Term('A'))
    );
    assert_eq!(
        calculator1::ExprParser::new().parse("(A|B)").unwrap(),
        Box::new(Expr::Or(
            Box::new(Expr::Term('A')),
            Box::new(Expr::Term('B'))
        ))
    );
    assert_eq!(
        calculator1::ExprParser::new().parse("(A&B)").unwrap(),
        Box::new(Expr::And(
            Box::new(Expr::Term('A')),
            Box::new(Expr::Term('B'))
        ))
    );

    assert_eq!(
        calculator1::ExprParser::new()
            .parse("((A&(!B))|(!(A&B)))")
            .unwrap(),
        Box::new(Expr::Or(
            Box::new(Expr::And(
                Box::new(Expr::Term('A')),
                Box::new(Expr::Not(Box::new(Expr::Term('B'))))
            )),
            Box::new(Expr::Not(Box::new(Expr::And(
                Box::new(Expr::Term('A')),
                Box::new(Expr::Term('B'))
            ))))
        ))
    );
}
