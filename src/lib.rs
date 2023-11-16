pub mod aag;
pub mod aig;
pub mod expr;
pub mod qmc;

use pyo3::prelude::*;
use qmc::{reduce, Imp, Tri};
use std::collections::{HashMap, HashSet};
use std::ops::Sub;

#[pyfunction]
fn minimize(minterms: Vec<usize>, variables: HashMap<String, usize>) -> PyResult<String> {
    let n = variables.len();
    let mut m = HashSet::new();
    for term in minterms {
        let mut imp = vec![];
        for i in 0..n {
            imp.push(if ((term >> i) & 1) == 1 {
                Tri::T
            } else {
                Tri::F
            });
        }
        m.insert(Imp(imp));
    }
    let mut columns = m.clone();
    let rows = reduce(&m);
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

    let mut expr = vec![];
    for x in &chosen {
        let mut comp = vec![];
        for (i, y) in x.0.iter().enumerate() {
            let v = variables.iter().find(|(k, v)| **v == i).unwrap().0;
            match y {
                Tri::T => comp.push(v.to_string()),
                Tri::F => comp.push("~".to_string() + v),
                Tri::X => (),
            }
        }
        expr.push(comp.join("*"))
    }

    Ok(expr.join("+"))
}

#[pymodule]
fn rbc(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(minimize, m)?)?;
    Ok(())
}
