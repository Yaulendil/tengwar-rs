# Tengwar

Automated conversion of Roman text into Tengwar in the Unicode Private Use Area. Currently supports only Quenya.

The codepoints used are the ones found in [the ConScript Unicode Registry](https://en.wikipedia.org/wiki/Tengwar#ConScript_Unicode_Registry), and will only correctly render if a font is installed that includes them.

## Usage

When run directly, reads all command-line arguments and prints them back in Tengwar, separated by spaces.

    > tengwar "Elen síla lúmenn' omentielvo."
      ' .

    > tengwar "Utúlie'n aurë! Aiya Eldalië ar Atanatári, utúlie'n aurë!"
    ' !    , ' !
