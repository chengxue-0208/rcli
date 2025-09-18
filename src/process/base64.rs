use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use crate::cli::base64_opts::Base64Format;
use base64::engine::general_purpose::STANDARD;
use anyhow;
use std::{fs::File, io::{Read, Write}};
use encoding_rs::UTF_16LE;
pub fn base64_encode(input: &str, format: Base64Format) -> anyhow::Result<()>{
    let mut reader: Box<dyn Read> = if input == "-"{
        Box::new(std::io::stdin())
    }else{
        Box::new(File::open(input)?)
    };
    let mut data = Vec::new();
    reader.read_to_end(&mut data)?;
    
    let encode = match format{
        Base64Format::Standard => STANDARD.encode(&data),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&data),
    };
    println!("{}",encode);
    Ok(())
}


pub fn base64_decode(input: &str, format: Base64Format) -> anyhow::Result<()>{
    let mut data = String::new();

    let mut reader: Box<dyn Read> = if input == "-"{
        Box::new(std::io::stdin())
    }else{
        Box::new(File::open(input)?)
    };
    if input == "-"{
        let mut buf = String::new();
        reader.read_to_string(&mut buf)?;                                                                                                      
        data = buf.trim().to_string(); 
    }else{
        let mut output = String::new();
        read_file(input, &mut output);
        data = output;
        
    }                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        
    
    let decode = match format{                                                                                                              
        Base64Format::Standard => STANDARD.decode(&data)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(&data)?,                                                                                            
    };
    println!("{:?}", decode);
    // 处理可能不是UTF-8编码的数据                                                              
    let text = String::from_utf8(decode)?;
    println!("{}", text);                                                                                                                       
    Ok(())
}

fn read_file(file_path: &str, output: &mut String)  {
    // let file_path = "zcx.txt";
    let mut reader = File::open(file_path).expect("文件打开失败");
    let mut file = Vec::new();
    reader.read_to_end(&mut file).expect("文件读取失败");
    //println!("文件内容: {:?}", file);
    
    // 检查是否有UTF-16LE的BOM标记 (FF FE)
    if file.len() >= 2 && file[0] == 0xFF && file[1] == 0xFE {
        // 使用encoding_rs库解码UTF-16LE
        let (cow, _, _) = UTF_16LE.decode(&file[2..]); // 跳过BOM标记
        let str = cow.trim();
       // println!("UTF-16LE解码后的文件内容: {}", str);
        *output = str.to_string();

    } else {
        //如果不是UTF-16LE，尝试使用UTF-8
        let str = String::from_utf8_lossy(&file);
        //println!("文件内容: {}", str);
        let str = str.trim();

        
        *output = str.to_string();
    }
}