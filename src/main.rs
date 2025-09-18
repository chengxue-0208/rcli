use clap::Parser;
use anyhow;
use rcli::cli::opts::{Opts,SubCommand};
use rcli::process::csv_convert::process_csv;
use rcli::process::gen_pass::process_gen_pass;
use rcli::cli::base64_opts::Base64Cmd;
use rcli::process::base64::{base64_decode,base64_encode};
fn main()-> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd{
        SubCommand::Csv(opts) => process_csv(&opts.input, &opts.output, &opts.format)?,
        SubCommand::GenPass(opts) =>  process_gen_pass(&opts)?, 
        SubCommand::Base64(subcmd) => match subcmd {
            Base64Cmd::Encode(opts) => base64_encode(&opts.input, opts.format)?,
            Base64Cmd::Decode(opts) => base64_decode(&opts.input, opts.format)?,
        },
    }
    Ok(())
}
