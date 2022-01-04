use crate::config::Config;
use crate::encryption::Encryption;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct File {
    /// Path to the scoob configuration file
    #[structopt(parse(from_os_str))]
    config: PathBuf,

    #[structopt(subcommand)]
    cmd: FileMode,
}

#[derive(Debug, StructOpt)]
struct EncryptOptions {
    /// The encryption key from the config file that will be used
    #[structopt(short, long, default_value = "*")]
    key: String,

    /// The raw, unencrypted file
    #[structopt(parse(from_os_str))]
    from: PathBuf,

    /// File that the encrypted file will be encrypted to
    #[structopt(parse(from_os_str))]
    to: PathBuf,
}

#[derive(Debug, StructOpt)]
struct DecryptOptions {
    /// The encryption key from the config file that will be used
    #[structopt(short, long, default_value = "*")]
    key: String,

    /// The encrypted file
    #[structopt(parse(from_os_str))]
    from: PathBuf,

    /// File that the encrypted file will be decrypted to
    #[structopt(parse(from_os_str))]
    to: PathBuf,
}

#[derive(Debug, StructOpt)]
enum FileMode {
    Encrypt(EncryptOptions),
    Decrypt(DecryptOptions),
}

fn encrypt_file(config: &Config, options: &EncryptOptions) -> Result<(), &'static str> {
    let encryption = Encryption { config };
    let raw_contents = match std::fs::read_to_string(&options.from) {
        Ok(c) => c,
        Err(_) => return Err("Unable to read file"),
    };

    let encrypted_contents = encryption.encrypt(&options.key, &raw_contents)?;
    std::fs::write(&options.to, &encrypted_contents).unwrap();
    Ok(())
}

fn decrypt_file(config: &Config, options: &DecryptOptions) -> Result<(), &'static str> {
    let encryption = Encryption { config };
    let encrypted_contents = match std::fs::read_to_string(&options.from) {
        Ok(c) => c,
        Err(_) => return Err("Unable to read encrypted file"),
    };

    let raw_contents = encryption.decrypt(&options.key, &encrypted_contents)?;
    std::fs::write(&options.to, &raw_contents).unwrap();
    Ok(())
}

pub fn file(cmd: &File) -> Result<(), &'static str> {
    if !Config::exists(&cmd.config) {
        return Err("The provided configuration file does not exist");
    }

    let config = Config::get(&cmd.config);

    match &cmd.cmd {
        FileMode::Encrypt(options) => encrypt_file(&config, options),
        FileMode::Decrypt(options) => decrypt_file(&config, options),
    }
}

#[cfg(test)]
mod tests {
    // TODO: Write tests
}
