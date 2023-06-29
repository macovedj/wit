use std::{path::Path, fs};

use clap::Args;
use anyhow::{Result};
use depsparser::{DepsParser};

#[derive(Args)]
/// Resolves Dependencies
pub struct ResolveCommand {
    /// The id of the package to add a dependency to.
    #[clap(value_name = "PACKAGE")]
    pub package: String,
    // #[clap(short, long, value_name = "PATH")]
    // pub output: Option<PathBuf>,
}

impl ResolveCommand {
  /// Executes the command.
  pub async fn exec(self) -> Result<()> {
    dbg!(&self.package);
    let file: &Path = self.package.as_ref();
    dbg!(&file);
    let bytes = std::fs::read(file)?;
    // let data = fs::read(path)?;
    let mut depsparser = DepsParser::new();
    let deps = depsparser.parse(&bytes);
    dbg!(deps);
    Ok(())
  }
}