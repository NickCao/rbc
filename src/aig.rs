#[derive(Debug, Clone)]
pub struct Sym(pub usize);

#[derive(Debug, Clone)]
pub struct Inv(pub Box<AIG>);

#[derive(Debug, Clone)]
pub struct And(pub Box<AIG>, pub Box<AIG>);

#[derive(Debug, Clone)]
pub enum AIG {
    A(And),
    I(Inv),
    G(Sym),
}

impl std::ops::BitAnd for Box<AIG> {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Box::new(AIG::A(And(self, rhs)))
    }
}

impl std::ops::Not for Box<AIG> {
    type Output = Self;
    fn not(self) -> Self::Output {
        Box::new(AIG::I(Inv(self)))
    }
}

impl AIG {
    pub fn eval(&self, syms: &[bool]) -> bool {
        match self {
            AIG::A(And(l, r)) => l.eval(syms) & r.eval(syms),
            AIG::I(Inv(r)) => !r.eval(syms),
            AIG::G(Sym(i)) => syms[*i],
        }
    }
}