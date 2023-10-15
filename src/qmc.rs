#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Tri {
    /// false
    F,
    /// true
    T,
    /// don't care
    X,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Imp(Vec<Tri>);

impl Imp {
    pub fn merge(&self, other: &Self) -> Option<Self> {
        assert_eq!(self.0.len(), other.0.len());
        let mut merged = false;
        let mut result = vec![];
        for (l, r) in self.0.iter().zip(other.0.iter()) {
            match (*l, *r) {
                (Tri::F, Tri::T) | (Tri::T, Tri::F) => {
                    if merged {
                        // more than one merge point
                        return None;
                    } else {
                        result.push(Tri::X);
                        merged = true;
                    }
                }
                (Tri::F, Tri::F) | (Tri::T, Tri::T) | (Tri::X, Tri::X) => result.push(*l),
                (_, Tri::X) | (Tri::X, _) => return None,
            }
        }
        Some(Self(result))
    }
}

#[cfg(test)]
mod test {
    use crate::qmc::{Imp, Tri};

    #[test]
    fn merge() {
        let m0 = Imp(vec![Tri::F, Tri::T, Tri::F, Tri::F]);
        let m1 = Imp(vec![Tri::T, Tri::T, Tri::F, Tri::F]);
        let m2 = Imp(vec![Tri::X, Tri::T, Tri::F, Tri::F]);
        let m3 = Imp(vec![Tri::X, Tri::F, Tri::F, Tri::F]);
        let m4 = Imp(vec![Tri::X, Tri::X, Tri::F, Tri::F]);
        assert_eq!(m0.merge(&m1), Some(m2.clone()));
        assert_eq!(m0.merge(&m2), None);
        assert_eq!(m2.merge(&m3), Some(m4.clone()));
    }
}
