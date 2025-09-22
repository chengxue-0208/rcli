use crate::cli::text_opts::{SignOpts, TextSignFormat, VerifyOpts};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use rand::rngs::OsRng;
use super::common::*;
use anyhow::{Ok, Result};
use anyhow;
use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
use super::gen_pass::process_gen_pass;
use crate::cli::genpass_opts::GenPassOpts;
use std::io::Read;
// use rand::rngs::OsRng;
//use rand_core::{TryRngCore, OsRng,CryptoRng };
// use ed25519_dalek::SigningKey;
// use ed25519_dalek::Signature;


trait TextSign {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}
trait TextVerify {
    fn verify(&self, reader: impl  Read, sig: &[u8]) -> Result<bool>;
}
pub struct Blake3{
 key: [u8;32],   
}

struct Ed25519Signer{
 key: SigningKey,   
}
struct Ed25519Verifier{
 key: VerifyingKey,   
}
impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>{
        println!("进入签名");
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        println!("退出签名");
        Ok(blake3::hash(&buf).as_bytes().to_vec())

    }
}
impl TextVerify for Blake3 {
    fn verify(&self, mut reader: impl Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        println!("BUF的内容{:?}",buf);
        let hash =  blake3::hash(&buf);
        let hash  = hash.as_bytes();
        println!("内容重新hash数值:{:?}", hash);
        Ok(hash == sig)
    }
    
}
impl TextSign for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>{
        println!("进入签名");
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        println!("退出签名");
        let sig = self.key.sign(&buf);
        Ok(sig.to_bytes().to_vec())

    }
}

impl TextVerify for Ed25519Verifier {
    fn verify(&self, mut reader: impl Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = Signature::from_bytes(sig.try_into()?);
        let ret = self.key.verify_strict(&buf, &sig ).is_ok();
        Ok(ret)
    }
    
}


pub fn process_text_sign(opts: &SignOpts) -> Result<()>{
    let mut reader = get_reader(&opts.input)?; 
    let signed = match opts.format {
        TextSignFormat::Blake3 => {
        let signer = Blake3::load(&opts.key)?;
        signer.sign(&mut reader)?
        },
        TextSignFormat::Ed25519 => {
            let signer = Ed25519Signer::load(&opts.key)?;
            signer.sign(&mut reader)?
        },        
    };
    // println!("{:?}", signed);
    let signed = URL_SAFE_NO_PAD.encode(&signed);
    println!("{}",signed);
    Ok(())
}

pub fn process_text_verify(opts: &VerifyOpts) -> Result<(),anyhow::Error>{
    let mut reader = get_reader(&opts.input)?; 
    let signed = match opts.format {
        TextSignFormat::Blake3 => {
            let verifier =Blake3::load(&opts.key)?;
            println!("签名解密前{:?}",opts.sig.clone());
            let sig =  URL_SAFE_NO_PAD.decode(opts.sig.clone())?;
            println!("签名解密后{:?}",sig);
            verifier.verify(&mut reader, &sig)?
        },
        TextSignFormat::Ed25519 => {
            let verifier = Ed25519Verifier::load(&opts.key)?;
            let sig =  URL_SAFE_NO_PAD.decode(opts.sig.clone())?;
            verifier.verify(&mut reader, &sig)?
        },        
    };
    println!("比较完成{}",signed);
    Ok(())
}

impl Blake3 {
    pub fn new(key: [u8; 32]) -> Self{
        Self {key}
    }
    pub fn try_new(key: &[u8]) ->Result<Self>{
        let key = &key[..32];
        let key = key.try_into()?;
        let signer = Blake3::new(key);
        Ok(signer)
    }
    pub fn load(key: &str) ->Result<Self>{
        let content = read_file(key)?;
        let buf = content.as_bytes();
        Self::try_new(&buf)

    }
}

impl Ed25519Signer {
    pub fn new(key: SigningKey) -> Self{
        Self {key}
    }
    pub fn try_new(key: &[u8; 32]) ->Result<Self>{
        let key = key;
        let key = SigningKey::from_bytes(key);
        println!("SigningKEY: {:?}",key);
        let signer = Ed25519Signer::new(key);
        Ok(signer)
    }
    pub fn load(key: &str) ->Result<Self>{
        let content = read_file(key)?;
        let buf = content.as_bytes().try_into()?;
        println!("签名读取{:?}", buf);
        Self::try_new(buf)
    }
}

impl Ed25519Verifier {
    pub fn new(key: VerifyingKey) -> Self{
        Self {key}
    }
    pub fn try_new(key: &[u8; 32]) ->Result<Self>{
        let key =key;
        println!("v key {:?}", key);
        let key = VerifyingKey::from_bytes(key)?;
        println!("v key {:?}", key);
        let signer = Ed25519Verifier::new(key);
        Ok(signer)
    }
    pub fn load(key: &str) ->Result<Self>{
        let content = read_file(key)?;
        let buf = content.as_bytes().try_into()?;
        Self::try_new(buf)
    }
}

pub trait KeyGenerator {
    fn generate() -> Result<Vec<Vec<u8>>>;
    
}

impl KeyGenerator for Blake3 {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let opts = GenPassOpts { length: 32,
                                                special: false,
                                                number: false, 
                                                upper: false, 
                                                lower: false };
        let key = process_gen_pass(&opts)?.as_bytes().to_vec();
    Ok(vec![key])
    }
    
}

impl KeyGenerator for Ed25519Signer {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let mut csprng = OsRng;
        // let mut csprng = [0u8; 16];
        
 
        //let signing_key = SigningKey::generate(&mut  csprng);
        //let key = signing_key.to_bytes().to_vec();
        Ok(vec![])
    }    
}
