use std::fmt::{Display, Formatter};
use super::consts::*;


pub const BASE_10_DOT: char = DC_OVER_DOT_1;
pub const BASE_10_LINE: char = DC_OVER_LINE;
pub const BASE_12_DOT: char = DC_UNDER_DOT_1;
pub const BASE_12_LINE: char = DC_UNDER_LINE_H;
pub const MOD_UNITS: char = DC_UNDER_RING;

/// Prefix expected to be found on input numbers meant to be shown as Base-10.
pub const PREF_DEC_IN: char = '#';
/// Suffix expected to be found on input numbers that are meant to be ordinal.
pub const SUFF_ORD_IN: char = '@';
/// Suffix expected to be found on input numbers that are sequence indices.
pub const SUFF_SEQ_IN: char = '#';

//  NOTE: The maximum value of this type determines the maximum supported base
//      of the number system. It is somewhat hard to imagine any new Tolkien
//      notes being discovered that introduce a system beyond Base-256.
type Digit = u8;


struct Digits {
    negative: bool,
    digits: Vec<Digit>,
}

impl Digits {
    fn zero() -> Self {
        Self {
            negative: false,
            digits: vec![0],
        }
    }

    fn get(n: isize, base: Digit) -> Self {
        if n == 0 {
            Self::zero()
        } else {
            let negative: bool = n.is_negative();
            let mut n: usize = n.unsigned_abs();

            //  TODO: https://github.com/rust-lang/rust/issues/70887
            // let len: usize = n.checked_ilog10().unwrap_or(0) as usize + 1;
            // let mut digits = Vec::with_capacity(len);
            let mut digits = Vec::new();

            while n != 0 {
                digits.push((n % base as usize) as _);
                n /= base as usize;
            }

            Self { negative, digits }
        }
    }

    fn decimal(value: isize) -> Self {
        Self::get(value, 10)
    }

    fn duodecimal(value: isize) -> Self {
        Self::get(value, 12)
    }

    fn size(&self) -> usize {
        self.negative as usize + self.digits.len() * 6
    }
}


pub const fn find_index(slice: &[char]) -> Option<(char, usize)> {
    let value: usize;
    let chars: usize;

    match slice {
        [c10 @ '0'..='9', c01 @ '0'..='9', SUFF_SEQ_IN, ..] => {
            let ones: usize = *c01 as usize - '0' as usize;
            let tens: usize = *c10 as usize - '0' as usize;

            value = 10 * tens + ones;
            chars = 3;
        }
        [c01 @ '0'..='9', SUFF_SEQ_IN, ..] => {
            let ones: usize = *c01 as usize - '0' as usize;

            value = ones;
            chars = 2;
        }
        _ => return None,
    }

    match value.checked_sub(1) {
        /*//  TODO: Not stable as const.
        Some(i) => match SEQUENCE.get(i) {
            Some(char) => Some((*char, len)),
            None => None,
        }*/
        Some(idx) if idx < SEQUENCE.len() => Some((SEQUENCE[idx], chars)),
        _ => None,
    }
}


/// A numeric value, paired with formatting information.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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

    /// Whether the less significant digits of will be written first. This is
    ///     the style preferred by the Eldar, but as the least significant digit
    ///     is marked, it can go either way without ambiguity.
    pub little_endian: bool,
}

impl Numeral {
    /// Prefix to be prepended to the output form of a negative number.
    pub const PREF_NEG_OUT: char = '-';
    /// Prefix to be prepended to the output form of a positive number.
    pub const PREF_POS_OUT: char = '+';
    /// Suffix to be appended to the output form of an ordinal number.
    pub const SUFF_ORD_OUT: char = TEMA_TINCO.single_ex; // 

    /// Define a value in either Base-10 or Base-12.
    pub const fn new(value: isize, base_10: bool) -> Self {
        Self {
            value,
            base_10,
            ordinal: false,
            lines: false,
            little_endian: true,
        }
    }

    /// Define a Base-10 decimal value.
    pub const fn decimal(value: isize) -> Self {
        Self::new(value, true)
    }

    /// Define a Base-12 duodecimal, or dozenal, value.
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
    /// Try to extract a numeric value from a slice of [`char`]s. If successful,
    ///     returns a new `Numeral`, along with the number of `char`s that were
    ///     processed in order to find it.
    pub fn parse(mut slice: &[char]) -> Option<(Self, usize)> {
        //  Idea for this notation borrowed from Tecendil. There is most likely
        //      a better way to do it, given the vastly different style of
        //      interface.
        let base_10: bool = match slice {
            [PREF_DEC_IN, after @ ..] => {
                slice = after;
                true
            }
            _ => false,
        };

        let neg: bool = 0 < slice.len() && slice[0] == '-';
        let end: usize = neg as usize
            + slice.iter()
            .skip(neg as usize)
            .take_while(|n| n.is_ascii_digit())
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
}

impl Display for Numeral {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut value: Digits;
        let size: usize;

        let base_marker: char;
        let mark_ones: bool;

        if self.base_10 ^ f.alternate() {
            //  Base-10 number.
            value = Digits::decimal(self.value);
            size = value.size() + 3;

            if self.lines {
                base_marker = BASE_10_LINE;
                mark_ones = true;
            } else {
                base_marker = BASE_10_DOT;
                mark_ones = true;
            }
        } else {
            //  Base-12 number.
            value = Digits::duodecimal(self.value);
            size = value.size();

            if self.lines {
                base_marker = BASE_12_LINE;
                mark_ones = true;
            } else {
                base_marker = BASE_12_DOT;
                mark_ones = false;
            }
        }

        let mut text = String::with_capacity(size + self.ordinal as usize * 3);
        // let mut width: usize = value.digits.len();

        if value.negative {
            text.push(Self::PREF_NEG_OUT);
            // width += 1;
        } else if f.sign_plus() {
            text.push(Self::PREF_POS_OUT);
            // width += 1;
        }

        if !self.little_endian {
            value.digits.reverse();
        }

        match value.digits.as_slice() {
            [] => {}
            /*[0, 1] if !self.base_10 => {
                //  TODO
                text.push(NUMERAL[12]);
                text.push(base_marker);
            }*/
            [digit] => {
                text.push(NUMERAL[*digit as usize]);
                text.push(base_marker);
            }
            [units, digits @ ..] if self.little_endian => {
                text.push(NUMERAL[*units as usize]);
                text.push(MOD_UNITS);

                if mark_ones {
                    text.push(base_marker);
                }

                for digit in digits {
                    text.push(NUMERAL[*digit as usize]);
                    text.push(base_marker);
                }
            }
            [digits @ .., units] => {
                for digit in digits {
                    text.push(NUMERAL[*digit as usize]);
                    text.push(base_marker);
                }

                text.push(NUMERAL[*units as usize]);
                text.push(MOD_UNITS);

                if mark_ones {
                    text.push(base_marker);
                }
            }
        }

        if self.ordinal {
            text.push(Self::SUFF_ORD_OUT);
            // width += 1;
        }

        //  TODO: Wait for tests before enabling this mess.
        /*let fill = f.fill();
        match f.width() {
            None => f.write_str(&text)?,
            Some(min) if min <= width => f.write_str(&text)?,
            Some(min) => match f.align() {
                None => f.write_str(&text)?,
                Some(Alignment::Center) => {
                    let fills = min - width;
                    let fills_l = fills / 2;
                    let fills_r = fills - fills_l;

                    for _ in 0..fills_l { f.write_char(fill)?; }
                    f.write_str(&text)?;
                    for _ in 0..fills_r { f.write_char(fill)?; }
                }
                Some(Alignment::Left) => {
                    f.write_str(&text)?;
                    for _ in 0..min - width { f.write_char(fill)?; }
                }
                Some(Alignment::Right) => {
                    for _ in 0..min - width { f.write_char(fill)?; }
                    f.write_str(&text)?;
                }
            }
        }

        Ok(())*/

        Display::fmt(text.as_str(), f)
    }
}

impl<N: Into<isize>> From<N> for Numeral {
    fn from(value: N) -> Self {
        Self::new(value.into(), false)
    }
}
