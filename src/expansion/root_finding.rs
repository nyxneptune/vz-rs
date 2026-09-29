use super::{Matrix, finite, scalar_fields, source, value};

pub fn ancestors(parents: &[Option<usize>], i: usize) -> Vec<usize> {
    let mut out = vec![];
    let mut j = Some(i);
    while parents[j.unwrap()].is_some() {
        j = parents[j.unwrap()];
        out.push(j.unwrap());
    }
    out
}

pub fn refine(parents: &[Option<usize>], values: &[usize]) -> Vec<Option<usize>> {
    (0..values.len())
        .map(|i| {
            ancestors(parents, i)
                .into_iter()
                .find(|&p| values[p] < values[i])
        })
        .collect()
}

pub fn parents(values: &[usize]) -> Vec<Option<usize>> {
    (0..values.len())
        .map(|i| (0..i).rev().find(|&p| values[p] < values[i]))
        .collect()
}

pub fn descends_from(parents: &[Option<usize>], root: usize, i: usize) -> bool {
    i == root || ancestors(parents, i).contains(&root)
}

pub fn terminal_anchor(m: &Matrix) -> Option<usize> {
    (0..(m.len() - 1))
        .rev()
        .find(|&i| m[i].len() == 1 && m[i] < m[m.len() - 1])
}

pub fn borrowed(m: &Matrix, i: usize) -> bool {
    let cut = terminal_anchor(m).unwrap();
    let s = source(m, i);
    let fs = scalar_fields(&s);

    !fs.is_empty() &&
        fs.iter().all(|f| f.index == vec![vec![0]]) &&
        i + s.columns.len() == cut &&
        m[cut] == vec![s.head() + 2]
}

pub fn entry(m: &Matrix, i: usize, depth: usize) -> usize {
    if depth == 0 {
        return m[i][0];
    }
    let cut = terminal_anchor(m).unwrap();
    if i == cut {
        return m[cut + depth][1];
    }
    let fs = scalar_fields(&source(m, i));
    if borrowed(m, i) && depth > fs.len() {
        fs[fs.len() - 1].payload[0]
    } else {
        value(&fs, &finite(depth - 1))
    }
}

pub fn strong_parent(m: &Matrix, i: usize, depth: usize) -> Option<usize> {
    let cut = terminal_anchor(m).unwrap();
    let anchors = (0..=cut).filter(|i| m[*i].len() == 1);
    let candidates = if depth == 0 {
        anchors.rev().collect()
    } else {
        lineage(m, i, depth - 1)
    };
    candidates
        .into_iter()
        .find(|&p| p < i && entry(m, p, depth) < entry(m, i, depth))
}

pub fn lineage(m: &Matrix, i: usize, depth: usize) -> Vec<usize> {
    let mut j = Some(i);
    let mut out = vec![];
    j = strong_parent(m, j.unwrap(), depth);
    while j.is_some() {
        out.push(j.unwrap());
        j = strong_parent(m, j.unwrap(), depth);
    }
    out
}
