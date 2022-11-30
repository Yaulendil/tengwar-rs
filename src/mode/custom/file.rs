use super::*;


#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum Check {
    Consonant,
    Diphthong,
    Vowel,
    Rince,
    Labial,
    Nasal,
    Palatal,
    Replacements,
}


#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum Allowance {
    Allow,
    Forbid,
    Require,
}


#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Replacement {
    pub old: char,
    pub new: char,
}


#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Position {
    pub after_consonant: Allowance,
    pub after_vowel: Allowance,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Modifier {
    pub pattern: String,
    pub valid: Position,
}


#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
pub struct GlyphSpec {
    pub tengwa: Option<char>,
    pub tehta: Option<TehtaSpec>,
    #[serde(default)]
    pub tehta_alt: bool,

    #[serde(default)]
    pub rince: bool,
    #[serde(default)]
    pub nasal: bool,
    #[serde(default)]
    pub labial: bool,
    #[serde(default)]
    pub palatal: bool,
    #[serde(default)]
    pub nuquerna: bool,
    #[serde(default)]
    pub long_cons: bool,
    #[serde(default)]
    pub dot_inner: bool,
    #[serde(default)]
    pub dot_under: bool,
}

impl From<char> for GlyphSpec {
    fn from(c: char) -> Self {
        Self { tengwa: Some(c), ..Default::default() }
    }
}

impl From<TehtaSpec> for GlyphSpec {
    fn from(spec: TehtaSpec) -> Self {
        Self { tehta: Some(spec), ..Default::default() }
    }
}

impl From<GlyphSpec> for Glyph {
    fn from(spec: GlyphSpec) -> Self {
        Self {
            base: spec.tengwa,
            tehta: spec.tehta.map(Tehta::from),
            tehta_alt: spec.tehta_alt,
            tehta_first: false,
            vowels: Default::default(),
            rince: spec.rince,
            rince_final: false,
            nasal: spec.nasal,
            labial: spec.labial,
            palatal: spec.palatal,
            nuquerna: spec.nuquerna,
            long_cons: spec.long_cons,
            dot_inner: spec.dot_inner,
            dot_under: spec.dot_under,
            ligate_short: false,
            ligate_zwj: 0,
            _p: Default::default(),
        }
    }
}


#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct TehtaSpec {
    pub base: char,

    #[serde(alias = "alt")]
    pub alternate: Option<char>,

    #[serde(default)]
    pub can_double: bool,
}

impl From<char> for TehtaSpec {
    fn from(base: char) -> Self {
        Self { base, alternate: None, can_double: false }
    }
}

impl From<TehtaSpec> for Tehta {
    fn from(spec: TehtaSpec) -> Self {
        Self {
            base: spec.base,
            alternate: spec.alternate,
            can_double: spec.can_double,
        }
    }
}


#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CharOr<T> {
    Char(char),
    Struct(T),
}

impl<T: From<char> + Sized> CharOr<T> {
    pub fn resolve(self) -> T {
        match self {
            Self::Char(c) => c.into(),
            Self::Struct(t) => t,
        }
    }
}


/// A runtime-defined mode of the Tengwar, which may be specified in a file.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ModeFile {
    pub chunks: usize,
    #[serde(default)]
    pub vowels_first: bool,

    pub checks_mod: Vec<Check>,
    pub checks_new: Vec<Check>,

    #[serde(default)]
    pub allow_rince: bool,
    #[serde(default)]
    pub allow_nuquernar: bool,

    #[serde(default, alias = "consonant")]
    pub consonants: HashMap<String, CharOr<GlyphSpec>>,

    #[serde(default, alias = "vowel")]
    pub vowels: HashMap<String, CharOr<TehtaSpec>>,
}

impl ModeFile {
    pub fn resolve(&self) -> CustomMode {
        CustomMode {
            chunks: self.chunks,
            vowels_first: self.vowels_first,
            checks_mod: self.checks_mod.clone(),
            checks_new: self.checks_new.clone(),
            consonants: self.consonants.iter()
                .map(|(s, &g)| (s.chars().collect(), g.resolve()))
                .collect(),
            vowels: self.vowels.iter()
                .map(|(s, &t)| (s.chars().collect(), t.resolve()))
                .collect(),
            current: None,
            previous: None,
        }
    }
}
