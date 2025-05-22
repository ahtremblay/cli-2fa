use clap::{Args, Parser, Subcommand};
use keyring::Entry;
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


const SERVICE_NAME: &str = "rust.twofa-cli";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    match args.command {
        Commands::Push(push) => {
            let entry = Entry::new(SERVICE_NAME, &push.name)?;
            entry.set_password(&push.secret)?;
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
    }
    Ok(())
}
