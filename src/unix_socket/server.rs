use std::{path::Path, sync::Arc};

use anyhow::anyhow;
use futures::prelude::*;
use tokio::signal::unix::{signal, SignalKind};
use tokio::{fs::remove_file, net::UnixListener, select, sync::Mutex};
use tokio_serde::formats::*;
use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};

use crate::{
  error::Result,
  system::{HyprspaceRequest, HyprspaceResponse, HyprspaceSystem},
};

pub async fn daemonize(fork: bool) -> Result<()> {
  if fork {
    match fork::fork() {
      Ok(fork::Fork::Parent { .. }) => return Ok(()),
      Ok(fork::Fork::Child) => (),
      Err(e) => Err(anyhow!(e.to_string()))?,
    }
  }

  // set up the backend system
  let system = Arc::new(Mutex::new(HyprspaceSystem::new()));

  // make a unix socket to listen on
  let socket_path = Path::new(super::SOCKET_PATH);
  let listener = UnixListener::bind(&socket_path)?;

  loop {
    // TODO: find a solution to this messy signal bs
    // I can't believe this is the state of signal handling in rust...
    let mut sigint = signal(SignalKind::interrupt())?;
    let mut sigterm = signal(SignalKind::terminate())?;
    let mut sighangup = signal(SignalKind::hangup())?;
    let mut sigquit = signal(SignalKind::quit())?;

    let socket = select! {
    r = listener.accept() => {
        match r {
          Err(e) => { eprintln!("accept error: {:?}", e);
          continue;
        },
        Ok((socket, _)) => {
          Some(socket)
        },
        }
      },
      _ = sigint.recv() => None,
      _ = sigterm.recv() => None,
      _ = sighangup.recv() => None,
      _ = sigquit.recv() => None,
    };
    let socket = if let Some(socket) = socket {
      socket
    } else {
      eprintln!("Received shutdown signal, shutting down");
      drop(listener);
      remove_file(&socket_path).await?;
      return Ok(());
    };

    // split them up because tokio doesn't understand how to keep
    // the same framing on both sides (even though the generics
    // already require types for both sides). If this is possible
    // it's undocumented and unclear from any of the source code.
    let (rsock, wsock) = socket.into_split();

    let mut fsock: tokio_serde::Framed<_, HyprspaceRequest, HyprspaceResponse, _> =
      tokio_serde::Framed::new(
        FramedRead::new(rsock, LengthDelimitedCodec::new()),
        Json::<HyprspaceRequest, HyprspaceResponse>::default(),
      );
    let mut wsock: tokio_serde::Framed<_, HyprspaceRequest, HyprspaceResponse, _> =
      tokio_serde::Framed::new(
        FramedWrite::new(wsock, LengthDelimitedCodec::new()),
        Json::<HyprspaceRequest, HyprspaceResponse>::default(),
      );

    let system = system.clone();
    tokio::spawn(async move {
      match fsock.try_next().await {
        Ok(Some(msg)) => {
          let mut system = system.lock().await;
          println!("{:?}", msg);
          let res = system.handle_request(msg).await;
          if let Err(e) = wsock.send(res).await {
            eprintln!("couldn't respond to client: {:?}", e);
          }
        }
        Ok(None) => (),
        Err(e) => println!("error: {:?}", e),
      }
    });
  }
}
