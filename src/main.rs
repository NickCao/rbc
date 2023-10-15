use boolean_expression::{BDDFunc, Expr, BDD};
use clap::Parser;
use rbc::qmc::{reduce, Imp, Tri};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Debug,
    io::Read,
    ops::Sub,
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
        let mut minterms = HashSet::new();
        let mut maxterms = vec![];

        for term in 0..2_usize.pow(graph.0.len() as u32) {
            let mut input = HashMap::<String, bool>::default();
            let mut imp = vec![];
            for i in 0..graph.0.len() {
                input.insert(graph.0[i].symbol.clone().unwrap(), ((term >> i) & 1) == 1);
                imp.push(if ((term >> i) & 1) == 1 {
                    Tri::T
                } else {
                    Tri::F
                });
            }
            let result = bdd.evaluate(*output, &input);
            if result {
                minterms.insert(Imp(imp));
            } else {
                maxterms.push(term);
            }
        }

        match args.command {
            1 => {
                // Return the design as a canonical SOP
                print!("{} = ", graph.1[i].symbol.clone().unwrap());
                for term in minterms {
                    print!("{} + ", term);
                }
                println!();
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

                println!("{} =", graph.1[i].symbol.clone().unwrap());

                let mut columns = minterms.clone();
                let mut rows = reduce(&columns);

                let mut chosen: HashSet<Imp> = HashSet::new();

                let mut fallback = false;

                loop {
                    let mut covered: HashSet<Imp> = HashSet::new();

                    for col in &columns {
                        let cover: Vec<_> = rows.iter().filter(|p| p.containes(col)).collect();
                        if cover.len() == 1 || fallback {
                            fallback = false;
                            println!("{} is ess", &cover[0]);
                            chosen.insert(cover[0].clone());
                            for col in &columns {
                                if cover[0].containes(col) {
                                    covered.insert(col.clone());
                                }
                            }
                        }
                    }

                    columns = columns.sub(&covered);

                    if columns.is_empty() {
                        break;
                    }

                    if covered.is_empty() {
                        fallback = true;
                    } else {
                        fallback = false;
                    }
                }
            }
            6 => {
                // Return a minimized number of literals representation in POS
                // Report on the number of saved literals vs. the canonical version
            }
            7 => {
                // Report the number of Prime Implicants
                println!(
                    "{} = {}",
                    graph.1[i].symbol.clone().unwrap(),
                    reduce(&minterms).len()
                );
            }
            8 => {
                // Report the number of Essential Prime Implicants
                let mut ess = 0;
                let prime = reduce(&minterms);
                for m in &minterms {
                    let rows: Vec<_> = prime.iter().filter(|p| p.containes(m)).collect();
                    if rows.len() == 1 {
                        ess += 1;
                    }
                }
                println!("{} = {}", graph.1[i].symbol.clone().unwrap(), ess);
            }
            9 => {
                // Report the number of ON-Set minterms
                println!(
                    "{} = {}",
                    graph.1[i].symbol.clone().unwrap(),
                    minterms.len()
                );
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
