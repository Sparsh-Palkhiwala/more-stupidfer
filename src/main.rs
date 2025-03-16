use clap::Parser;

use polars::frame::DataFrame;
use stupidf::{data::STDF, test_information::FullTestInformation};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    verbose: bool,
    #[arg(short, long)]
    df: bool,
    #[arg(short, long)]
    summarize: bool,
    fname: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let fname = cli.fname;
    let verbose = cli.verbose;
    let verbose_df = cli.df;
    let summarize = cli.summarize;

    if let Ok(stdf) = STDF::from_fname(&fname, verbose) {
        let df: DataFrame = (&stdf.test_data).into();
        let df_fmti: DataFrame = (&stdf.test_data.test_information).into();
        if verbose {
            println!("{stdf:#?}");
        }
        if verbose_df {
            println!("{df:#?}");
            println!("{df_fmti}");
        }
    } else {
        eprintln!("Failed to parse file {fname}");
        Err("Failed to parse file {fnames}")?;
    }
    if summarize {
        if let Ok((_test_data, summary)) =
            FullTestInformation::from_fname_and_summarize(&fname, verbose)
        {
            println!("{summary:#?}");
        }
    }

    Ok(())
}
