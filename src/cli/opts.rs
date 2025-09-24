use clap::Parser;
use super::csv_opts::CsvOpts;
use super::genpass_opts::GenPassOpts;
use super::base64_opts::Base64Cmd;
use super::text_opts::TextCmd;
use super::http_opts::HttpCmd;
use std::path::PathBuf;


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
    #[command(subcommand, name = "base64", about = "base64 encode or decode a string")]
    Base64(Base64Cmd),
    #[command(subcommand, name = "text",  about = "text sign or verify")]
    Text(TextCmd),
    #[command(subcommand, name = "http",  about = "http request")]
    Http(HttpCmd),

}

use std::path::Path;
pub fn verify_file(fi: &str) -> Result<String,&'static str>{
    if fi == "-" || Path::new(fi).exists() {
        Ok(fi.to_string())
    }else{
        Err("File dose not exit")
    }
}

pub fn verify_path(fi: &str) -> Result<PathBuf,&'static str>{
    let p = PathBuf::from(fi);
    if p.exists() {
        Ok(p)
    }else{
        Err("Path dose not exit")
    }
}



