use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
  #[command(subcommand)]
  pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
  /// Start the daemon (in the foreground by default)
  Daemonize {
    #[arg(long, help = "Fork into the background")]
    fork: bool,
  },
  /// Create a new hyprspace (group of hyprland workspaces)
  Create {
    #[arg(help = "The name of the hyprspace")]
    name: String,
    #[arg(short, long, help = "Use only the workspaces active on these monitors")]
    monitors: Vec<String>,
  },
  /// Remove a hyprspace
  Rm {
    #[arg(help = "The name of the hyprspace")]
    name: String,
  },
  /// Display a hyprspace
  Show {
    #[arg(help = "The name of the hyprspace")]
    name: String,
  },
  /// Show hyprspace information
  #[command(
    long_about = "Prints all hyprspaces by default, or information for a specific hyprspace if given."
  )]
  Query {
    #[arg(help = "The name of the hyprspace")]
    name: Option<String>,
  },
}

/*
#[derive(Arg)]
pub enum MonitorsAndOrWorkspaces {}
*/
