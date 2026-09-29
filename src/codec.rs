use super::Matrix;

#[must_use]
pub fn parse(s: &str) -> Matrix {
    s.split(' ')
        .map(|v| {
            v.chars()
                .map(|v| {
                    v.to_digit(36)
                        .unwrap_or_else(|| panic!("Unknown character {v} in string {s}"))
                        as usize
                })
                .collect()
        })
        .collect()
}

pub fn format(m: &Matrix) -> String {
    m.iter()
        .map(|c| {
            c.iter()
                .map(|n| {
                    char::from_digit(u32::try_from(*n).unwrap(), 36)
                        .unwrap_or_else(|| panic!("Number {n} at least 36 in matrix {m:?}"))
                })
                .collect::<String>()
        })
        .intersperse(String::from(" "))
        .collect()
}
