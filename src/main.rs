mod cli;
mod vercel;

use crate::cli::{run, Args};
use eyre::Result;
use log::LevelFilter;
use simplelog::{Config, SimpleLogger};
use structopt::StructOpt;

fn main() -> Result<()> {
    SimpleLogger::init(LevelFilter::Error, Config::default())?;

    let args = Args::from_args();

    run(args)
}
