use std::path::Path;

use futures::prelude::*;
use tokio::net::UnixSocket;
use tokio_serde::formats::*;
use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};

use crate::{
  error::Result,
  system::{HyprspaceRequest, HyprspaceResponse},
};

pub async fn client_send(req: HyprspaceRequest) -> Result<HyprspaceResponse> {
  // make a unix socket to listen on
  let socket_path = Path::new(super::SOCKET_PATH);
  let socket = UnixSocket::new_stream()?;
  let stream = socket.connect(socket_path).await?;

  // no idea why tokio refuses to frame a combined stream
  // TODO: fix this if it's even possible
  let (rstream, wstream) = stream.into_split();

  let rstream = FramedRead::new(rstream, LengthDelimitedCodec::new());
  let wstream = FramedWrite::new(wstream, LengthDelimitedCodec::new());

  let mut rstream: tokio_serde::Framed<_, HyprspaceResponse, HyprspaceRequest, _> =
    tokio_serde::Framed::new(
      rstream,
      Json::<HyprspaceResponse, HyprspaceRequest>::default(),
    );
  let mut wstream: tokio_serde::Framed<_, HyprspaceResponse, HyprspaceRequest, _> =
    tokio_serde::Framed::new(
      wstream,
      Json::<HyprspaceResponse, HyprspaceRequest>::default(),
    );

  wstream.send(req).await?;
  let res = rstream
    .try_next()
    .await
    .map(|r| r.unwrap_or(HyprspaceResponse::Success))?;
  Ok(res)
}
