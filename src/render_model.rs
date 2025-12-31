/*
    render_model
    This represents a model to be passed into Tera for rendering.
 */

use serde::Serialize;
use crate::model;
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

