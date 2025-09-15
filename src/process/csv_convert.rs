use csv::Reader;
use anyhow;
use serde::{Deserialize, Serialize};
use serde_json::{self, Value};
use crate::cli::csv_opts::OutputFormat;

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



pub fn process_csv(input: &str, output: &str, format: &OutputFormat)-> anyhow::Result<()>{
    let mut rdr = Reader::from_path(input)?;
    let mut ret: Vec<Value> = Vec::with_capacity(128);
    println!("{:?}", format);
    let headers = rdr.headers()?.clone();
    for result in rdr.records(){
        let record = result?;
        let json_value = headers.iter().zip(record.iter()).
        collect::<Value>();
        ret.push(json_value);
    }
    let content = match format{
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
        // OutputFormat::Toml => toml::to_string(&ret)?,
    };
    std::fs::write(output, content)?;
    Ok(())

}