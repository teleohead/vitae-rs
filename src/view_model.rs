/*
   view_model
   This represents a model to be passed into Tera for rendering.
*/

use crate::model;
use serde::Serialize;

pub fn render(cv: &model::RawCv) -> ViewCv {
    let lang = &cv.meta.lang;
    ViewCv {
        meta: cv.meta.clone(),
        headline: render_headline(&cv.headline),
        sections: cv
            .sections
            .iter()
            .map(|s| render_section(s, lang))
            .collect(),
    }
}

fn render_headline(headline: &model::RawHeadline) -> ViewHeadline {
    ViewHeadline {
        name: headline.name.clone(),
        email: headline.email.clone(),
        phone: headline.phone.clone(),
        website: headline.website.clone(),
        github: headline.github.clone(),
    }
}

fn render_section(section: &model::RawSection, lang: &str) -> ViewSection {
    ViewSection {
        title: section.title.clone(),
        items: section.items.iter().map(|i| render_item(i, lang)).collect(),
    }
}

fn render_item(item: &model::RawItem, lang: &str) -> ViewItem {
    let anchor = match &item.heading {
        Some(h) => ViewAnchor::Heading(ViewItemHeading {
            title: h.title.clone(),
            subtitle: h.subtitle.clone(),
            location: h.location.clone(),
            date_range: format_range(&h.start_date, h.end_date.as_deref(), lang),
        }),
        None => ViewAnchor::BareItem,
    };

    let bullets = item
        .bullets
        .as_ref()
        .map(|bs| {
            bs.iter()
                .map(|b| ViewItemBullet {
                    content: markdown_to_latex(&b.content),
                })
                .collect()
        })
        .unwrap_or_default();

    ViewItem { anchor, bullets }
}

/* Markdown to LaTeX conversion function */
fn markdown_to_latex(s: &str) -> String {
    /* TODO */
    s.to_string()
}

#[derive(Debug, Serialize)]
pub struct ViewCv {
    /* let's pass through meta */
    pub meta: model::Meta,
    pub headline: ViewHeadline,
    pub sections: Vec<ViewSection>,
}

#[derive(Debug, Serialize)]
pub struct ViewHeadline {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub website: String,
    pub github: String,
}

#[derive(Debug, Serialize)]
pub struct ViewSection {
    pub title: String,
    pub items: Vec<ViewItem>,
}

#[derive(Debug, Serialize)]
pub struct ViewItem {
    pub anchor: ViewAnchor,
    pub bullets: Vec<ViewItemBullet>,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum ViewAnchor {
    Heading(ViewItemHeading),
    BareItem,
}

#[derive(Debug, Serialize)]
pub struct ViewItemHeading {
    pub title: String,
    pub subtitle: String,
    pub location: String,
    pub date_range: String,
}

#[derive(Debug, Serialize)]
pub struct ViewItemBullet {
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

fn format_range(start: &str, end: Option<&str>, lang: &str) -> String {
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
