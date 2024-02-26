use crate::cli::Commands;
use crate::error::Result;
use crate::system::{HyprspaceRequest, HyprspaceResponse};

mod client;
mod server;

const SOCKET_PATH: &str = "/tmp/hyprspace.sock";

pub async fn dispatch(command: Commands) -> Result<()> {
  let resp = match command {
    Commands::Create { name, monitors } => {
      client::client_send(HyprspaceRequest::CreateHyprspace { name, monitors }).await?
    }
    Commands::Show { name } => {
      client::client_send(HyprspaceRequest::ShowHyprspace { name }).await?
    }
    Commands::Rm { name } => {
      client::client_send(HyprspaceRequest::DeleteHyprspace { name }).await?
    }
    Commands::Daemonize { fork } => {
      server::daemonize(fork).await?;
      HyprspaceResponse::Success
    }
    _ => unimplemented!(),
  };

  println!("{}", resp);

  Ok(())
}
