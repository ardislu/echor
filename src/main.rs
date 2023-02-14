use clap::Parser;

/// Display a line of text
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The text to echo
    text: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    if let Some(string) = cli.text.as_deref() {
        println!("{string}");
    }
}
