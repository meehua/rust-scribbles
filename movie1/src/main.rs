use clap::{Parser, Subcommand, command};
use movie1::handler::handle_login;

#[derive(Parser)]
#[command(version, about = "Movie app", long_about = "Movie infomation app")]
struct Cli {
    #[command(subcommand)]
    commands: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// User log into the app
    Login {
        /// the username of the user
        #[arg(short, long)]
        username: String,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.commands {
        Some(Commands::Login { username }) => {
            handle_login(&username).unwrap();
        }
        _ => {
            println!("No command provided");
        }
    }
}
