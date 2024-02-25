use std::fmt;

use serde::{Deserialize, Serialize};

use crate::error::Result;

#[derive(Serialize, Deserialize, Debug)]
pub enum HyprspaceResponse {
  Success,
  Error(String),
}

impl fmt::Display for HyprspaceResponse {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      HyprspaceResponse::Success => write!(f, "OK"),
      HyprspaceResponse::Error(e) => write!(f, "{}", e),
    }
  }
}

impl Into<HyprspaceResponse> for Result<HyprspaceResponse> {
  fn into(self) -> HyprspaceResponse {
    match self {
      Ok(res) => res,
      Err(e) => HyprspaceResponse::Error(e.to_string()),
    }
  }
}
