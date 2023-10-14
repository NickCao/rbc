use boolean_expression::{BDDFunc, Expr, BDD};
use clap::Parser;
use std::{
    collections::{HashMap, VecDeque},
    fmt::Debug,
    io::Read,
};

/// RBC: System for Combinational Logic Synthesis
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, short)]
    command: usize,
}

fn print_expr(expr: &Expr<String>) -> String {
    match expr {
        Expr::Terminal(t) => t.clone(),
        Expr::Const(b) => format!("{}", b),
        Expr::Not(n) => format!("{}'", print_expr(n)),
        Expr::And(a, b) => format!("{}{}", print_expr(a), print_expr(b)),
        Expr::Or(a, b) => format!("{}+{}", print_expr(a), print_expr(b)),
    }
}

fn main() {
    let args = Args::parse();

    let mut buf = vec![];
    std::io::stdin().read_to_end(&mut buf).unwrap();

    let graph = rbc::de::aag(&buf).unwrap();

    let mut bdd: BDD<String> = boolean_expression::BDD::new();

    let mut state = HashMap::<usize, BDDFunc>::new();

    for input in graph.0.iter() {
        state.insert(input.variable, bdd.terminal(input.symbol.clone().unwrap()));
    }

    let mut rem: VecDeque<rbc::de::A> = graph.2.clone().into();

    while !rem.is_empty() {
        let cur = rem.pop_back().unwrap();
        if let (Some(rhs0), Some(rhs1)) = (state.get(&cur.rhs0), state.get(&cur.rhs1)) {
            let r0 = if cur.rhs0_negate == 0 {
                *rhs0
            } else {
                bdd.not(*rhs0)
            };

            let r1 = if cur.rhs1_negate == 0 {
                *rhs1
            } else {
                bdd.not(*rhs1)
            };
            state.insert(cur.lhs, bdd.and(r0, r1));
        } else {
            rem.push_front(cur);
        }
    }

    let mut outputs: Vec<BDDFunc> = vec![];

    for output in &graph.1 {
        let node = state.get(&output.variable).unwrap();
        outputs.push(if output.negate == 0 {
            *node
        } else {
            bdd.not(*node)
        });
    }

    for output in outputs {
        println!("{}", print_expr(&bdd.to_expr(output)));
    }

    match args.command {
        _ => unimplemented!(),
    }
}
