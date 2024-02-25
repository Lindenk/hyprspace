use std::collections::HashSet;

use anyhow::anyhow;

use crate::error::Result;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct HyprlandWorkspace {
  pub id: i32,
  pub monitor_id: u8,
}

#[derive(Clone, Debug)]
pub struct HyprSpace {
  /// Name of the Hyprspace
  name: String,
  /// Hyprland workspaces by ID
  workspaces: HashSet<HyprlandWorkspace>,

  /// Already used workspaces - must be unique
  used_workspaces: HashSet<i32>,
  /// Already used monitors - must be unique
  used_monitors: HashSet<u8>,
}

impl HyprSpace {
  pub fn new(name: &str) -> Self {
    Self {
      name: name.to_string(),
      workspaces: HashSet::new(),
      used_workspaces: HashSet::new(),
      used_monitors: HashSet::new(),
    }
  }

  pub fn with_workspaces(
    name: &str,
    workspaces: impl Iterator<Item = HyprlandWorkspace>,
  ) -> Result<Self> {
    let mut space = Self::new(name);
    for workspace in workspaces {
      space.add_workspace(workspace)?;
    }
    Ok(space)
  }

  pub fn add_workspace(&mut self, workspace: HyprlandWorkspace) -> Result<()> {
    if self.used_workspaces.contains(&workspace.id) {
      Err(anyhow!(
        "Workspace {} already exists for hyprspace {}",
        workspace.id,
        self.name
      ))?;
    }
    if self.used_monitors.contains(&workspace.monitor_id) {
      Err(anyhow!(
        "Monitor {} already exists for hyprspace {}",
        workspace.monitor_id,
        self.name
      ))?;
    }

    self.workspaces.insert(workspace);
    self.used_workspaces.insert(workspace.id);
    self.used_monitors.insert(workspace.monitor_id);

    Ok(())
  }

  pub fn iter_workspaces(&self) -> impl Iterator<Item = &HyprlandWorkspace> {
    self.workspaces.iter()
  }
}
