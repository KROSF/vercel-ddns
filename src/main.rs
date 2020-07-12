mod cli;
mod vercel;

use crate::cli::{run, Args};
use eyre::Result;
use structopt::StructOpt;

fn main() -> Result<()> {
    let args = Args::from_args();

    run(args)
}
