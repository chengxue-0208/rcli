use clap::Parser;
use anyhow;
use rcli::{Opts,SubCommand};
use rcli::process_csv;


fn main()-> anyhow::Result<()> {
    let opts = Opts::parse();
    //println!("{:?}" , opts);
    match opts.cmd{
        SubCommand::Csv(opts) => process_csv(&opts.input, &opts.output)?,
    }
    Ok(())
}
