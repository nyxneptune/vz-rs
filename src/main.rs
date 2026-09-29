use std::io::{Write as _, stdin, stdout};
use std::str::FromStr as _;

use vz_rs::codec::{format, parse};
use vz_rs::expand;

fn main() -> ! {
    loop {
        print!("Matrix to expand: ");
        stdout().flush().unwrap();
        let mut buf1 = String::new();
        stdin().read_line(&mut buf1).unwrap();

        let mut buf2 = String::new();
        while usize::from_str(buf2.trim()).is_err() {
            buf2 = String::new();
            print!("Index: ");
            stdout().flush().unwrap();
            stdin().read_line(&mut buf2).unwrap();
        }

        let n = usize::from_str(buf2.trim()).unwrap();
        println!("{}", format(&expand(&parse(buf1.trim()), n)));
    }
}
