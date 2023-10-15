#[derive(Debug, Clone)]
pub struct Term(pub usize);

#[derive(Debug, Clone)]
pub struct Neg(pub Box<Node>);

#[derive(Debug, Clone)]
pub struct And(pub Box<Node>, pub Box<Node>);

#[derive(Debug, Clone)]
pub enum Node {
    T(Term),
    N(Neg),
    A(And),
}

impl Node {
    pub fn eval(&self, inputs: &[bool]) -> bool {
        match self {
            Node::T(Term(i)) => inputs[*i],
            Node::N(Neg(r)) => !r.eval(inputs),
            Node::A(And(l, r)) => l.eval(inputs) & r.eval(inputs),
        }
    }
}
