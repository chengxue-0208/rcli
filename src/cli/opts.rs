use clap::Parser;
use super::csv_opts::CsvOpts;
use super::genpass_opts::GenPassOpts;
use super::base64_opts::Base64Cmd;


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
    

}

use std::path::Path;
pub fn verify_input_file(fi: &str) -> Result<String,&'static str>{
    if fi == "-" || Path::new(fi).exists() {
        Ok(fi.to_string())
    }else{
        Err("File dose not exit")
    }
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_verify_input_file(){
        assert_eq!(verify_input_file("-"), Ok('-'.into()));
        assert_eq!(verify_input_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_input_file("csdfvs"), Err("File dose not exit"));
    }
}



