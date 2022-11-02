#![feature(test)]

extern crate test;

use tengwar::*;


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
const TEXTS_S: &[&str] = &[
    "mae govannen",
    "ennyn durin aran moria , pedo mellon a minno",
    "im narvi hain echant , celebrimbor o eregion teithant i thiw hin",
	"a Elbereth Gilthoniel",
	"silivren penna míriel",
	"o menel aglar elenath",
	"na-chaered palan-díriel",
	"o galadhremmin ennorath",
	"Fanuilos , le linnathon",
	"nef aear , sí nef aearon",
	"a Elbereth Gilthoniel",
	"o menel palan-diriel",
	"le nallon sí di'nguruthos",
	"a tiro nin , Fanuilos :-",
];


#[bench]
fn bench_consonants(b: &mut test::Bencher) {
    let mut out: Vec<Option<char>> = Vec::with_capacity(SAMPLES.len());

    b.iter(|| {
        out.clear();

        for slice in SAMPLES {
            out.push(mode::quenya::consonant_char(slice));
        }
    });

    // println!("{:?}", out.into_iter().map(|c| c.unwrap_or('!')).collect::<String>());
}


#[bench]
fn bench_mode_quenya(b: &mut test::Bencher) {
    let mut out: Vec<Vec<_>> = Vec::with_capacity(TEXTS_Q.len());

    b.iter(|| {
        out.clear();

        for text in TEXTS_Q {
            // out.push(Quenya::tokens(text));
            out.push(<Quenya as TengwarMode>::transcribe(text));
            // out.push(text.to_tengwar2::<Quenya, Vec<_>>());
        }
    });

    // check::<Quenya>(TEXTS_Q, &out);
}


#[bench]
fn bench_mode_beleriand(b: &mut test::Bencher) {
    let mut out: Vec<Vec<_>> = Vec::with_capacity(TEXTS_S.len());

    b.iter(|| {
        out.clear();

        for text in TEXTS_S {
            out.push(mode::beleriand::Beleriand2::tokens(text));
        }
    });

    // check::<Beleriand>(TEXTS_S, &out);
}


#[bench]
fn bench_mode_gondor(b: &mut test::Bencher) {
    let mut out: Vec<Vec<_>> = Vec::with_capacity(TEXTS_S.len());

    b.iter(|| {
        out.clear();

        for text in TEXTS_S {
            out.push(Gondor::tokens(text));
            // out.push(<Gondor as TengwarMode>::transcribe(text));
        }
    });

    check::<Gondor>(TEXTS_S, &out);
}


fn check<M: Rules>(sample: &[&str], converted: &[Vec<Token>]) {
    assert_eq!(sample.len(), converted.len());

    for (src, new) in sample.iter().zip(converted) {
        let str_old: String = M::transcribe(src);
        let str_new: String = new.iter().cloned().collect();

        assert_eq!(
            str_old, str_new,
            "New transcription does not match old.\
            \nOld: {}\
            \nNew: {}",
            str_old, str_new,
        );
    }
}
