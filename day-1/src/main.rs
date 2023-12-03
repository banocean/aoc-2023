// https://adventofcode.com/2023/day/1
use std::str::{FromStr};

fn main() {
    let data = include_bytes!("input.txt");
    let mut i = 0;
    let mut count: usize = 0;
    while let Some((c1, c2)) = process_line(&mut i, data) {
        #[cfg(debug_assertions)]
        println!("{} {}", c1 as char, c2 as char);

        count += usize::from_str(
            std::str::from_utf8(&[c1, c2]).unwrap()
        ).unwrap();
    }

    println!("{count}")
}

fn process_line(i: &mut usize, chars: &[u8]) -> Option<(u8, u8)> {
    let mut x = (None, None);
    loop {
        let char = *chars.get(*i)?;
        if matches!(char, b'0'..=b'9') {
            x.0 = Some(char);
            loop {
                *i += 1;
                let char = *chars.get(*i)?;
                match char {
                    b'0'..=b'9' => x.1 = Some(char),
                    b'\n' => break,
                    _ => {}
                }
            }
            break;
        }
        *i += 1;
    }

    Some((x.0?, x.1.or(x.0).unwrap()))
}