use clap::Parser;

use polars::frame::DataFrame;
use stupidf::data::{STDF, STDFDataFrame};

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
        if verbose {
            println!("{stdf:#?}");
        }
        let df = STDFDataFrame::new(&stdf.test_data);
        println!("{df:#?}");

        let df_fmti: DataFrame = (&stdf.test_data.test_information).into();
        println!("{df_fmti}");
    } else {
        eprintln!("Failed to parse file {fname}");
        Err("Failed to parse file {fnames}")?;
    }

    Ok(())
}
