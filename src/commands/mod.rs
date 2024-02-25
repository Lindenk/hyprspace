use crate::cli::Commands;
use crate::error::Result;

mod daemonize;

pub async fn dispatch(command: Commands) -> Result<()> {
  match command {
    Commands::Daemonize { fork } => daemonize::daemonize(fork).await?,
    _ => unimplemented!(),
  }

  Ok(())
}
