#![forbid(unsafe_code)]
use clap::{Parser, Subcommand};
#[derive(Parser)]
#[command(name = "rdbms-superset", about = "SigmaOS RDBMS Superset")]
struct Cli { #[command(subcommand)] cmd: Commands }
#[derive(Subcommand)]
enum Commands { List, Status }
fn main() {
    let cli = Cli::parse();
    match cli.cmd {
        Commands::List   => println!("rdbms-superset: no entries configured."),
        Commands::Status => println!("rdbms-superset: ready."),
    }
}
