#![allow(dead_code)]

use std::str::FromStr;
use crate::characters::Numeral;


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


pub fn find_integer(mut slice: &[char]) -> Option<(Numeral<isize>, usize)> {
    //  Idea for this notation borrowed from Tecendil. There is most likely a
    //      better way to do it, given the vastly different style of interface.
    let decimal: bool = match slice {
        ['#', after @ ..] => {
            slice = after;
            true
        }
        _ => false,
    };

    let negative: bool = matches!(slice, ['-', ..]);
    let end: usize = negative as usize
        + slice[negative as usize..].iter()
        .take_while(|&&n| '0' <= n && n <= '9')
        .count();

    if end > negative as usize {
        let value: isize = slice[..end].iter()
            .collect::<String>()
            .parse()
            .ok()?;

        //  TODO: Decide on a language-agnostic ordinal suffix. Also, decide
        //      whether this is even the right place to check for it. Would it
        //      be better to look for it as a completely separate Tengwa? That
        //      would make it easier to add modifiers to it.
        /*let ord_suf = match value.abs() % 100 {
            10..=19 => ['t', 'h'],
            n => match n % 10 {
                1 => ['s', 't'],
                2 => ['n', 'd'],
                3 => ['r', 'd'],
                _ => ['t', 'h'],
            }
        };
        let ordinal: bool = slice[end..].starts_with(&ord_suf);*/

        Some((
            Numeral::new(value, decimal)/*.with_ordinal(ordinal)*/,
            end + decimal as usize/*
                + ordinal as usize * 2*/,
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
