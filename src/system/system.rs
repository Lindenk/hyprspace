use std::collections::HashMap;

use anyhow::anyhow;
use hyprland::{
  data::{Monitors, Workspace, Workspaces},
  dispatch::{Dispatch, DispatchType, MonitorIdentifier, WorkspaceIdentifier},
  shared::HyprData,
};

use super::{
  hyprspace::{HyprSpace, HyprlandWorkspace},
  HyprspaceRequest, HyprspaceResponse,
};
use crate::error::Result;

pub struct HyprspaceSystem {
  pub hyprspaces: HashMap<String, HyprSpace>,
}

impl HyprspaceSystem {
  pub fn new() -> Self {
    Self {
      hyprspaces: HashMap::new(),
    }
  }

  pub async fn handle_request(&mut self, request: HyprspaceRequest) -> HyprspaceResponse {
    match request {
      HyprspaceRequest::CreateHyprspace { name, monitors } => self
        .create_hyprspace(name, monitors)
        .await
        .and(Ok(HyprspaceResponse::Success)),
      HyprspaceRequest::ShowHyprspace { name } => self
        .show_hyprspace(name)
        .await
        .and(Ok(HyprspaceResponse::Success)),
    }
    .into()
  }

  pub async fn create_hyprspace(&mut self, name: String, _monitors: Vec<String>) -> Result<()> {
    let active_monitors = Monitors::get_async().await?.map(|m| HyprlandWorkspace {
      id: m.active_workspace.id,
      monitor_id: m.id as u8, // TODO: report inconsistent monitor id types to upstream
    });
    let hyprspace = HyprSpace::with_workspaces(&name, active_monitors)?;
    println!("hyprspace: {:?}", hyprspace);

    self.hyprspaces.insert(name, hyprspace);

    Ok(())
  }

  pub async fn show_hyprspace(&mut self, space: String) -> Result<()> {
    let space = self
      .hyprspaces
      .get(&space)
      .ok_or(anyhow!("Hyprspace {} not found", space))?;

    println!("swapping to hyprspace: {:?}", space);

    for ws in space.iter_workspaces() {
      println!("moving workspace {} to monitor {}", ws.id, ws.monitor_id);
      Dispatch::call_async(DispatchType::MoveWorkspaceToMonitor(
        WorkspaceIdentifier::Id(ws.id),
        MonitorIdentifier::Id(ws.monitor_id),
      ))
      .await?;
      Dispatch::call_async(DispatchType::Workspace(
        hyprland::dispatch::WorkspaceIdentifierWithSpecial::Id(ws.id),
      ))
      .await?;
    }

    Ok(())
  }
}
