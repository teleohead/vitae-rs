/*
   render_model
   This represents a model to be passed into Tera for rendering.
*/

use crate::model;
use serde::Serialize;
#[derive(Debug, Serialize)]
pub struct RenderCv {
    /* let's pass through meta */
    pub meta: model::Meta,
    pub headline: RenderHeadline,
    pub sections: Vec<RenderSection>,
}

#[derive(Debug, Serialize)]
pub struct RenderHeadline {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub website: String,
    pub github: String,
}

#[derive(Debug, Serialize)]
pub struct RenderSection {
    pub title: String,
    pub items: Vec<RenderItem>,
}

#[derive(Debug, Serialize)]
pub struct RenderItem {
    pub anchor: RenderAnchor,
    pub bullets: Vec<RenderItemBullet>,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum RenderAnchor {
    Heading(RenderItemHeading),
    BareItem,
}

#[derive(Debug, Serialize)]
pub struct RenderItemHeading {
    pub title: String,
    pub subtitle: String,
    pub location: String,
    pub date_range: String,
}

#[derive(Debug, Serialize)]
pub struct RenderItemBullet {
    pub content: String,
}

/* date helper structures and functions */
struct YearMonth {
    year: u32,
    month: u8,
}

fn parse_date(s: &str) -> Option<YearMonth> {
    let (y, m) = s.split_once('-')?;
    Some(YearMonth {
        year: y.parse().ok()?,
        month: m.parse().ok()?,
    })
}

fn format_date(ym: &YearMonth, lang: &str) -> String {
    const MONTHS_EN: [&str; 12] = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];

    match lang {
        "zh" => format!("{}.{}", ym.year, ym.month),
        _ => format!("{} {}", MONTHS_EN[(ym.month - 1) as usize], ym.year),
    }
}

pub fn format_range(start: &str, end: Option<&str>, lang: &str) -> String {
    let s = parse_date(start);
    let e = end.and_then(parse_date);

    match (s, e, lang) {
        (Some(s), Some(e), _) => {
            format!("{} -- {}", format_date(&s, lang), format_date(&e, lang))
        }
        (Some(s), None, "zh") => format!("{} 至今", format_date(&s, lang)),
        (Some(s), None, _) => format!("{} -- Present", format_date(&s, lang)),
        _ => String::new(),
    }
}
