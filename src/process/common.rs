use anyhow::{self, Ok,Result};
// use anyhow;
use std::{fs::File, io::Read};
use encoding_rs::UTF_16LE;

pub fn get_reader(input: &str) -> Result<Box<dyn Read>>{
    let reader: Box<dyn Read> = if input == "-"{
        Box::new(std::io::stdin())
    }else{
        Box::new(File::open(input)?)
    };
    Ok(reader)
}

pub fn read_file(file_path: &str ) -> Result<String> {
    // let file_path = "zcx.txt";
    let  output: String;
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
        output = str.to_string();

    } else {
        //如果不是UTF-16LE，尝试使用UTF-8
        let str = String::from_utf8_lossy(&file);
        //println!("文件内容: {}", str);
        let str = str.trim();

        
        output = str.to_string();
    }
    Ok(output)
}

//UTF-8编码和UTF-16LE编码转换成字符串
pub fn decode_to_string(data: &[u8]) -> Result<String> {
    if data.len() >= 2 && data[0] == 0xFF && data[1] == 0xFE {
        // 使用encoding_rs库解码UTF-16LE
        let (cow, _, _) = UTF_16LE.decode(&data[2..]); // 跳过BOM标记
        let str = cow.trim();
        let text = str.to_string();
        Ok(text)
    }else{      
        let str = String::from_utf8_lossy(data);
        let str = str.trim();  
        let text = str.to_string();
        Ok(text)
    }
}