use clap::{Parser, ValueEnum};

// Version constants

const VERSION: &str = env!("CARGO_PKG_VERSION");
const COMMIT: &str = env!("GIT_COMMIT_HASH");
const BUILD_DATE: &str = env!("BUILD_DATE");
const AUTHOR : &str = env!("CARGO_PKG_AUTHORS");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

fn long_version() -> &'static str {
    Box::leak(
        format!(
            "Version: {}\nCommit: {}\nBuild Date: {}",
            VERSION, COMMIT, BUILD_DATE
        )
        .into_boxed_str()
    )
}

// Command line interface

#[derive(Clone, ValueEnum)]
enum Cmd {
    /// Say hello
    Hello,
    /// Say hi
    Hi,
}

#[derive(Parser)]
#[command(
  author = AUTHOR,
  version = VERSION,
  about = DESCRIPTION,
  long_version = long_version()
)]
struct Cli {
    /// Command to execute
    command: Cmd,

    /// Directory containing the images to process
    #[arg(short = 'n', long = "name", required = true)]
    name: String,

}

// main

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Cmd::Hello => println!("Hello, {}!", cli.name),
        Cmd::Hi => println!("Hi, {}!", cli.name),
    }

    Ok(())
}
