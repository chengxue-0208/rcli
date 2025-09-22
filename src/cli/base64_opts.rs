use clap::Parser;
use std::str::FromStr;
use super::opts::verify_file;
use std::fmt;

#[derive(Debug, Parser)]
pub enum Base64Cmd{
    #[command(name = "encode",  about = "base64 encode a string")]
    Encode(Base64EncodeOpts),
    #[command(name = "decode",  about = "base64 decode a string")]
    Decode(Base64DecodeOpts),
}
#[derive(Debug, Parser)]
pub struct Base64EncodeOpts{
    #[arg(short,long, value_parser = verify_file, default_value = "-", help = "input string")]
    pub input: String,
    #[arg(long, value_parser = parse_base64_format, default_value = "standard", help = "base64 format")]
    pub format: Base64Format
}
#[derive(Debug, Parser)]
pub struct Base64DecodeOpts{
    #[arg(short,long, value_parser = verify_file, default_value = "-", help = "input string")]
    pub input: String,
    #[arg(long, value_parser = parse_base64_format, default_value = "standard", help = "base64 format")]
    pub format: Base64Format
}

#[derive(Debug, Clone)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

fn parse_base64_format(s: &str) -> Result<Base64Format,anyhow::Error>{
    s.parse()
}

impl From<Base64Format> for &'static str{
    fn from(value: Base64Format) -> Self {
        match value{
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "urlsafe",
        }
    }
}
impl FromStr for Base64Format{
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str(){
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("不支持的base64格式")),
        }
    }
}

impl fmt::Display for Base64Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(self.clone()))
    }
}
