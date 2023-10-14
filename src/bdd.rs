use std::vec;
use std::rc::Rc;

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
        let mut prev = vec![];
        let mut next = vec![];
        for layer in 0..(n + 1) {
            prev = next.clone();
            next.clear();
            for i in 0..2_usize.pow((n - layer).try_into().unwrap()) {
                if layer == 0 {
                    next.push(Rc::new(Box::new(BDD::L(Leaf { label: table[i] }))))
                } else {
                    next.push(Rc::new(Box::new(BDD::N(Node {
                        f: prev[i * 2].clone(),
                        t: prev[i * 2 + 1].clone(),
                    }))))
                }
            }
        }
        prev[0].clone()
    }
}

#[cfg(test)]
mod test {
    use super::BDD;

    #[test]
    fn new() {
        let bdd = BDD::new(
            4,
            &[
                false, false, false, true, false, true, true, true, false, true, true, true, true,
                true, true, false,
            ],
        );
        dbg!(bdd);
    }
}
