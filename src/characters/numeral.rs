use super::consts::*;


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


//  TODO: Either figure out what to do about floats or drop the generic.
#[derive(Clone, Copy, Debug)]
pub struct Numeral<N> {
    /// Numeric value.
    pub value: N,

    /// Whether the number will be displayed in Decimal, base 10, rather than in
    ///     Duodecimal, base 12.
    pub decimal: bool,
    /// Whether the number is ordinal ("first"), rather than cardinal ("one").
    pub ordinal: bool,

    /// Whether the base of the number will be denoted with lines, rather than
    ///     with dots.
    pub lines: bool,
}

impl<N> Numeral<N> {
    pub const PREFIX_NEGATIVE: char = '-';

    pub const fn new(value: N, decimal: bool) -> Self {
        Self {
            value,
            decimal,
            ordinal: false,
            lines: false,
        }
    }

    pub const fn decimal(value: N) -> Self {
        Self::new(value, true)
    }

    pub const fn duodecimal(value: N) -> Self {
        Self::new(value, false)
    }

    pub const fn with_decimal(mut self, decimal: bool) -> Self {
        self.decimal = decimal;
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

impl Numeral<isize> {
    //noinspection RsBorrowChecker
    pub fn render(&self) -> String {
        let negative: bool;
        let digits: Vec<usize>;
        let size: usize;

        let base_marker: char;
        let mark_ones: bool;

        if self.decimal {
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

        let mut text = String::with_capacity(size);

        if negative {
            text.push(Self::PREFIX_NEGATIVE);
        }

        match digits.as_slice() {
            [] => {}
            /*[0, 1] if !self.decimal => {
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
            text.push(TEMA_TINCO.single_ex);
        }

        text
    }
}

impl<N> From<N> for Numeral<N> {
    fn from(value: N) -> Self {
        Self::new(value, false)
    }
}
