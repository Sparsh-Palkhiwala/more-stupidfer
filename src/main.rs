use clap::Parser;

use polars::frame::DataFrame;
use stupidf::data::STDF;

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

    if let Ok(stdf) = STDF::from_fname(&fname, verbose) {
        let df: DataFrame = (&stdf.test_data).into();
        let df_fmti: DataFrame = (&stdf.test_data.test_information).into();
        if verbose {
            println!("{stdf:#?}");
            println!("{df:#?}");
            println!("{df_fmti}");
        }
    } else {
        eprintln!("Failed to parse file {fname}");
        Err("Failed to parse file {fnames}")?;
    }

    Ok(())
}
