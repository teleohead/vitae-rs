/*
    model.rs
    This represents user-authored input data.
 */
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Meta {
    pub lang: String,
    pub en_font: String,
    pub cjk_font: String,
    pub font_size: u8,
}

#[derive(Debug, Deserialize)]
pub struct RawCv {
    pub meta: Meta,
    pub headline: RawHeadline,
    #[serde(default)]
    pub sections: Vec<RawSection>,
}

#[derive(Debug, Deserialize)]
pub struct RawHeadline {
    pub name: String,
    pub email: String,
    pub phone: String,
    #[serde(default)]
    pub website: String,
    #[serde(default)]
    pub github: String
}
#[derive(Debug, Deserialize)]
pub struct RawSection {
    pub title: String,
    pub items: Vec<RawItem>,
}

#[derive(Debug, Deserialize)]
pub struct RawItem {
    pub heading: Option<RawItemHeading>,
    pub bullets: Option<Vec<RawItemBullet>>,
}

#[derive(Debug, Deserialize)]
pub struct RawItemHeading {
    pub title: String,
    #[serde(default)]
    pub subtitle: String,
    #[serde(default)]
    pub location: String,
    #[serde(default)]
    pub start_date: String,
    #[serde(default)]
    pub end_date: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RawItemBullet {
    pub content: String,
}