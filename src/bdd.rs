use std::collections::HashMap;
use std::rc::Rc;
use std::{default, vec};

#[derive(Debug, Clone)]
pub enum BDD {
    L(Leaf),
    N(Node),
}

#[derive(Debug, Clone)]
pub struct Leaf {
    pub label: bool,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub f: Rc<Box<BDD>>,
    pub t: Rc<Box<BDD>>,
}

impl BDD {
    pub fn new(n: usize, table: &[bool]) -> Rc<Box<Self>> {
        let (t, f) = (
            Rc::new(Box::new(BDD::L(Leaf { label: true }))),
            Rc::new(Box::new(BDD::L(Leaf { label: false }))),
        );
        let mut prev = vec![];
        let mut next = vec![];
        for layer in 0..(n + 1) {
            prev = next.clone();
            next.clear();
            for i in 0..2_usize.pow((n - layer).try_into().unwrap()) {
                if layer == 0 {
                    next.push(if table[i] { t.clone() } else { f.clone() })
                } else {
                    next.push(Rc::new(Box::new(BDD::N(Node {
                        f: prev[i * 2].clone(),
                        t: prev[i * 2 + 1].clone(),
                    }))))
                }
            }
        }
        next[0].clone()
    }
    fn reduce(bdd: &Rc<Box<BDD>>) -> Rc<Box<BDD>> {
        match bdd.as_ref().as_ref() {
            BDD::N(Node { f, t }) => {
                if Rc::ptr_eq(f, t) {
                    println!("reduced");
                    f.clone()
                } else {
                    Rc::new(Box::new(BDD::N(Node {
                        f: BDD::reduce(f),
                        t: BDD::reduce(t),
                    })))
                }
            }
            _ => bdd.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::BDD;

    #[test]
    fn new() {
        let bdd = BDD::new(3, &[false, false, false, true, true, true, true, true]);
        dbg!(&bdd);
        dbg!(BDD::reduce(&BDD::reduce(&bdd)));
    }
}
