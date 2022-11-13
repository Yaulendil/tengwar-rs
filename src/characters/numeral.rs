use super::consts::*;


/// Prefix expected to be found on input numbers meant to be shown as Base-10.
const PREF_B10_IN: char = '#';
/// Suffix expected to be found on input numbers that are meant to be ordinal.
const SUFF_ORD_IN: char = '@';


fn int(mut n: isize, base: isize) -> (bool, Vec<usize>) {
    if n == 0 {
        return (false, vec![0]);
    }

    let mut digits = Vec::new();
    let neg = n.is_negative();

    while n != 0 {
        digits.push((n % base).unsigned_abs());
        n /= base;
    }

    (neg, digits)
}


#[derive(Clone, Copy, Debug)]
pub struct Numeral {
    /// Numeric value.
    pub value: isize,

    /// Whether the number will be displayed in Decimal, base 10, rather than in
    ///     Duodecimal, base 12.
    pub base_10: bool,
    /// Whether the number is ordinal ("first"), rather than cardinal ("one").
    pub ordinal: bool,

    /// Whether the base of the number will be denoted with lines, rather than
    ///     with dots.
    pub lines: bool,
}

impl Numeral {
    /// Prefix to be prepended to the output form of a negative number.
    pub const PREF_NEG_OUT: char = '-';
    /// Suffix to be appended to the output form of an ordinal number.
    pub const SUFF_ORD_OUT: char = TEMA_TINCO.single_ex; // 

    pub const fn new(value: isize, base_10: bool) -> Self {
        Self {
            value,
            base_10,
            ordinal: false,
            lines: false,
        }
    }

    pub const fn decimal(value: isize) -> Self {
        Self::new(value, true)
    }

    pub const fn duodecimal(value: isize) -> Self {
        Self::new(value, false)
    }

    pub const fn with_decimal(mut self, decimal: bool) -> Self {
        self.base_10 = decimal;
        self
    }

    pub const fn with_lines(mut self, lines: bool) -> Self {
        self.lines = lines;
        self
    }

    pub const fn with_ordinal(mut self, ordinal: bool) -> Self {
        self.ordinal = ordinal;
        self
    }
}

impl Numeral {
    pub fn parse(mut slice: &[char]) -> Option<(Self, usize)> {
        //  Idea for this notation borrowed from Tecendil. There is most likely
        //      a better way to do it, given the vastly different style of
        //      interface.
        let base_10: bool = match slice {
            [PREF_B10_IN, after @ ..] => {
                slice = after;
                true
            }
            _ => false,
        };

        let neg: bool = 0 < slice.len() && slice[0] == '-';
        let end: usize = neg as usize
            + slice.iter()
            .skip(neg as usize)
            .take_while(|&&n| '0' <= n && n <= '9')
            .count();

        if end > neg as usize {
            let value: isize = slice.iter()
                .take(end)
                .collect::<String>()
                .parse()
                .ok()?;

            //  TODO: Maybe move the Ordinal check up to the Tengwa level.
            let ordinal = end < slice.len() && slice[end] == SUFF_ORD_IN;
            let numeral = Self::new(value, base_10).with_ordinal(ordinal);
            let chars = end
                + base_10 as usize // +1 if Base-10.
                + ordinal as usize // +1 if Ordinal.
                ;

            Some((numeral, chars))
        } else {
            None
        }
    }

    //noinspection RsBorrowChecker
    pub fn render(&self) -> String {
        let negative: bool;
        let digits: Vec<usize>;
        let size: usize;

        let base_marker: char;
        let mark_ones: bool;

        if self.base_10 {
            //  Base-10 number.
            (negative, digits) = int(self.value, 10);
            size = negative as usize + digits.len() * 6 + 3;

            if self.lines {
                base_marker = DC_OVER_LINE;
                mark_ones = true;
            } else {
                base_marker = DC_OVER_DOT_1;
                mark_ones = true;
            }
        } else {
            //  Base-12 number.
            (negative, digits) = int(self.value, 12);
            size = negative as usize + digits.len() * 6;

            if self.lines {
                base_marker = DC_UNDER_LINE_H;
                mark_ones = true;
            } else {
                base_marker = DC_UNDER_DOT_1;
                mark_ones = false;
            }
        }

        let mut text = String::with_capacity(size + self.ordinal as usize * 3);

        if negative {
            text.push(Self::PREF_NEG_OUT);
        }

        match digits.as_slice() {
            [] => {}
            /*[0, 1] if !self.base_10 => {
                //  TODO
                text.push(NUMERAL[12]);
                text.push(base_marker);
            }*/
            [digit] => {
                text.push(NUMERAL[*digit]);
                text.push(base_marker);
            }
            [first, digits @ ..] => {
                text.push(NUMERAL[*first]);
                text.push(DC_UNDER_RING);

                if mark_ones {
                    text.push(base_marker);
                }

                for digit in digits {
                    text.push(NUMERAL[*digit]);
                    text.push(base_marker);
                }
            }
        }

        if self.ordinal {
            text.push(Self::SUFF_ORD_OUT);
        }

        text
    }
}

impl<N: Into<isize>> From<N> for Numeral {
    fn from(value: N) -> Self {
        Self::new(value.into(), false)
    }
}
