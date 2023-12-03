// https://adventofcode.com/2023/day/1
use std::str::{FromStr};

fn main() {
    let data = include_bytes!("input.txt");
    let mut i = 0;
    let mut count: usize = 0;
    while let Some((c1, c2)) = process_line(&mut i, data) {
        println!("{c1} {c2}");
        count += usize::from_str(
            std::str::from_utf8(&[c1 as u8, c2 as u8]).unwrap()
        ).unwrap();
    }

    println!("{count}")
}

fn process_line(i: &mut usize, chars: &[u8]) -> Option<(char, char)> {
    let mut x = (None, None);
    loop {
        let char = *chars.get(*i)? as char;
        if char.is_numeric() {
            x.0 = Some(char);
            loop {
                *i += 1;
                let char = *chars.get(*i)? as char;
                if char.is_numeric() {
                    x.1 = Some(char);
                } else if char == '\n' {
                    break;
                }
            }
            break;
        }
        *i += 1;
    }

    Some((x.0?, x.1.or(x.0).unwrap()))
}