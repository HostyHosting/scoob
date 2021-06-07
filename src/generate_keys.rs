use colored::Colorize;
use structopt::StructOpt;

#[derive(Debug, StructOpt, Default)]
pub struct GenerateKeys {}

pub fn generate_keys(_cmd: &GenerateKeys) -> Result<(), &'static str> {
    let (public_key, secret_key) = crate::encryption::Encryption::gen_keypair();

    println!(
        "\nYou may place this key pair in a new key under the `{}` section of your config file.\n",
        "keys".blue().bold()
    );
    println!("  {}:", "your_key_name".green());
    println!("    {}: {}", "publicKey".green(), public_key);
    println!("    {}: {}", "secretKey".green(), secret_key);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_keys() {
        generate_keys(&Default::default()).expect("Should generate keys without erroring.");
    }
}
