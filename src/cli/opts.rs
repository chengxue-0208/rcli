use clap::Parser;
use super::csv_opts::CsvOpts;
use super::genpass_opts::GenPassOpts;


#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts{
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand{
    #[command(name = "csv",  about = "Show CSV , or Convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass",  about = "generate a password")]
    GenPass(GenPassOpts),
}


