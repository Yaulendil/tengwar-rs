#![allow(dead_code)]

use std::str::FromStr;


pub fn find_decimal<N: FromStr>(slice: &[char]) -> Option<(N, usize)> {
    let mut decimal = false;
    let (mut end, negative) = {
        if matches!(slice, ['-', ..]) {
            (1, true)
        } else {
            (0, false)
        }
    };

    for ch in slice[end..].iter() {
        match ch {
            '0'..='9' => {
                end += 1;
            }
            '.' if !decimal => {
                decimal = true;
                end += 1;
            }
            _ => break,
        }
    }

    if end > negative as usize && slice[end - 1] == '.' {
        end -= 1;
    }

    if end > negative as usize {
        Some((
            slice[..end].iter()
                .collect::<String>()
                .parse()
                .ok()?,
            end,
        ))
    } else {
        None
    }
}


pub fn find_integer<N: FromStr>(slice: &[char]) -> Option<(N, usize)> {
    let negative: bool = matches!(slice, ['-', ..]);
    let end: usize = negative as usize
        + slice[negative as usize..].iter()
        .take_while(|&&n| '0' <= n && n <= '9')
        .count();

    if end > negative as usize {
        Some((
            slice[..end].iter()
                .collect::<String>()
                .parse()
                .ok()?,
            end,
        ))
    } else {
        None
    }
}


pub fn find_integer_positive<N: FromStr>(slice: &[char]) -> Option<(N, usize)> {
    let end: usize = slice.iter()
        .take_while(|&&n| '0' <= n && n <= '9')
        .count();

    if end > 0 {
        Some((
            slice[..end].iter()
                .collect::<String>()
                .parse()
                .ok()?,
            end,
        ))
    } else {
        None
    }
}
