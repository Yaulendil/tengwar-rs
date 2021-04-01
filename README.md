# Tengwar

[
![Crates.io](https://img.shields.io/crates/v/tengwar?logo=rust&style=for-the-badge&label=crate)
![Downloads](https://img.shields.io/crates/d/tengwar?style=flat-square)
](https://crates.io/crates/tengwar)

[
![docs.rs](https://docs.rs/tengwar/badge.svg?style=for-the-badge)
](https://docs.rs/tengwar)

Automated conversion of Roman text into Tengwar in the Unicode Private Use Area.
Currently supports Quenya, the Sindarin Mode of Beleriand, and the Sindarin Mode of Gondor.

The codepoints used are the ones found in [the ConScript Unicode Registry](https://en.wikipedia.org/wiki/Tengwar#ConScript_Unicode_Registry), and will only correctly render if a font is installed that includes them.

## Usage

When run directly, reads all command-line arguments and prints them back in Tengwar, separated by spaces:

    > tengwar "Elen síla lúmenn' omentielvo."
       

    > tengwar "Utúlie'n aurë! Aiya Eldalië ar Atanatári, utúlie'n aurë!"
            

Alternative "Modes" can be selected via command-line switch:

    > tengwar --beleriand "Ennyn Durin aran Moria: pedo mellon a minno!"
           

    > tengwar --gondor "Ennyn Durin aran Moria: pedo mellon a minno!"
           

The full list of available modes can be viewed with `tengwar --help`.

If no arguments are provided, reads from Standard Input and prints transliteration to Standard Output line by line.
All input is assumed to be in UTF-8 encoding, and output will also be in UTF-8 encoding.

## Features

Cargo Features are available to provide alternative treatment of "long" vowels;
Some typefaces lack the correct ligatures to properly display doubled diacritics, while others lack characters for the dedicated characters for them.

The default behavior is to place long vowels on a lengthened "ára" carrier telco.
In order to use doubled diacritical characters instead, run the compilation with `--features "long-vowel-double"`.
In order to use the unique diacritical characters, do so with `--features "long-vowel-unique"`.
