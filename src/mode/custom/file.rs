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


#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
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


#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct TehtaSpec {
    pub base: char,
    pub alt: Option<char>,

    #[serde(default)]
    pub can_double: bool,
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
    pub consonants: HashMap<String, GlyphSpec>,

    #[serde(default, alias = "vowel")]
    pub vowels: HashMap<String, TehtaSpec>,
}

impl ModeFile {
    pub fn resolve(&self) -> CustomMode {
        CustomMode {
            chunks: self.chunks,
            vowels_first: self.vowels_first,
            checks_mod: self.checks_mod.clone(),
            checks_new: self.checks_new.clone(),
            consonants: self.consonants.iter()
                .map(|(s, g)| (s.chars().collect(), *g))
                .collect(),
            vowels: self.vowels.iter()
                .map(|(s, g)| (s.chars().collect(), *g))
                .collect(),
            current: None,
            previous: None,
        }
    }
}
