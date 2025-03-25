mod app_commands;
mod notif_commands;
mod func;
mod api;

use clap::{Parser, Subcommand};
use dotenv;

const API_URL: &str = "https://endpoint.hey.cafe/";

//---------------------
// Command Structure
//---------------------
// Primary commands
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    /// Setup a connection between heycli and your account with an API key
    Connect,
    
    /// Sever the connection between heycli and Hey.Cafe
    Disconnect,
    
    /// Handle Hey.Cafe notifications
    Notif(notif_commands::NotifArgs),
}

fn main() {
    dotenv::from_filename("./heycli.env").ok();
    let cli = Cli::parse();
    
    match cli.command {
        Some(Commands::Connect) => app_commands::app_connect(),
        Some(Commands::Disconnect) => app_commands::app_disconnect(),
        Some(Commands::Notif(sub_com)) => notif_commands::notif_handler(sub_com),
        None => app_commands::app_base(),
    }
}
