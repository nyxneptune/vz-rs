use super::Matrix;
use super::fields::fields;
use super::root_finding::terminal_anchor;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Block {
    pub start:   usize,
    pub columns: Matrix
}

impl Block {
    const fn new(start: usize, columns: Matrix) -> Self {
        Self {
            start,
            columns
        }
    }

    pub fn head(&self) -> usize {
        self.columns[0][0]
    }
}

pub fn blocks(m: &Matrix) -> Vec<Block> {
    let mut out = vec![];
    let mut start = 0;

    for i in 0..m.len() {
        if i == start {
            if m[i].len() > 1 {
                out.push(Block::new(i, vec![m[i].clone()]));
                start = i + 1;
            }
            continue;
        }
        let head = m[start][0];
        if (m[i].len() == 1 && (m[i][0] <= head + 1 || m[i - 1].len() >= 3)) ||
            (m[i].len() > 1 && m[i][0] <= head)
        {
            out.push(Block::new(start, Vec::from(&m[start..i])));
            start = i;
            if m[i].len() > 1 {
                out.push(Block::new(i, vec![m[i].clone()]));
                start = i + 1;
            }
        }
    }

    if start < m.len() {
        out.push(Block::new(start, Vec::from(&m[start..])));
    }
    out
}

pub fn shift(m: &Matrix, top: usize, lower: usize) -> Matrix {
    m.iter()
        .map(|c| {
            let mut cp = c.clone();
            cp[0] += top;
            if cp.len() > 1 {
                cp[1] += lower;
            }
            cp
        })
        .collect()
}

pub fn boundary(m: &Matrix, b: &[Block], n: usize) -> Option<Matrix> {
    if b.len() < 2 {
        return None;
    }
    let parent = &b[b.len() - 2];
    let terminal = &b[b.len() - 1];
    if terminal.head() != parent.head() + 1 ||
        *terminal.columns.last().unwrap() != vec![terminal.head() + 1, 1] ||
        parent.columns[0].len() != 1
    {
        return None;
    }

    let mut f = fields(terminal);
    f = Vec::from(&f[..f.len() - 1]);
    let earlier = fields(parent);

    if f.is_empty() ||
        !f.iter().chain(earlier.iter()).all(|x| {
            if x.payload.len() > 1 {
                x.payload[0] >= 1
            } else {
                x.payload == vec![1]
            }
        }) ||
        !f.iter().any(|x| x.payload.len() > 1 || x.rooted)
    {
        return None;
    }

    let mut seed = terminal.columns.clone();
    seed.pop();
    let mut result = Vec::from(&m[..parent.start]);
    let mut block;
    let copies = if parent.columns.len() == 1 {
        result.extend(if n > 0 {
            parent.columns.clone()
        } else {
            vec![]
        });
        block = seed;
        if n > 0 { n - 1 } else { 0 }
    } else {
        block = parent.columns.clone();
        block.extend(seed);
        n
    };

    for k in 0..copies {
        result.extend(shift(&block, k, k));
    }
    Some(result)
}

pub fn source(m: &Matrix, i: usize) -> Block {
    let cut = terminal_anchor(m).unwrap();
    blocks(&Vec::from(&m[i..cut]))[0].clone()
}
