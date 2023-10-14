use std::rc::Rc;
use std::vec;

#[derive(Debug, Clone)]
pub enum BDD {
    L(Leaf),
    N(Node),
}

#[derive(Debug, Clone)]
pub struct Leaf {
    pub i: usize,
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
                    next.push(Rc::new(Box::new(BDD::L(Leaf { label: table[i], i }))))
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
    /*
    fn reduce(&self) -> Rc<Box<BDD>> {
        // if there are more than two leaves then perform reduction 1
        // while possible do
        // begin
        //   if applicable then perform reduction 2
        //   if applicable then perform reduction 3
        // end
        // return
    }
    */
}

#[cfg(test)]
mod test {
    use super::BDD;

    #[test]
    fn new() {
        let bdd = BDD::new(3, &[false, false, false, true, true, true, true, true]);
        dbg!(bdd);
    }
}
