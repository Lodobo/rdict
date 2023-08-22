use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Serialize, Deserialize)]
pub struct Word<'a> {
    #[serde(borrow)]
    pub word: Cow<'a, str>,
    #[serde(borrow)]
    pub pos: Cow<'a, str>,
}

#[derive(Serialize, Deserialize)]
pub struct WordInfo<'a> {
    pub senses: Vec<Sense<'a>>,
    #[serde(borrow)]
    pub etymology_text: Option<Cow<'a, str>>,
    pub sounds: Option<Vec<Sound<'a>>>,
}

#[derive(Serialize, Deserialize)]
pub struct Sound<'a> {
    #[serde(borrow)]
    pub ipa: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub tags: Option<Vec<Cow<'a, str>>>,
}

#[derive(Serialize, Deserialize)]
pub struct Sense<'a> {
    #[serde(borrow)]
    pub glosses: Option<Vec<Cow<'a, str>>>,
    pub examples: Option<Vec<Example<'a>>>,
}

#[derive(Serialize, Deserialize)]
pub struct Example<'a> {
    #[serde(borrow)]
    pub text: Option<Cow<'a, str>>,
}

pub struct Row {
    pub word: String,
    pub pos: String,
    pub information: Option<String>,
}
