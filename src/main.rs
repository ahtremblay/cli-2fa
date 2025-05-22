use std::collections::BTreeSet;
use anyhow::anyhow;
use clap::{Args, Parser, Subcommand};
use keyring::Entry;
use keyring::Error::NoEntry;
use totp_rs::{Algorithm, Secret, TOTP};

#[derive(Parser, Debug)]
#[command(author, version, about = "A CLI tool to manage 2FA TOTP codes")]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Push(Push),
    Get(Get),
    Delete(Delete),
    List
}


#[derive(Args, Debug)]
struct Push {
    name: String,
    secret: String,
}

#[derive(Args, Debug)]
struct Get {
    name: String,
}

#[derive(Args, Debug)]
struct Delete {
    name: String,
}


const SERVICE_NAME: &str = "rust.twofa-cli";

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    let args = Cli::parse();
    match args.command {
        Commands::Push(push) => {
            let entry = Entry::new(SERVICE_NAME, &push.name)?;
            entry.set_password(&push.secret)?;
            let entry_index = Entry::new(SERVICE_NAME, "index@rust.twofa-cli")?;
            match entry_index.get_password() {
                Ok(json) => {
                    let mut keys:BTreeSet<&str> = serde_json::from_str(&json)?;
                    keys.insert(&push.name);
                    let json = serde_json::to_string(&keys)?;
                    entry_index.set_password(&json)?;
                }
                Err(NoEntry) => {
                    let mut keys:BTreeSet<&str> = BTreeSet::new();
                    keys.insert(&push.name);
                    let json = serde_json::to_string(&keys)?;
                    entry_index.set_password(&json)?;
                }
                Err(e) => {
                    return Err(anyhow!(e));
                }
            }
        }
        Commands::Get(get) => {
            let entry = Entry::new(SERVICE_NAME, &get.name)?;
            let secret_base32 = entry.get_password()?;

            let topt = TOTP::new(
                Algorithm::SHA1, 
                6, 
                1, 
                30,
                Secret::Encoded(secret_base32).to_bytes()?)?;
            let otp = topt.generate_current()?;
            println!("{}", otp);
        }
        Commands::Delete(delete) => {
            let entry = Entry::new(SERVICE_NAME, &delete.name)?;
            entry.delete_credential()?;
            let entry_index = Entry::new(SERVICE_NAME, "index@rust.twofa-cli")?;
            match entry_index.get_password() {
                Ok(json) => {
                    let mut keys:BTreeSet<&str> = serde_json::from_str(&json)?;
                    keys.remove(delete.name.as_str());
                    let json = serde_json::to_string(&keys)?;
                    entry_index.set_password(&json)?;
                }
                Err(NoEntry) => {
                }
                Err(e) => {
                    return Err(anyhow!(e));
                }
            }
        }
        Commands::List => {
            let entry_index = Entry::new(SERVICE_NAME, "index@rust.twofa-cli")?;
            match entry_index.get_password() {
                Ok(json) => {
                    let keys:BTreeSet<&str> = serde_json::from_str(&json)?;
                    let mut i=1;
                    keys.iter().for_each(|key| {
                        println!("{}. {}", i, key);
                        i+=1;
                    })
                }
                Err(NoEntry) => {
                    println!("no entries.")
                }
                Err(e) => {
                    return Err(anyhow!(e));
                }
            }
        }
    }
    Ok(())
}
