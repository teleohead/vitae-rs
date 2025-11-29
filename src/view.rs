use crate::model::{format_range, CV};
use regex::Regex;
use serde::Serialize;

#[derive(Serialize)]
struct ViewMeta {
    lang: String,
    en_font: String,
    cjk_font: String,
    font_size: u8,
}

#[derive(Serialize)]
struct ViewProfile {
    name: String,
    email: String,
    phone: String,
    website: String,
    github: String,
}

#[derive(Serialize)]
struct ViewHeading {
    title: String,
    subtitle: String,
    location: String,
    date: String,
    bullets: Vec<String>,
}

#[derive(Serialize)]
struct ViewSection {
    title: String,
    headings: Vec<ViewHeading>,
    bullets: Vec<String>,
}

#[derive(Serialize)]
pub struct ViewCV {
    meta: ViewMeta,
    profile: ViewProfile,
    sections: Vec<ViewSection>,
}

pub fn build_view(cv: &CV) -> ViewCV {
    let lang = cv.meta.lang.as_str();

    let sections = cv
        .sections
        .iter()
        .map(|sec| {
            let headings = sec
                .headings
                .iter()
                .map(|h| {
                    let date = format_range(h.start.as_deref(), h.end.as_deref(), lang);
                    let bullets = h
                        .bullets
                        .iter()
                        .map(|b| b.content.clone())
                        .collect::<Vec<_>>();

                    ViewHeading {
                        title: h.title.clone(),
                        subtitle: h.subtitle.clone(),
                        location: h.location.clone(),
                        date,
                        bullets,
                    }
                })
                .collect::<Vec<_>>();

            let bullets = sec
                .bullets
                .iter()
                .map(|b| markdown_to_latex(&b.content))
                .collect::<Vec<_>>();

            ViewSection {
                title: sec.title.clone(),
                headings,
                bullets,
            }
        })
        .collect::<Vec<_>>();

    ViewCV {
        meta: ViewMeta {
            lang: cv.meta.lang.clone(),
            en_font: cv.meta.en_font.clone(),
            cjk_font: cv.meta.cjk_font.clone(),
            font_size: cv.meta.font_size.clone(),
        },
        profile: ViewProfile {
            name: cv.profile.name.clone(),
            email: cv.profile.email.clone(),
            phone: cv.profile.phone.clone(),
            website: cv.profile.website.clone(),
            github: cv.profile.github.clone(),
        },
        sections,
    }
}

fn markdown_to_latex(s: &str) -> String {
    // Start with the original text
    let mut out = s.to_string();

    // **bold** → \textbf{...}
    // non-greedy match between **...**
    let re_bold = Regex::new(r"\*\*(.+?)\*\*").unwrap();
    out = re_bold.replace_all(&out, r"\\textbf{$1}").into_owned();

    // *italic* → \emph{...}
    // simple: match a single * ... * where inside has no other *
    // this is safe because **...** is already gone from the string above
    let re_italic = Regex::new(r"\*([^*]+)\*").unwrap();
    out = re_italic.replace_all(&out, r"\\emph{$1}").into_owned();

    // `code` → \texttt{...}
    let re_code = Regex::new(r"`(.+?)`").unwrap();
    out = re_code.replace_all(&out, r"\\texttt{$1}").into_owned();

    out
}
