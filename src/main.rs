use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Do not output the trailing newline
    #[arg(short, long, action)]
    no_newline: bool,

    /// The string to display
    string: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    if let Some(string) = cli.string.as_deref() {
        if cli.no_newline {
            print!("{string}");
        } else {
            println!("{string}");
        }
    }
}
