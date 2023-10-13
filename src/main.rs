use clap::Parser;
use std::{
    collections::{HashMap, VecDeque},
    fmt::Debug,
    io::Read,
    rc::Rc,
};

/// RBC: System for Combinational Logic Synthesis
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {}

struct Input {
    symbol: String,
}

impl Debug for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.symbol))
    }
}

struct Negate {
    rhs: Rc<Node>,
}

struct Identity {
    rhs: Rc<Node>,
}

impl Debug for Identity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.rhs))
    }
}

impl Debug for Negate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("!{:?}", self.rhs))
    }
}

struct And {
    rhs0: Rc<Node>,
    rhs1: Rc<Node>,
}

impl Debug for And {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({:?} & {:?})", self.rhs0, self.rhs1))
    }
}

enum Node {
    I(Input),
    N(Negate),
    A(And),
    D(Identity),
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::I(i) => i.fmt(f),
            Node::N(i) => i.fmt(f),
            Node::A(i) => i.fmt(f),
            Node::D(i) => i.fmt(f),
        }
    }
}

impl Node {
    fn eval(&self, inputs: &HashMap<String, usize>) -> usize {
        match self {
            Node::I(Input { symbol }) => *inputs.get(symbol).unwrap(),
            Node::N(Negate { rhs }) => rhs.eval(inputs) ^ 1,
            Node::A(And { rhs0, rhs1 }) => rhs0.eval(inputs) & rhs1.eval(inputs),
            Node::D(Identity { rhs }) => rhs.eval(inputs),
        }
    }
}

fn main() {
    let _args = Args::parse();

    let mut buf = vec![];
    std::io::stdin().read_to_end(&mut buf).unwrap();

    let graph = rbc::de::aag(&buf).unwrap();

    let mut state = HashMap::<usize, Rc<Node>>::new();

    for input in &graph.0 {
        state.insert(
            input.variable,
            Node::I(Input {
                symbol: input.symbol.clone().unwrap(),
            })
            .into(),
        );
    }

    let mut rem: VecDeque<rbc::de::A> = graph.2.clone().into();

    while rem.len() > 0 {
        let cur = rem.pop_back().unwrap();
        if let (Some(rhs0), Some(rhs1)) = (state.get(&cur.rhs0), state.get(&cur.rhs1)) {
            state.insert(
                cur.lhs,
                Node::A(And {
                    rhs0: if cur.rhs0_negate == 0 {
                        rhs0.clone()
                    } else {
                        Node::N(Negate { rhs: rhs0.clone() }).into()
                    },
                    rhs1: if cur.rhs1_negate == 0 {
                        rhs1.clone()
                    } else {
                        Node::N(Negate { rhs: rhs1.clone() }).into()
                    },
                })
                .into(),
            );
        } else {
            rem.push_front(cur);
        }
    }


    let mut outputs: Vec<Rc<Node>> = vec![];

    for output in &graph.1 {
        let node = state.get(&output.variable).unwrap();
        outputs.push(if output.negate == 0 {
            Node::D(Identity { rhs: node.clone() }).into()
        } else {
            Node::N(Negate { rhs: node.clone() }).into()
        });
    }

    dbg!(&outputs);

    for (x, y) in [(0, 0), (0, 1), (1, 0), (1, 1)] {
        print!("{}{} ", x, y);
        for output in &outputs {
            let value = output.eval(&HashMap::from([("x".to_string(), x), ("y".to_string(), y)]));
            print!("{}", value);
        }
        println!();
    }
}
