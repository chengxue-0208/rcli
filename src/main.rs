use clap::Parser;
use anyhow;
use rcli::cli::opts::{Opts,SubCommand};
use rcli::process::csv_convert::process_csv;
use rcli::process::gen_pass::process_gen_pass;
use rcli::cli::base64_opts::Base64Cmd;
use rcli::process::base64::{base64_decode,base64_encode};
use rcli::process::text::{process_text_sign,process_text_verify};
use rcli::cli::text_opts::TextCmd;
use rcli::cli::http_opts::HttpCmd;
use rcli::process::http_server::process_http_server;

#[tokio::main]
async fn main()-> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let opts = Opts::parse();
    match opts.cmd{
        SubCommand::Csv(opts) => process_csv(&opts.input, &opts.output, &opts.format)?,
        SubCommand::GenPass(opts) => {
            let passwd = process_gen_pass(&opts)?;
            println!("{}",passwd)
        }, 
        SubCommand::Base64(subcmd) => match subcmd {
            Base64Cmd::Encode(opts) => base64_encode(&opts.input, opts.format)?,
            Base64Cmd::Decode(opts) => base64_decode(&opts.input, opts.format)?,
        },
        SubCommand::Text(subcmd) => match subcmd {
            TextCmd::Sign(opts) => process_text_sign(&opts)?,
            TextCmd::Verify(opts) => process_text_verify(&opts)?,
        },
        SubCommand::Http(subcmd) => match subcmd {
            HttpCmd::Server(opts) => {
                // println!("http server start at {}:{}", "127.0.0.1", opts.port);
                // println!("http server root dir: {:?}", opts.path);
                process_http_server(&opts).await?;
            }
        }
    }
    Ok(())
}
