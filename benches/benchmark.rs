#![feature(test)]

extern crate test;

use tengwar::quenya::*;


const SAMPLES: &[&[char]] = &[
    &['c'],
    &['k'],
    &['y'],
    &['w'],
    &['r', 'd'],
    &['n', 'g'],
    &['i'],
    &['k', 'w'],
    &['ü'],
    &['á'],
    &['a', 'a'],
    &['ï'],
    &['ë'],
    &['í'],
    &['i', 'i'],
    &['g'],
    &['c', 'h'],
    &['l'],
    &['l', 'd'],
    &['q', 'u'],
    &['q'],
    &['c', 'w'],
    &['n', 'g', 'w'],
    &['n', 'w'],
    &['s'],
    &['s', 's'],
    &['ó'],
    &['ö'],
    &['h', 'w'],
    &['ñ'],
    &['e'],
    &['u'],
    &['é'],
    &['e', 'e'],
    &['o', 'o'],
    &['o'],
    &['n', 'q', 'u'],
    &['z'],
    &['a'],
    &['ä'],
    &['n', 'c'],
    &['ú'],
    &['u', 'u'],
];


#[bench]
fn bench_consonants(b: &mut test::Bencher) {
    let mut out: Vec<Option<char>> = Vec::with_capacity(SAMPLES.len());

    b.iter(|| {
        for slice in SAMPLES {
            out.push(consonant_char(slice));
        }
    });

    // println!("{:?}", out.into_iter().map(|c| c.unwrap_or('!')).collect::<String>());
}
