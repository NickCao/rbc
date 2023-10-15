use std::{collections::HashSet, fmt::Display, hash::Hash, ops::Sub};

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum Tri {
    /// false
    F,
    /// true
    T,
    /// don't care
    X,
}

impl Display for Tri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Tri::F => "0",
                Tri::T => "1",
                Tri::X => "-",
            }
        )
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Imp(Vec<Tri>);

impl Display for Imp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(Tri::to_string)
                .collect::<Vec<_>>()
                .join("")
        )
    }
}

impl Imp {
    pub fn containes(&self, other: &Self) -> bool {
        assert_eq!(self.0.len(), other.0.len());
        for (l, r) in self.0.iter().zip(other.0.iter()) {
            match (*l, *r) {
                (Tri::T, Tri::T) | (Tri::F, Tri::F) => continue,
                (Tri::X, _) => continue,
                (_, _) => return false,
            }
        }
        true
    }
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

fn reduce_one(minterms: &HashSet<Imp>) -> (HashSet<Imp>, HashSet<Imp>) {
    let mut next = HashSet::<Imp>::default();
    let mut used = HashSet::<Imp>::default();
    for a in minterms {
        for b in minterms {
            if a == b {
                continue;
            }
            if let Some(v) = a.merge(b) {
                next.insert(v);
                used.insert(a.clone());
                used.insert(b.clone());
            }
        }
    }
    (minterms.sub(&used), next)
}

fn reduce(minterms: &HashSet<Imp>) -> HashSet<Imp> {
    let mut essential = HashSet::<Imp>::default();
    let mut curr = minterms.clone();
    loop {
        let (rem, next) = reduce_one(&curr);
        essential.extend(rem);
        curr = next.clone();
        if next.is_empty() {
            break;
        }
    }
    essential
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::qmc::{Imp, Tri};

    #[test]
    fn basic() {
        let m0 = Imp(vec![Tri::F, Tri::T, Tri::F, Tri::F]);
        let m1 = Imp(vec![Tri::T, Tri::T, Tri::F, Tri::F]);
        let m2 = Imp(vec![Tri::X, Tri::T, Tri::F, Tri::F]);
        let m3 = Imp(vec![Tri::X, Tri::F, Tri::F, Tri::F]);
        let m4 = Imp(vec![Tri::X, Tri::X, Tri::F, Tri::F]);
        assert_eq!(m0.merge(&m1), Some(m2.clone()));
        assert_eq!(m0.merge(&m2), None);
        assert_eq!(m2.merge(&m3), Some(m4.clone()));
        assert_eq!(m2.containes(&m0), true);
        assert_eq!(m2.containes(&m1), true);
        assert_eq!(m3.containes(&m2), false);
    }

    #[test]
    fn reduce() {
        let m4 = Imp(vec![Tri::F, Tri::T, Tri::F, Tri::F]);
        let m8 = Imp(vec![Tri::T, Tri::F, Tri::F, Tri::F]);
        let m9 = Imp(vec![Tri::T, Tri::F, Tri::F, Tri::T]);
        let m10 = Imp(vec![Tri::T, Tri::F, Tri::T, Tri::F]);
        let m11 = Imp(vec![Tri::T, Tri::F, Tri::T, Tri::T]);
        let m12 = Imp(vec![Tri::T, Tri::T, Tri::F, Tri::F]);
        let m14 = Imp(vec![Tri::T, Tri::T, Tri::T, Tri::F]);
        let m15 = Imp(vec![Tri::T, Tri::T, Tri::T, Tri::T]);
        let mset = HashSet::from([m4, m8, m9, m10, m11, m12, m14, m15]);
        dbg!(super::reduce(&mset));
    }
}
