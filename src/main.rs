use clap::Parser;

mod cli;
mod config;
mod error;
mod system;
mod unix_socket;

#[tokio::main]
async fn main() {
  let args = cli::Args::parse();

  if let Some(command) = args.command {
    if let Err(e) = unix_socket::dispatch(command).await {
      eprintln!("{}", e);
    }
  }
}
