use std::path::Path;
use clap::Parser;

fn verify_input_file(fi: &str) -> Result<String,String>{
    if Path::new(fi).exists(){
        Ok(fi.to_string())
    }else{
        Err(format!("File {} not exists",fi))
    }
}

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
}
#[derive(Debug, Parser)]
pub struct CsvOpts{
    #[arg(short,long, value_parser = verify_input_file)]
    pub input: String,

    #[arg(short,long, default_value = "output.json")]
    pub output: String,

    #[arg(short,long,default_value_t = ',')]
    pub delimiter: char,

     #[arg(long, default_value_t = true)]
     pub header: bool,  
}
