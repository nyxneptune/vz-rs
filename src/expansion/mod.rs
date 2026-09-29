use std::collections::{HashMap, HashSet};

use super::{Matrix, copy_n_times, diagonal, limit};

mod root_finding;
use root_finding::{borrowed, descends_from, entry, lineage, parents, refine, strong_parent,
                   terminal_anchor};

mod blocks;
use blocks::{Block, blocks, boundary, shift, source};

mod fields;
use fields::{Field, emit, emit_intervals, field, finite, intervals, length, predecessor,
             scalar_fields, split, value};

fn scalar_cut(m: &Matrix, b: &[Block], n: usize) -> Matrix {
    let fields: Vec<_> = b.iter().map(scalar_fields).collect();
    let end = intervals(fields.last().unwrap()).last().unwrap().1.clone();
    let cut = predecessor(&end);
    let initial = parents(&b.iter().map(Block::head).collect::<Vec<_>>());
    let mut unsorted_points = HashSet::new();

    unsorted_points.insert(vec![]);
    unsorted_points.insert(cut.clone());

    for f in &fields {
        for (a, b, _) in split(f, cut.clone()).0 {
            unsorted_points.insert(a);
            unsorted_points.insert(b);
        }
    }

    let mut points: Vec<_> = unsorted_points.into_iter().collect();
    points.sort();

    let mut parents = initial.clone();
    let mut history = HashMap::new();

    for point in &points {
        parents = refine(
            &parents,
            &fields.iter().map(|f| value(f, point)).collect::<Vec<_>>()
        );
        history.insert(point, parents.clone());
    }

    let root = history[&cut]
        .last()
        .unwrap()
        .expect("the last scalar position has no surviving parent");
    let mut result: Matrix = Vec::from(&m[..b[root].start]);
    for k in 0..n {
        for i in root..(b.len() - 1) {
            let block = &b[i];
            if k == 0 {
                result.extend(block.columns.clone());
                continue;
            }
            let mut head = block.head();
            if descends_from(&initial, root, i) {
                head += k * (b[b.len() - 1].head() - b[root].head());
            }
            let mut changed = vec![];
            for (a, b) in points.iter().zip(points.iter().skip(1)) {
                let mut v = value(&fields[i], a);
                if *a < cut && descends_from(&history[a], root, i) {
                    v += k * (value(&fields[fields.len() - 1], a) - value(&fields[root], a));
                }
                changed.push((a.clone(), b.clone(), v));
            }
            let suffix = emit(&split(&fields[i], cut.clone()).1);
            let prefix = if i == root && k == 1 {
                let f = &fields[fields.len() - 1];
                emit(&Vec::from(&f[..f.len() - 1]))
            } else {
                emit_intervals(changed)
            };
            let a = shift(&([prefix, suffix].concat()), head, 0);
            result.extend(vec![vec![head]]);
            result.extend(a);
        }
    }
    result
}

fn unit_extension(m: &Matrix, n: usize) -> Option<Matrix> {
    if !(m.len() >= 3 &&
        m[0] == vec![0] &&
        m[1] == vec![1] &&
        m[m.len() - 1].len() == 2 &&
        m[m.len() - 1][1] == 1 &&
        m.iter()
            .all(|c| c.len() == 1 || (c.len() == 2 && c[1] == 1)))
    {
        return None;
    }
    let mut out = if n == 0 { vec![] } else { vec![vec![0]] };
    out.extend(
        (0..(n.saturating_sub(1)))
            .map(|k| {
                shift(
                    &Vec::from(&m[1..m.len() - 1]),
                    (m[m.len() - 1][0] - 1) * k,
                    k
                )
            })
            .collect::<Vec<Matrix>>()
            .concat()
    );
    Some(out)
}

fn exposed_cut(m: &Matrix, n: usize) -> Option<Matrix> {
    let cut = terminal_anchor(m)?;
    if m.iter()
        .skip(cut + 1)
        .any(|c| c.len() != 2 || c[0] != m[cut][0] + 1)
    {
        return None;
    }
    let active = (0..(m.len() - cut))
        .filter(|&d| strong_parent(m, cut, d).is_some())
        .max()
        .expect("the exposed front has no surviving parent");
    let root = strong_parent(m, cut, active).unwrap();
    let retained = usize::from(
        blocks(&Vec::from(&m[..cut]))
            .into_iter()
            .all(|b| b.start != root)
    );

    let mut out = Vec::from(&m[..root]);
    for k in 0..(n + retained) {
        for block in blocks(&Vec::from(&m[root..cut])) {
            let i = root + block.start;
            if k == 0 {
                out.extend(block.columns);
                continue;
            }
            let mut head = block.head();
            let mut body = scalar_fields(&source(m, i));
            if borrowed(m, i) && active > body.len() {
                let n = body.len() - 1;
                body[n] = field(body[n].payload[0], length(&limit(2)));
            }
            if active > 0 && (i == root || lineage(m, i, 0).contains(&root)) {
                head += k * (entry(m, cut, 0) - entry(m, root, 0));
            }
            for j in 0..(active.saturating_sub(1)) {
                let amount = if i == root || lineage(m, i, j + 1).contains(&root) {
                    k * (entry(m, cut, j + 1) - entry(m, root, j + 1))
                } else {
                    0
                };
                if j == body.len() {
                    body.push(Field {
                        payload: vec![0],
                        index:   vec![vec![0]],
                        rooted:  false,
                        columns: vec![vec![1, 0]]
                    });
                }
                let old = &body[j];
                body[j] = field(old.payload[0] + amount, length(&old.index));
            }
            while body.last().map(|x| x.payload.clone()) == Some(vec![0]) {
                body.pop();
            }
            out.push(vec![head]);
            out.extend(shift(&emit(&body), head, 0));
        }
    }
    Some(out)
}

fn row_cut(m: &Matrix, n: usize) -> Matrix {
    let mut history = vec![parents(&m.iter().map(|c| c[0]).collect::<Vec<_>>())];

    for depth in 1..(m[m.len() - 1].len()) {
        let values = m
            .iter()
            .map(|c| if depth < c.len() { c[depth] } else { 0 })
            .collect::<Vec<_>>();
        history.push(refine(&history[history.len() - 1], &values));
    }
    let active = history
        .iter()
        .enumerate()
        .filter(|(_, p)| p.last().unwrap().is_some())
        .map(|(d, _)| d)
        .max()
        .expect("the terminal column has no surviving parent");
    let root = history[active].last().unwrap().unwrap();
    let mut out: Matrix = Vec::from(&m[..root]);

    for k in 0..n {
        for i in root..(m.len() - 1) {
            let mut values = m[i].clone();
            values.extend(vec![0; active.saturating_sub(m[i].len())]);
            for depth in 0..active {
                if descends_from(&history[depth], root, i) {
                    let base = if depth < m[root].len() {
                        m[root][depth]
                    } else {
                        0
                    };
                    values[depth] += k * (m[m.len() - 1][depth] - base);
                }
            }
            while values.len() > 1 && values[values.len() - 1] == 0 {
                values.pop();
            }
            out.push(values);
        }
    }

    out
}

fn candidate(m: &Matrix, n: usize) -> (Matrix, &'static str) {
    if m.is_empty() {
        return (vec![], "zero");
    } else if m[m.len() - 1] == vec![0] {
        return (Vec::from(&m[..m.len() - 1]), "succ");
    } else if m[m.len() - 1].len() == 1 {
        let root = *(parents(&m.iter().map(|x| x[0]).collect::<Vec<_>>())
            .last()
            .unwrap());
        if let Some(r) = root {
            let mut out = Vec::from(&m[..r]);
            out.extend(copy_n_times(&Vec::from(&m[r..(m.len() - 1)]), n));
            return (out, "prss");
        }
        panic!("last column is prss but has no parent");
    } else if *m == limit(m.len()) && m.len() >= 3 {
        let mut result = Vec::from(if n == 0 {
            &m[..m.len() - 3]
        } else {
            &m[..m.len() - 2]
        });
        if n > 1 {
            result.extend(
                (0..(n - 1))
                    .map(|k| m[m.len() - 2].iter().map(|&v| v + k).collect())
                    .collect::<Matrix>()
            );
        }
        return (result, "dbms");
    } else if m.len() >= 3 &&
        m[m.len() - 2].len() == 1 &&
        m[m.len() - 1] == vec![m[m.len() - 2][0] + 1, 1] &&
        m[m.len() - 3] == vec![m[m.len() - 2][0] - 1]
    {
        let mut result = Vec::from(&m[..m.len() - 3]);
        result.extend((0..n).map(|k| vec![m[m.len() - 3][0] + k]));
        return (result, "eps0");
    }

    let mut height = 0;
    while height < m.len() && m[height] == diagonal(height) {
        height += 1;
    }
    assert!(height != 0, "first entry of m is nonzero!");
    height -= 1;
    if height >= 3 && m[(height + 1)..].iter().all(|x| *x == diagonal(height)) {
        let mut result = if n > 0 {
            Vec::from(&m[..(height - 1)])
        } else {
            vec![]
        };
        if n > 1 {
            for k in 0..(n - 1) {
                result.extend(
                    m[(height - 1)..(m.len() - 1)]
                        .iter()
                        .map(|c| c.iter().map(|v| v + k).collect())
                );
            }
        }
        return (result, "dbms full-ascend");
    }

    let b = blocks(m);
    let terminal = &b[b.len() - 1];
    if terminal.columns.len() > 1 &&
        m[m.len() - 1][0] == terminal.head() + 1 &&
        m[m.len() - 1].len() == 2
    {
        let boundary = boundary(m, &b, n);
        return boundary.map_or_else(
            || (scalar_cut(m, &b, n), "scalar cut"),
            |v| (v, "protected boundary")
        );
    }
    let cut = terminal_anchor(m).unwrap();
    let unit = unit_extension(m, n);
    let inner_unit = b.iter().all(|bl| cut != bl.start) &&
        m.len() > cut + 2 &&
        m.iter().skip(cut + 1).all(|v| *v == vec![m[cut][0] + 1, 1]);
    if inner_unit && let Some(u) = unit {
        return (u, "unit extension");
    }
    let exposed = exposed_cut(m, n);
    if let Some(e) = exposed {
        return (e, "exposed cut");
    }
    unit.map_or_else(|| (row_cut(m, n), "row cut"), |u| (u, "unit extension"))
}

#[must_use]
pub fn expand_with_rule(m: &Matrix, n: usize) -> (Matrix, &'static str) {
    let (result, rule) = candidate(m, n);
    assert!(
        m.is_empty() || result < *m,
        "{rule} at n = {n}: candidate does not descend; candidate = {m:?} -> {result:?}"
    );
    (result, rule)
}

#[must_use]
pub fn expand(m: &Matrix, n: usize) -> Matrix {
    let (e, _) = expand_with_rule(m, n);
    e
}
