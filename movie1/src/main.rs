use clap::{Parser, Subcommand, command};
use movie1::handler::{handle_add, handle_delete, handle_edit, handle_list, handle_login, handle_logout};

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
    /// Log out
    Logout,
    /// list all the movies
    List,
    /// Add a movie
    Add {
        /// The disc no. of the movie
        #[arg(short, long)]
        disc: usize,

        /// The year when the movie was released
        #[arg(short, long)]
        year: String,

        /// The title / file name of the movie
        #[arg(short, long)]
        title: String,

        /// Optional remark of the movie
        #[arg(short, long)]
        remark: Option<String>,
    },
    /// Delete a movie
    Delete {
        /// The disc no. of the movie
        #[arg(short, long)]
        disc: usize,

        /// The index of the movie in the disc
        #[arg(short, long)]
        index: usize,
    },
    /// Modify a movie
    Edit {
        /// The disc no. of the movie
        #[arg(short, long)]
        disc: usize,

        /// The index of the movie in the disc
        #[arg(short, long)]
        index: usize,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.commands {
        Some(Commands::Login { username }) => {
            handle_login(&username).unwrap();
        }
        Some(Commands::Logout) => handle_logout(),
        Some(Commands::List) => handle_list().unwrap(),
        Some(Commands::Add {
            disc,
            year,
            title,
            remark,
        }) => handle_add(disc, &year, &title, &remark).unwrap(),
        Some(Commands::Delete { disc, index }) => handle_delete(disc, index).unwrap(),
        Some(Commands::Edit { disc, index }) => handle_edit(disc, index).unwrap(),
        _ => {
            println!("No command provided");
        }
    }
}
