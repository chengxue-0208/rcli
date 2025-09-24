use clap::Parser;
// use crate::cli::opts::verify_file;
// use std::str::FromStr;
// use std::fmt;
use std::path::PathBuf;
use crate::cli::opts::verify_path;

#[derive(Debug, Parser)]
pub enum HttpCmd{
    #[command(name = "server",  about = "server a directoory over http")]
    Server(ServerOpts),

}
#[derive(Debug, Parser)]
pub struct ServerOpts{      
    #[arg(short,long, default_value = "8080", help = "输入http server的端口")]
    pub port: u16,
    #[arg( long, value_parser= verify_path, default_value = ".", help = "输入http server的根目录")]
    pub path: PathBuf,

 }
// #[derive(Debug, Parser)]
// pub struct VerifyOpts{
//     #[arg(short,long, value_parser = verify_file, default_value = "-", help = "输入一个字符串")]
//     pub input: String,
//     #[arg(short, long, value_parser = verify_file, default_value = "-", help = "输入一个加密的key")]
//     pub key: String,
//     #[arg( long, default_value = "-", help = "输入一个signature")]
//     pub sig: String,
//     #[arg(long,default_value="blake3",value_parser=parse_format, help = "输入一个加密格式")]
//     pub format: TextSignFormat,
//  }

// #[derive(Debug, Clone,Copy)]
// pub enum TextSignFormat {
//     Blake3,
//     Ed25519,
// }

// fn parse_format(s: &str) -> Result<TextSignFormat,anyhow::Error>{
//     s.parse()
// }

// impl From<TextSignFormat> for &'static str{
//     fn from(value: TextSignFormat) -> Self {
//         match value{
//             TextSignFormat::Blake3 => "blake3",
//             TextSignFormat::Ed25519 => "ed25519",
//         }
//     }
// }
// impl FromStr for TextSignFormat{
//     type Err = anyhow::Error;
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s.to_lowercase().as_str(){
//             "blake3" => Ok(TextSignFormat::Blake3),
//             "ed25519" => Ok(TextSignFormat::Ed25519),
//             _ => Err(anyhow::anyhow!("不支持的加密格式")),
//         }
//     }
// }

// impl fmt::Display for TextSignFormat {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", Into::<&str>::into(self.clone()))
//     }
// }
