use clap::Parser;
use rbc::qmc::{reduce, Imp, ImpMax, Tri};
use std::{collections::HashSet, fmt::Debug, ops::Sub};

/// RBC: System for Combinational Logic Synthesis
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, short)]
    command: usize,

    #[arg(long, short)]
    expression: bool,

    file: String,
}

fn main() {
    let args = Args::parse();

    let buf = std::fs::read(args.file).unwrap();

    let (inputs, outputs) = if !args.expression {
        rbc::aag::parse(&buf).unwrap()
    } else {
        let e: Box<rbc::aig::AIG> = rbc::expr::calculator1::ExprParser::new()
            .parse(&String::from_utf8(buf).unwrap())
            .unwrap()
            .into();
        (e.syms(), vec![e])
    };

    for (i, output) in outputs.iter().enumerate() {
        let mut minterms = HashSet::new();
        let mut maxterms = HashSet::new();
        let mut truthtab = vec![];

        for term in 0..2_usize.pow(inputs as u32) {
            let mut input = vec![];
            let mut imp = vec![];
            for i in 0..inputs {
                input.push(((term >> i) & 1) == 1);
                imp.push(if ((term >> i) & 1) == 1 {
                    Tri::T
                } else {
                    Tri::F
                });
            }
            let result = output.eval(&input);
            truthtab.push((input, result));
            if result {
                minterms.insert(Imp(imp));
            } else {
                maxterms.insert(Imp(imp));
            }
        }

        match args.command {
            1 => {
                // Return the design as a canonical SOP
                println!(
                    "canonical SOP of output {}: {}",
                    i,
                    minterms
                        .iter()
                        .map(Imp::to_string)
                        .collect::<Vec<_>>()
                        .join(" + ")
                );
            }
            2 => {
                // Return the design as a canonical POS
                println!(
                    "canonical POS of output {}: {}",
                    i,
                    maxterms
                        .clone()
                        .into_iter()
                        .map(ImpMax::from)
                        .collect::<Vec<_>>()
                        .iter()
                        .map(ImpMax::to_string)
                        .collect::<Vec<_>>()
                        .join("")
                );
            }
            3 => {
                // Return the design INVERSE as a canonical SOP
                println!(
                    "canonical SOP of output {} INVERSE: {}",
                    i,
                    maxterms
                        .iter()
                        .map(Imp::to_string)
                        .collect::<Vec<_>>()
                        .join(" + ")
                );
            }
            4 => {
                // Return the design INVERSE as a canonical POS
                println!(
                    "canonical POS of output {} INVERSE: {}",
                    i,
                    minterms
                        .clone()
                        .into_iter()
                        .map(ImpMax::from)
                        .collect::<Vec<_>>()
                        .iter()
                        .map(ImpMax::to_string)
                        .collect::<Vec<_>>()
                        .join("")
                );
            }
            5 | 11 => {
                // Return a minimized number of literals representation in SOP
                // Report on the number of saved literals vs. the canonical version

                let mut columns = minterms.clone();
                let rows = reduce(&columns);

                let mut chosen: HashSet<Imp> = HashSet::new();

                let mut fallback = false;

                loop {
                    let mut covered: HashSet<Imp> = HashSet::new();

                    for col in &columns {
                        let cover: Vec<_> = rows.iter().filter(|p| p.containes(col)).collect();
                        if cover.len() == 1 || fallback {
                            fallback = false;
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

                    fallback = covered.is_empty();
                }

                if args.command == 5 {
                    println!(
                        "minimized SOP of output {}: {}, saved {} literals",
                        i,
                        chosen
                            .iter()
                            .map(Imp::to_string)
                            .collect::<Vec<_>>()
                            .join(" + "),
                        minterms.iter().map(Imp::literals).sum::<usize>()
                            - chosen.iter().map(Imp::literals).sum::<usize>()
                    );
                } else {
                    for term in chosen {
                        println!(
                            "{} {}",
                            term,
                            (0..outputs.len())
                                .map(|j| if i == j { "1" } else { "0" })
                                .collect::<Vec<_>>()
                                .join("")
                        );
                    }
                }
            }
            6 => {
                // Return a minimized number of literals representation in POS
                // Report on the number of saved literals vs. the canonical version

                let mut columns = maxterms.clone();
                let rows = reduce(&columns);

                let mut chosen: HashSet<Imp> = HashSet::new();

                let mut fallback = false;

                loop {
                    let mut covered: HashSet<Imp> = HashSet::new();

                    for col in &columns {
                        let cover: Vec<_> = rows.iter().filter(|p| p.containes(col)).collect();
                        if cover.len() == 1 || fallback {
                            fallback = false;
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

                    fallback = covered.is_empty();
                }

                println!(
                    "minimized POS of output {}: {}, saved {} literals",
                    i,
                    chosen
                        .clone()
                        .into_iter()
                        .map(ImpMax::from)
                        .collect::<Vec<_>>()
                        .iter()
                        .map(ImpMax::to_string)
                        .collect::<Vec<_>>()
                        .join(" + "),
                    maxterms.iter().map(Imp::literals).sum::<usize>()
                        - chosen.iter().map(Imp::literals).sum::<usize>()
                );
            }
            7 => {
                // Report the number of Prime Implicants
                println!(
                    "number of Prime Implicants of output {}: {}",
                    i,
                    reduce(&minterms).len()
                );
            }
            8 => {
                // Report the number of Essential Prime Implicants
                let mut ess = 0;
                let prime = reduce(&minterms);
                for m in &minterms {
                    if prime.iter().filter(|p| p.containes(m)).count() == 1 {
                        ess += 1;
                    }
                }
                println!(
                    "number of Essential Prime Implicants of output {}: {}",
                    i, ess
                );
            }
            9 => {
                // Report the number of ON-Set minterms
                println!(
                    "number of ON-Set minterms of output {}: {}",
                    i,
                    minterms.len()
                );
            }
            10 => {
                // Report the number of ON-Set maxterms
                println!(
                    "number of ON-Set maxterms of output {}: {}",
                    i,
                    maxterms.len()
                );
            }
            12 => {
                // Command of your choice #2
                println!("AIG of output {}: {}", i, output);
            }
            _ => unimplemented!(),
        }
    }
}
