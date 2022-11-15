#![feature(test)]

extern crate test;

use tengwar::*;


const TEXTS_Q: &[&str] = &[
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
fn bench_mode_quenya(b: &mut test::Bencher) {
    let mut out: Vec<Vec<_>> = Vec::with_capacity(TEXTS_Q.len());

    b.iter(|| {
        out.clear();

        for text in TEXTS_Q {
            out.push(text.transcriber::<Quenya>().collect());
        }
    });
}


#[bench]
fn bench_mode_beleriand(b: &mut test::Bencher) {
    let mut out: Vec<Vec<_>> = Vec::with_capacity(TEXTS_S.len());

    b.iter(|| {
        out.clear();

        for text in TEXTS_S {
            out.push(text.transcriber::<Beleriand>().collect());
        }
    });
}


#[bench]
fn bench_mode_gondor(b: &mut test::Bencher) {
    let mut out: Vec<Vec<_>> = Vec::with_capacity(TEXTS_S.len());

    b.iter(|| {
        out.clear();

        for text in TEXTS_S {
            out.push(text.transcriber::<Gondor>().collect());
        }
    });
}
