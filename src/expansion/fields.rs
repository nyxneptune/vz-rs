use std::cmp::min;
use std::collections::VecDeque;

use super::blocks::Block;
use crate::{Column, Matrix, copy_n_times, diagonal, limit};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Field {
    pub payload: Column,
    pub index:   Matrix,
    pub rooted:  bool,
    pub columns: Matrix
}

impl Field {
    const fn new(payload: Column, index: Matrix, rooted: bool, columns: Matrix) -> Self {
        Self {
            payload,
            index,
            rooted,
            columns
        }
    }
}

pub fn fields(block: &Block) -> Vec<Field> {
    assert!(
        block.columns[0].len() == 1,
        "a structured block root has no scalar profile"
    );
    let mut out = vec![];
    for c in &block.columns[1..] {
        assert!(c[0] > block.head(), "profile column is below its anchor");
        let mut cp = c.clone();
        cp[0] -= block.head();
        if c.len() > 1 {
            out.push(Field::new(
                Vec::from(&cp[1..]),
                limit(cp[0]),
                cp[0] > 1,
                vec![cp]
            ));
        } else if !out.is_empty() {
            let mut old = out.pop().unwrap();
            old.index.push(vec![cp[0] - 1]);
            old.columns.push(cp);
            out.push(old);
        } else {
            panic!("bare repetition marker has no seed");
        }
    }

    out
}

type Position = Vec<(Matrix, usize)>;

fn add(left: &Position, right: &Position) -> Position {
    let mut terms = left.clone();
    for (principal, count) in right {
        while terms.last().is_some_and(|x| x.0 < *principal) {
            terms.pop();
        }
        let count_copy = if terms.last().is_some_and(|x| x.0 == *principal) {
            *count + terms.pop().unwrap().1
        } else {
            *count
        };
        terms.push((principal.clone(), count_copy));
    }
    terms
}

pub fn length(index: &Matrix) -> Position {
    let mut result = vec![];
    let mut component = vec![];

    for c in index {
        if *c == vec![0] && !component.is_empty() {
            result = add(&result, &vec![(component, 1)]);
            component = vec![];
        }
        assert!(
            *c == vec![0] || !component.is_empty(),
            "a constant field's index must begin with zero"
        );
        component.push(c.clone());
    }
    if component.is_empty() {
        result
    } else {
        add(&result, &vec![(component, 1)])
    }
}

pub fn field(value: usize, length: Position) -> Field {
    assert!(value != 0, "the computed profile needs an interior zero");
    let index = length
        .into_iter()
        .map(|(name, count)| copy_n_times(&name, count))
        .collect::<Vec<Matrix>>()
        .concat();
    let mut columns = vec![];
    for c in &index {
        if *c == vec![0] {
            columns.push(vec![1, value]);
        } else if c.len() == 1 {
            columns.push(vec![c[0] + 1]);
        } else if *c == diagonal(c[0]) {
            columns.push(vec![c[0] + 1, value]);
        } else {
            panic!("no raw spelling specified for this cut remainder");
        }
    }
    Field::new(vec![value], index, false, columns)
}

pub fn scalar_fields(block: &Block) -> Vec<Field> {
    let mut result = vec![];
    for f in fields(block) {
        assert!(f.payload.len() == 1, "this cut enters a structured payload");
        if f.rooted && result.last().map(|x: &Field| x.index.clone()) == Some(vec![vec![0]]) {
            let n = result.len();
            result[n - 1] = field(result[result.len() - 1].payload[0], length(&f.index));
        }
        result.push(f);
    }
    result
}

pub fn intervals(fields: &Vec<Field>) -> Vec<(Position, Position, usize)> {
    let mut out = vec![];
    let mut start: Position = vec![];

    for field in fields {
        let end = add(&start, &length(&field.index));
        out.push((start, end.clone(), field.payload[0]));
        start = end;
    }
    out
}

pub fn finite(n: usize) -> Position {
    if n == 0 {
        vec![]
    } else {
        vec![(vec![vec![0]], n)]
    }
}

pub fn predecessor(position: &Position) -> Position {
    assert!(
        position.last().map(|x| x.0.clone()) == Some(vec![vec![0]]),
        "the selected profile has no last scalar position"
    );
    let mut pos_copy = position.clone();
    let x = pos_copy.pop().unwrap();
    pos_copy.extend(finite(x.1 - 1));
    pos_copy
}

fn after(total: Position, prefix: Position) -> Position {
    assert!(prefix <= total, "cut lies beyond the constant field");
    let mut left = VecDeque::from(prefix);
    let mut right = VecDeque::from(total);
    while !left.is_empty() && !right.is_empty() && left.front() == right.front() {
        left.pop_front();
        right.pop_front();
    }
    if !left.is_empty() &&
        !right.is_empty() &&
        left.front().map(|x| x.0.clone()) == right.front().map(|x| x.0.clone())
    {
        right[0].1 -= left[0].1;
    }
    Vec::from(right)
}

pub fn split(
    fields: &[Field], position: Position
) -> (Vec<(Position, Position, usize)>, Vec<Field>) {
    let mut start = vec![];
    let mut prefix = vec![];

    for i in 0..fields.len() {
        if position <= start {
            return (prefix, Vec::from(&fields[i..]));
        }
        let end = add(&start, &length(&fields[i].index));
        prefix.push((
            start.clone(),
            min(position.clone(), end.clone()),
            fields[i].payload[0]
        ));
        if position < end {
            let mut suffix = vec![field(fields[i].payload[0], after(end, position))];
            suffix.extend(Vec::from(&fields[(i + 1)..]));
            return (prefix, suffix);
        }
        start = end;
    }
    if start < position {
        prefix.push((start, position, 0));
    }
    (prefix, vec![])
}

pub fn value(fields: &Vec<Field>, position: &Position) -> usize {
    intervals(fields)
        .into_iter()
        .filter(|(a, b, _)| *a <= *position && *position < *b)
        .map(|(_, _, v)| v)
        .next()
        .unwrap_or(0)
}

pub fn emit(fields: &[Field]) -> Matrix {
    fields
        .iter()
        .map(|f| f.columns.clone())
        .collect::<Vec<Matrix>>()
        .concat()
}

pub fn emit_intervals(intervals: Vec<(Position, Position, usize)>) -> Matrix {
    let mut merged: Vec<(Position, Position, usize)> = vec![];
    for (a, b, value) in intervals {
        if merged.last().map(|x| (&x.1, &x.2)) == Some((&a, &value)) {
            let n = merged.len() - 1;
            merged[n] = (merged[merged.len() - 1].0.clone(), b, value);
        } else {
            merged.push((a, b, value));
        }
    }
    while merged.last().map(|x| x.2) == Some(0) {
        merged.pop();
    }
    emit(
        &merged
            .into_iter()
            .filter(|(a, b, _)| a < b)
            .map(|(a, b, v)| field(v, after(b, a)))
            .collect::<Vec<Field>>()
    )
}
