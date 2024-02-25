use clap::Parser;

mod cli;
mod commands;
mod config;
mod error;
mod system;

#[tokio::main]
async fn main() {
  let args = cli::Args::parse();

  if let Some(command) = args.command {
    if let Err(e) = commands::dispatch(command).await {
      eprintln!("{}", e);
    }
  }
}
