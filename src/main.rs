use clap::Parser;
use anyhow;
use rcli::cli::opts::{Opts,SubCommand};
use rcli::process::csv_convert::process_csv;
use rcli::process::gen_pass::process_gen_pass;
fn main()-> anyhow::Result<()> {
    let opts = Opts::parse();
    println!("{:?}" , opts);
    match opts.cmd{
        SubCommand::Csv(opts) => process_csv(&opts.input, &opts.output, &opts.format)?,
        SubCommand::GenPass(opts) =>  process_gen_pass(&opts)?, 
    }
    Ok(())
}
