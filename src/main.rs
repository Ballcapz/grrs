use structopt::StructOpt;
use std::fs::File;
use std::io::{BufReader, BufRead};
use anyhow::{Context, Result};


/// Search for a pattern in a file and display the lines that contain it
#[derive(StructOpt)]
struct Cli {
    /// the pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::from_args();

    let f = File::open(&args.path)
        .with_context(|| format!("could not read file `{:?}`", &args.path))?;
    let reader = BufReader::new(f);


    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())

}
