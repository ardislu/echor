use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Do not output the trailing newline
    #[arg(short, long, action)]
    no_newline: bool,

    /// The string to display
    string: Vec<String>,
}

fn main() {
    let cli = Cli::parse();

    let output = cli.string.join(" ");

    if cli.no_newline {
        print!("{output}");
    } else {
        println!("{output}");
    }
}
