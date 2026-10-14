# miscrust

small rust tools in one workspace.

- `unixtime` prints a Discord timestamp pair for a time today. `cargo run -p unixtime -- 18 30` gives 18:30; a trailing `+1` adds days.
- `Test` composites a logo onto every image in a folder: `cargo run -p Test -- logo.png images/`.

`./lint.sh` runs update, clippy, fmt, check and test across the workspace.
