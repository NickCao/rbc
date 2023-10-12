use clap::Parser;
use std::{
    collections::{HashMap, VecDeque},
    io::Read,
    rc::Rc,
};

/// RBC: System for Combinational Logic Synthesis
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {}

#[derive(Debug)]
struct Input {
    symbol: String,
}

#[derive(Debug)]
struct Negate {
    rhs: Rc<Node>,
}

#[derive(Debug)]
struct And {
    rhs0: Rc<Node>,
    rhs1: Rc<Node>,
}

#[derive(Debug)]
enum Node {
    I(Input),
    N(Negate),
    A(And),
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

    dbg!(state);
}
