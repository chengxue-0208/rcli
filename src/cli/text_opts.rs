use clap::Parser;
use crate::cli::opts::verify_file;
use std::str::FromStr;
use std::fmt;

#[derive(Debug, Parser)]
pub enum TextCmd{
    #[command(name = "sign",  about = "")]
    Sign(SignOpts),
    #[command(name = "verify",  about = "")]
    Verify(VerifyOpts),
}
#[derive(Debug, Parser)]
pub struct SignOpts{
    #[arg(short,long, value_parser = verify_file, default_value = "-", help = "输入一个字符串")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file, help = "输入一个加密的key")]
    pub key: String,
    #[arg(long,default_value="blake3",value_parser=parse_format, help = "输入一个加密格式")]
    pub format: TextSignFormat,

 }
#[derive(Debug, Parser)]
pub struct VerifyOpts{
    #[arg(short,long, value_parser = verify_file, default_value = "-", help = "输入一个字符串")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file, default_value = "-", help = "输入一个加密的key")]
    pub key: String,
    #[arg( long, default_value = "-", help = "输入一个signature")]
    pub sig: String,
    #[arg(long,default_value="blake3",value_parser=parse_format, help = "输入一个加密格式")]
    pub format: TextSignFormat,
 }

#[derive(Debug, Clone,Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

fn parse_format(s: &str) -> Result<TextSignFormat,anyhow::Error>{
    s.parse()
}

impl From<TextSignFormat> for &'static str{
    fn from(value: TextSignFormat) -> Self {
        match value{
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}
impl FromStr for TextSignFormat{
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str(){
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(anyhow::anyhow!("不支持的加密格式")),
        }
    }
}

impl fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(self.clone()))
    }
}
