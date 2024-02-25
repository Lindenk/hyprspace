use std::path::Path;

use futures::prelude::*;
use serde::{Deserialize, Serialize};
use tokio::{fs::remove_file, net::UnixListener, select, signal};
use tokio_serde::formats::*;
use tokio_util::codec::{FramedRead, LengthDelimitedCodec};

use crate::error::Result;

#[derive(Serialize, Deserialize, Debug)]
struct SockReq {
  test: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SockRes {
  test: String,
}

pub async fn daemonize(_fork: bool) -> Result<()> {
  let socket_path = Path::new("/tmp/hyprspace.sock");
  let listener = UnixListener::bind(&socket_path)?;

  loop {
    let socket = select! {
    r = listener.accept() => {
        match r {
          Err(e) => { eprintln!("accept error: {:?}", e);
          continue;
        },
        Ok((socket, _)) => {
          socket
        },
        }
      },
      _ = signal::ctrl_c() => {
        eprintln!("Received SIGINT, shutting down");
        drop(listener);
        remove_file(&socket_path).await?;
        return Ok(());
      }
    };

    let length_delimited = FramedRead::new(socket, LengthDelimitedCodec::new());

    let mut deserialized: tokio_serde::Framed<_, SockReq, SockRes, _> =
      tokio_serde::Framed::new(length_delimited, Json::<SockReq, SockRes>::default());

    tokio::spawn(async move {
      match deserialized.try_next().await {
        Ok(Some(msg)) => println!("{:?}", msg),
        Ok(None) => (),
        Err(e) => println!("error: {:?}", e),
      }
    });
  }
}
