use clap::Parser;

use stupidf::data::TestData;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    verbose: bool,
    fname: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let fname = cli.fname;
    let verbose = cli.verbose;

    if let Ok(test_data) = TestData::from_fname(&fname, verbose) {
        if verbose {
            println!("{test_data:#?}");
        }
    } else {
        eprintln!("Failed to parse file {fname}");
        Err("Failed to parse file {fnames}")?;
    }

    Ok(())
}
