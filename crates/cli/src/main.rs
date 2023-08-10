use august_compiler::{compile, CompilerError, OutputType};
use clap::{Parser, ValueEnum};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum OutputTypeArgument {
    Json,
    Toml,
    Yaml,
}

impl From<OutputTypeArgument> for OutputType {
    fn from(value: OutputTypeArgument) -> Self {
        match value {
            OutputTypeArgument::Json => Self::Json,
            OutputTypeArgument::Toml => Self::Toml,
            OutputTypeArgument::Yaml => Self::Yaml,
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'o', long = "output", value_enum)]
    output_type: OutputTypeArgument,

    #[arg()]
    input: String,
}

#[derive(Error, Debug)]
pub enum CliError {
    #[error(transparent)]
    CompilationError(#[from] CompilerError),
}

fn main() -> Result<(), CliError> {
    let Args { output_type, input } = Args::parse();
    let output = compile(&input, &output_type.into())?;
    println!("{output}");
    Ok(())
}
