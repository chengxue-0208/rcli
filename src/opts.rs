use std::path::Path;
use clap::Parser;
use std::str::FromStr;

fn verify_input_file(fi: &str) -> Result<String,String>{
    if Path::new(fi).exists(){
        Ok(fi.to_string())
    }else{
        Err(format!("File {} not exists",fi))
    }
}


#[derive(Debug,Clone,Copy)]
pub enum OutputFormat{
    Json,
    Yaml,
    // Toml,
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

    #[arg(short,long, value_parser = verify_output_format)]
    pub format: OutputFormat,

    #[arg(short,long,  default_value = "output.json")]
    pub output: String,


    #[arg(short,long,default_value_t = ',')]
    pub delimiter: char,

     #[arg(long, default_value_t = true)]
     pub header: bool,  
}
impl From<OutputFormat> for &'static str{
    fn from(value: OutputFormat) -> Self {
        match value{
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
            // OutputFormat::Toml => "toml",
        }
    }
}
impl FromStr for OutputFormat{
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str(){
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            // "toml" => Ok(OutputFormat::Toml),
            _ => Err(anyhow::anyhow!("不支持的输出格式")),
        }
    }
}
fn verify_output_format(fi: &str) -> Result<OutputFormat,anyhow::Error>{
    fi.parse()
}


