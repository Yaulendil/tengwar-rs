#![feature(test)]

extern crate test;

use tengwar::{mode::quenya::*, Rules, TengwarMode};


const SAMPLES: &[&[char]] = {
    &[
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
    ]
};
const TEXTS_Q: &[&str] = &[
    "tinco parma calma qessë ando umbar anga ungwë\
    \nþúlë formen harma hwesta anto ampa anca unqë\
    \nnúmen malta ngoldo ngwalmë orë vala anna wilya\
    \nrómen arda lambë alda silmë ázë essë\
    \nhyarmen yanta úrë ossë halla telco ára",

	"ai! laurië lantar lassi súrinen",
	"yéni únótimë ve rámar aldaron",
	"yéni ve lintë yuldar avánier",
	"mi oromardi lissë miruvóreva",
	"Andúnë pella Vardo tellumar nu luini",
	"yassen tintilar i eleni",
	"ómaryo airetári.lírinen",
	"sí man i yulma nin enquantuva?",
	"an sí Tintallë Varda Oiolossëo ve fanyar máryat Elentári ortanë",
	"ar ilyë tier undulávë lumbulë",
	"ar sindanóriello caita mornië",
	"i falmalinnar imbë met",
	"ar hísië untúpa Calaciryo míri oialë",
	"sí vanwa ná , Rómello vanwa , Valimar!",
	"namárië! nai hiruvalyë Valimar",
	"nai elyë hiruva : namárië! :-",
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


#[bench]
fn bench_quenya_1(b: &mut test::Bencher) {
    let mut out: Vec<Vec<_>> = Vec::with_capacity(TEXTS_Q.len());

    b.iter(|| {
        out.clear();

        for text in TEXTS_Q {
            out.push(Quenya::tokens(text));
        }
    });
}


#[bench]
fn bench_quenya_2(b: &mut test::Bencher) {
    let mut out: Vec<Vec<_>> = Vec::with_capacity(TEXTS_Q.len());

    b.iter(|| {
        out.clear();

        for text in TEXTS_Q {
            out.push(<Quenya as TengwarMode>::transcribe(text));
        }
    });

    let mut out_orig: Vec<Vec<_>> = Vec::with_capacity(TEXTS_Q.len());

    for text in TEXTS_Q {
        out_orig.push(Quenya::tokens(text));
    }

    for i in 0..out.len() {
        assert_eq!(
            out[i].iter().cloned().collect::<String>(),
            out_orig[i].iter().cloned().collect::<String>(),
        );
    }
}
