use std::fs;
use std::process::Command;

use anyhow::{bail, Context, Result};
use serde::Serialize;
use tempfile::tempdir;
use tera::{Context as TeraContext, Tera};

mod model, view;
use crate::model::{format_range, CV};
