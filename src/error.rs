pub type Result<T> = std::result::Result<T, HyprspaceError>;

#[derive(Debug, thiserror::Error)]
pub enum HyprspaceError {
  #[error("hyprland error: {0}")]
  Hyprland(#[from] hyprland::shared::HyprError),
  #[error("io error: {0}")]
  IoError(#[from] std::io::Error),
  #[error("{0}")]
  AnyhowError(#[from] anyhow::Error),
}
