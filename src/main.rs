use std::fs;
use std::process::Command;

use anyhow::{bail, Context, Result};
use tempfile::tempdir;
use tera::{Context as TeraContext, Tera};

mod model;
mod view;
use crate::model::CV;
use crate::view::build_view;

fn main() -> Result<()> {
    // Hard-coded paths for now
    let data_path = "data/cv.yaml";
    let template_path = "templates/cv.tex.tera";
    let out_pdf = "cv.pdf";

    // 1. Read YAML
    let yaml = fs::read_to_string(data_path)
        .with_context(|| format!("Failed to read data file {data_path}"))?;
    let cv: CV = serde_yaml::from_str(&yaml).context("Failed to parse YAML into CV")?;

    // 2. Convert to view model (formatted dates + plain bullet strings)
    let view = build_view(&cv);

    // 3. Load template into Tera
    let template_source = fs::read_to_string(template_path)
        .with_context(|| format!("Failed to read template {template_path}"))?;

    let mut tera = Tera::default();
    tera.add_raw_template("cv", &template_source)
        .context("Failed to add template to Tera")?;

    let context = TeraContext::from_serialize(&view)
        .context("Failed to convert view CV into template context")?;

    let tex_source = tera
        .render("cv", &context)
        .context("Failed to render LaTeX template")?;

    // 4. Write cv.tex to a temporary directory
    let tmpdir = tempdir().context("Failed to create temporary directory")?;
    let tex_path = tmpdir.path().join("cv.tex");
    fs::write(&tex_path, tex_source).context("Failed to write cv.tex")?;

    // 5. Run latexmk -xelatex
    let status = Command::new("latexmk")
        .arg("-xelatex")
        .arg("-interaction=nonstopmode")
        .arg("-halt-on-error")
        .arg(tex_path.file_name().unwrap())
        .current_dir(tmpdir.path())
        .status()
        .context("Failed to execute latexmk")?;

    if !status.success() {
        bail!("latexmk exited with status: {}", status);
    }

    // 6. Copy resulting PDF to project root as cv.pdf
    let pdf_src = tmpdir.path().join("cv.pdf");
    fs::copy(&pdf_src, out_pdf).with_context(|| format!("Failed to copy PDF to {out_pdf}"))?;

    println!("Wrote {out_pdf}");
    Ok(())
}
