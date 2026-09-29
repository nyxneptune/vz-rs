use vz_rs::codec::{format, parse};
use vz_rs::expand;

fn main() {
    let mut m = parse("0 1 21 321 4321");
    for i in 0..150 {
        m = expand(&m, 3 + i);
        // Truncate to avoid formatter dying
        if let Some((j, _)) = m
            .iter()
            .enumerate()
            .find(|(_, c)| *c.iter().max().unwrap() >= 36)
        {
            m = Vec::from(&m[..j]);
        }
    }
}
