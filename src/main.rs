use clap::Parser;
use std::env;

use polars::frame::DataFrame;
use stupidf::{data::STDF, test_information::FullTestInformation};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    // print record information during construction
    #[arg(short, long)]
    verbose: bool,

    // convert into and print out the dataframe
    #[arg(short, long)]
    df: bool,

    // print record summary information
    #[arg(short, long)]
    summarize: bool,
    fname: String,
}

fn polars_config() {
    unsafe {
        env::set_var("POLARS_FMT_MAX_COLS", "20");
        env::set_var("POLARS_FMT_MAX_ROWS", "-1");
        env::set_var("POLARS_TABLE_WIDTH", "300");
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let fname = cli.fname;
    let verbose = cli.verbose;
    let verbose_df = cli.df;
    let summarize = cli.summarize;

    polars_config();

    if let Ok(stdf) = STDF::from_fname(&fname, verbose) {
        if verbose {
            println!("{stdf:#?}");
        }
        if verbose_df {
            let df_fmti: DataFrame = (&stdf.test_data.test_information).into();
            let df_fmti_sorted = df_fmti.sort(["test_num"], Default::default()).unwrap();
            println!("{df_fmti_sorted}");
            let df: DataFrame = (&stdf.test_data).into();
            println!("{df:#?}");
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
