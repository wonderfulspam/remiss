# Remiss - count assigned Redmine issues

A simple binary that queries Redmine for issues assigned
to "Me" and prints it. Made for use as a polybar module.

Install with `cargo install --git https://github.com/wonderfulspam/remiss`.
Set `redmine_url` and `redmine_api_key` in `~/.config/remiss/remiss.config`.

Example polybar config:
```
[module/remiss]
type = custom/script
interval = 5
format = <label>
exec = remiss
```
