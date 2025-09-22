use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use crate::{cli::base64_opts::Base64Format, process::text};
use base64::engine::general_purpose::STANDARD;
use anyhow::{self, Ok,Result};
use super::common::*;
use encoding_rs::UTF_16LE;
// use anyhow;
use std::io::Read;
pub fn base64_encode(input: &str, format: Base64Format) -> Result<()>{
    let mut reader: Box<dyn Read> = get_reader(input).unwrap();
    let mut data = Vec::new();
    reader.read_to_end(&mut data)?;
    
    let encode = match format{
        Base64Format::Standard => STANDARD.encode(&data),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&data),
    };
    println!("{}",encode);
    Ok(())
}


pub fn base64_decode(input: &str, format: Base64Format) -> Result<()>{
    let mut data = String::new();

    let mut reader: Box<dyn Read> = get_reader(input).unwrap();
    if input == "-"{
        let mut buf = String::new();
        reader.read_to_string(&mut buf)?;                                                                                                      
        data = buf.trim().to_string();
    }else{
        data = read_file(input).unwrap();
    }                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        
    let decode = match format{                                                                                                              
        Base64Format::Standard => STANDARD.decode(&data)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(&data)?,                                                                                            
    };
    //println!("base64 decode data: {:?}", decode);
    let text = decode_to_string(&decode)?;
    //println!("base64 decode text: {}", text);
    println!("{}", text);
    Ok(())
}
