use clap::Parser;


#[derive(Debug, Parser)]
pub struct GenPassOpts{
    #[arg(short,long, default_value_t = 16, help = "密码长度")]
    pub length: usize,
    #[arg(short,long, default_value_t = false, help = "是否包含特殊字符")]
    pub special: bool,
    #[arg(short,long, default_value_t = false, help = "是否包含数字")]
    pub number: bool,
    #[arg(short,long, default_value_t = false, help = "是否包含大写字母")]
    pub upper: bool,
    #[arg (long, default_value_t = false, help = "是否包含小写字母")]
    pub lower: bool,
}
