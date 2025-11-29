use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{bail, Context, Result};
use tempfile::tempdir;
use tera::{Context as TeraContext, Tera};

mod model;
mod view;

use crate::model::CV;
use crate::view::build_view;

fn main() -> Result<()> {
    // ----- 1. Parse CLI argument: vitae <cv.yaml> -----
    let yaml_arg = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: vitae <cv.yaml>");
        std::process::exit(1);
    });

    let yaml_path = Path::new(&yaml_arg);

    // Derive output PDF name: my_cv.yaml -> my_cv.pdf
    let stem = yaml_path
        .file_stem()
        .unwrap_or_else(|| std::ffi::OsStr::new("cv"));
    let out_pdf: PathBuf = yaml_path.with_file_name(format!("{}.pdf", stem.to_string_lossy()));

    // Template is still fixed for now
    const TEMPLATE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/templates/cv.tex.tera");

    // ----- 2. Read YAML -----
    let yaml = fs::read_to_string(&yaml_path)
        .with_context(|| format!("Failed to read data file {}", yaml_path.display()))?;
    let cv: CV = serde_yaml::from_str(&yaml).context("Failed to parse YAML into CV")?;

    // ----- 3. Build view model (dates + bullet strings) -----
    let view = build_view(&cv);

    // ----- 4. Load template into Tera -----
    let template_source = fs::read_to_string(TEMPLATE_PATH)
        .with_context(|| format!("Failed to read template {TEMPLATE_PATH}"))?;

    let mut tera = Tera::default();
    tera.add_raw_template("cv", &template_source)
        .context("Failed to add template to Tera")?;

    let context = TeraContext::from_serialize(&view)
        .context("Failed to convert view CV into template context")?;

    let tex_source = tera
        .render("cv", &context)
        .context("Failed to render LaTeX template")?;

    // ----- 5. Write cv.tex to a temporary directory -----
    let tmpdir = tempdir().context("Failed to create temporary directory")?;
    let tex_path = tmpdir.path().join("cv.tex");
    fs::write(&tex_path, tex_source).context("Failed to write cv.tex")?;

    // If you’re using a .cls, copy it into the temp dir here, e.g.:
    // fs::copy("yzcv.cls", tmpdir.path().join("yzcv.cls"))
    //     .context("Failed to copy yzcv.cls into temp dir")?;

    // ----- 6. Run latexmk -xelatex -----
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

    // ----- 7. Copy resulting PDF next to the YAML -----
    let pdf_src = tmpdir.path().join("cv.pdf");
    fs::copy(&pdf_src, &out_pdf)
        .with_context(|| format!("Failed to copy PDF to {}", out_pdf.display()))?;

    println!("Wrote {}", out_pdf.display());
    Ok(())
}
