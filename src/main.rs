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

    for (i, output) in outputs.iter().enumerate() {
        let mut minterms = vec![];
        let mut maxterms = vec![];

        for term in 0..2_usize.pow(graph.0.len() as u32) {
            let mut input = HashMap::<String, bool>::default();
            for i in 0..graph.0.len() {
                input.insert(graph.0[i].symbol.clone().unwrap(), ((term >> i) & 1) == 1);
            }
            let result = bdd.evaluate(*output, &input);
            if result {
                minterms.push(term);
            } else {
                maxterms.push(term);
            }
        }

        match args.command {
            1 => {
                // Return the design as a canonical SOP
                println!("{} = {:?}", graph.1[i].symbol.clone().unwrap(), minterms);
            }
            2 => {
                // Return the design as a canonical POS
                println!("{} = {:?}", graph.1[i].symbol.clone().unwrap(), maxterms);
            }
            3 => {
                // Return the design INVERSE as a canonical SOP
                println!("{} = {:?}", graph.1[i].symbol.clone().unwrap(), maxterms);
            }
            4 => {
                // Return the design INVERSE as a canonical POS
                println!("{} = {:?}", graph.1[i].symbol.clone().unwrap(), maxterms);
            }
            5 => {
                // Return a minimized number of literals representation in SOP
                // Report on the number of saved literals vs. the canonical version
            }
            6 => {
                // Return a minimized number of literals representation in POS
                // Report on the number of saved literals vs. the canonical version
            }
            7 => {
                // Report the number of Prime Implicants
            }
            8 => {
                // Report the number of Essential Prime Implicants
            }
            9 => {
                // Report the number of ON-Set minterms
            }
            10 => {
                // Report the number of ON-Set maxterms
            }
            11 => {
                // Command of your choice #1
            }
            12 => {
                // Command of your choice #2
            }
            _ => unimplemented!(),
        }
    }
}
