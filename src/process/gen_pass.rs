use crate::cli::genpass_opts::GenPassOpts;
use rand::Rng;

pub fn process_gen_pass(opts: &GenPassOpts) -> anyhow::Result<()>{
    let mut rng = rand::rng();
    let mut pass = String::new();
    let mut chars: String = String::new();
    if opts.special{
        chars.push_str("!@#$%^&*()_+-=[]{}|;:',.<>/?");
    }
    if opts.number{
        chars.push_str("0123456789");
    }
    if opts.upper{
        chars.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }
    if opts.lower{
        chars.push_str("abcdefghijklmnopqrstuvwxyz");
    }
    if chars.is_empty(){
        chars.push_str("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()_+-=[]{}|;:',.<>/?");
    }
    for _ in 0..opts.length{
        pass.push(chars.chars().nth(rng.random_range(0..chars.len())).unwrap());
    }
    println!("{}", pass);
    Ok(())
}