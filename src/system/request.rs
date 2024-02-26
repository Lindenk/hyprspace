use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum HyprspaceRequest {
  ShowHyprspace { name: String },
  CreateHyprspace { name: String, monitors: Vec<String> },
  DeleteHyprspace { name: String },
}
