use serde::Deserialize;

#[derive(Deserialize)]
pub struct Meta {
    pub lang: String, // "zh" or "en"
    pub en_font: String,
    pub cjk_font: String,
}

#[derive(Deserialize)]
pub struct CvProfile {
    pub name: String,
    pub email: String,
    pub phone: String,
    #[serde(default)]
    pub website: String,
    #[serde(default)]
    pub github: String,
}

#[derive(Deserialize)]
pub struct CvHeading {
    pub title: String,
    #[serde(default)]
    pub subtitle: String,
    #[serde(default)]
    pub location: String,
    #[serde(default)]
    pub start: Option<String>,
    #[serde(default)]
    pub end: Option<String>,
    #[serde(default)]
    pub bullets: Vec<CvBullet>,
}

#[derive(Deserialize)]
pub struct CvSection {
    pub title: String,
    #[serde(default)]
    pub headings: Vec<CvHeading>,
    #[serde(default)]
    pub bullets: Vec<CvBullet>,
}

#[derive(Deserialize)]
pub struct CvBullet {
    pub content: String,
}

impl CvItem {
    pub fn to_latex(&self) -> &str {
        &self.content
    }
}

#[derive(Deserialize)]
pub struct CV {
    pub meta: Meta,
    pub profile: CvProfile,
    #[serde(default)]
    pub sections: Vec<CvSection>,
}

// ----- date formatting helpers -----

struct YearMonth {
    year: i32,
    month: u8,
}

fn parse_year_month(s: &str) -> Option<YearMonth> {
    let (y, m) = s.split_once('-')?;
    Some(YearMonth {
        year: y.parse().ok()?,
        month: m.parse().ok()?,
    })
}

fn format_year_month(ym: &YearMonth, lang: &str) -> String {
    const MONTHS_EN: [&str; 12] = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];

    match lang {
        "zh" => format!("{}.{}", ym.year, format!("{:02}", ym.month)),
        _ => format!("{} {}", MONTHS_EN[(ym.month - 1) as usize], ym.year),
    }
}

pub fn format_range(start: Option<&str>, end: Option<&str>, lang: &str) -> String {
    let s = start.and_then(parse_year_month);
    let e = end.and_then(parse_year_month);

    match (s, e, lang) {
        (Some(s), Some(e), _) => {
            format!(
                "{} -- {}",
                format_year_month(&s, lang),
                format_year_month(&e, lang)
            )
        }
        (Some(s), None, "zh") => format!("{} -- 至今", format_year_month(&s, lang)),
        (Some(s), None, _) => format!("{} -- Present", format_year_month(&s, lang)),
        _ => String::new(),
    }
}
