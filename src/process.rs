
use csv::Reader;
use anyhow;
use serde::{Deserialize, Serialize};
use serde_json::{self, Value};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Player{
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, output: &str)-> anyhow::Result<()>{
    let mut rdr = Reader::from_path(input)?;
    let mut ret: Vec<Value> = Vec::with_capacity(128);
    let headers = rdr.headers()?.clone();
    for result in rdr.records(){
        let record = result?;
        let json_value = headers.iter().zip(record.iter()).
        collect::<Value>();
        ret.push(json_value);
    }
    let json = serde_json::to_string_pretty(&ret)?;
    std::fs::write(output, json)?;
    Ok(())
}