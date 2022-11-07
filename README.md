# Tengwar

[crates.io]: https://img.shields.io/crates/v/tengwar?logo=rust&label=crates.io
[docs.rs]: https://docs.rs/tengwar/badge.svg
[![crates.io]](https://crates.io/crates/tengwar)
[![docs.rs]](https://docs.rs/tengwar)

Automated conversion of Latin text into Tengwar codepoints in the Unicode Private Use Area.
Primarily targets the Classical Quenya mode, with tenuous support for the Sindarin modes of Beleriand and Gondor.

The codepoints used are the ones defined by the [Free Tengwar Font Project](https://freetengwar.sourceforge.net/mapping.html), based on the mapping in the [ConScript Unicode Registry](https://en.wikipedia.org/wiki/Tengwar#ConScript_Unicode_Registry), and will only correctly render if a font is installed that includes them.

## Installation

With [Cargo](https://github.com/rust-lang/cargo) installed, the following command will build and install from [crates.io](https://crates.io) automatically:

    cargo install tengwar

## Usage

When run directly, reads all command-line arguments and prints them back in Tengwar, separated by spaces:

    > tengwar "Elen síla lúmenn' omentielvo :"
        

    > tengwar "Utúlie'n aurë ! Aiya Eldalië ar Atanatári , utúlie'n aurë !."
              

If no arguments are provided, reads from Standard Input and prints transliteration to Standard Output line by line.
All input is assumed to be UTF-8 encoded, and output will also be UTF-8 encoded.

### Modes

Alternative "Modes" can be selected via command-line switch:

    > tengwar --beleriand "Ennyn Durin aran Moria :. pedo mellon a minno :"
             

    > tengwar --gondor "Ennyn Durin aran Moria :. pedo mellon a minno :"
             

The full list of available modes can be viewed with `tengwar -h`.

### Ligation with Zero-Width Joiners

In certain typefaces, the [Zero-Width Joiner](https://en.wikipedia.org/wiki/Zero-width_joiner) may be used to form ligatures of Tengwar.
When invoked with the `--ligate-zwj` (or `-z`) switch on the command line, this program will insert joiners into the output text between certain characters, based on various overly convoluted rules.
These rules are based on the ligature behavior of [Tengwar Telcontar](https://freetengwar.sourceforge.net/tengtelc.html), as well as some degree of personal taste.

For typefaces that do not support these ligatures, the presence of the joining characters ***should*** not affect the rendering;
However, it does increase the number of bytes in the output string by approximately 15%.

### Note on punctuation

Tengwar [punctuation](https://at.mansbjorkman.net/teng_punctuation.htm) is essentially impossible to reliably extrapolate from punctuated Latin text.
It is therefore probably best to not even try, and instead to punctuate the input text with the output in mind.

This program does convert punctuation marks into Unicode codepoints, but whitespace is passed through verbatim, neither added nor subtracted¹.
As of this writing at version `0.8.0`, punctuation is processed as can be seen above and in the following table:

| Input              | Output |
|--------------------|:------:|
| `'`, `.`, `,`, `·` |  ``   |
| `:`, `;`           |  ``   |
| `⁝`, `︙`           |  ``   |
| `⁘`, `⁛`, `…`      |  ``   |
| `⸭`                |  ``   |
| `-`                |  ``   |
| `=`                |  ``   |
| `?`                |  ``   |
| `!`                |  ``   |
| `‖`*               |  ``   |
| `(`, `[`, `“`      |  ``   |
| `)`, `]`, `”`, `„` |  ``   |

The Pipe character (`|`) is also converted to ``, but it cannot be included in this table due to a technical limitation of the Markdown format.

It may be important to note that the Tilde (`~`) is **not** converted, despite strongly resembling the punctuation character ``.
This is because the Tilde represents a specific type of whitespace in LaTeX, and running within LaTeX is the motivating use case for the creation of this program.
The Tilde is left alone so that it can safely pass through and be read by LaTeX.

---

## Features

[Cargo Features](https://doc.rust-lang.org/cargo/reference/features.html#command-line-feature-options) allow very powerful changes to the behavior of a program to be baked in at compile-time.
For my current project, I am using a version of this program compiled with `--no-default-features --features "alt-rince long-vowel-double"` as arguments to Cargo.

### `alt-rince`

A consonant which is followed by an /s/ sound is often represented by adding an additional hook, known as a *sa-rincë*, for example turning "" into "".
If this occurs at the end of a word, a larger version may be used, as seen in "".
Because in many typefaces it is not well-supported, this alternate version will not be used by default;
However, it can be enabled by including `--features "alt-rince"` when installing.

### `circumflex`

In some manuscripts, it is not uncommon to replace the three-dot Tehta () with a simpler one resembling a circumflex ().
This variant may be preferred when transliterating text that is meant to represent a note quickly written by hand.
This can be enabled with `--features "circumflex"`.

### Long Vowels

Multiple Cargo features are available to provide alternative treatment of "long" vowels;
Some typefaces lack the correct ligatures to properly display doubled diacritics, while others lack forms for the dedicated codepoints.
Some may not even support either approach at all.

The default behavior is to double the characters for long vowels, with the feature `long-vowel-double` set by default.
In order to use the *unique* diacritical characters, run the compilation with `--features "long-vowel-unique"`.
In order to instead place long vowel marks onto the lengthened *ára* carrier, do so by disabling the doubling behavior with `--no-default-features`².

Note: Some diacritical vowels will **always** use the *ára* carrier, due to their shape being unsuitable for doubling.

### `nuquernar`

Two of the Tengwar, *Silmë* () and *Essë* (), have inverted *"Nuquerna"* variants.
These variants are often used when a diacritical vowel is to be added, and this is the default behavior of this program.
However, in some typefaces, the normal non-inverted versions may look better.
This behavior can therefore be disabled with `--no-default-features`².

---

¹ While ***this program*** does not affect whitespace, it can only work with the discrete arguments provided to it by the environment running it.
Most environments will split your input at spaces and provide the surrounding words as unconnected values;
This program cannot distinguish between `tengwar asdf qwert` and `tengwar asdf<TAB>qwert`.
It is therefore highly recommended to enclose all of your input text in double quotes.

² If you run the compilation with `--no-default-features`, it will disable *both* default features.
This is a limitation of how Cargo resolves the defaults.
If you want to, for example, disable the doubled vowels behavior, but retain the *Nuquerna* variants, you may disable both but then reenable the one you want by using `--no-default-features --features "nuquernar"`.
