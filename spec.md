# Subcommands

## Launch the daemon

`hyprspace daemonize` will launch the daemon

`hyprspace daemonize --fork` will launch the daemon in the background

## Create a hyprspace (a group of workspaces)

`hyprspace create <name/id> (-m <monitors(,workspace)>)` will create a hyprspace using the workspaces currently used on each monitor by default.

Specifying monitors will use the currently active workspaces on those monitors by default, otherwise the selected workspaces will be used.

## Show a hyprspace

`hyprspace show <name/id>` will move all workspaces in the hyprspace to their respective monitors

## Delete a hyprspace

`hyprspace delete <name/id>`

## Query hyprspace state

`hyprspace query (--id)` gets the names(ids) of all hyprspaces

`hyprspace query <name/id>`