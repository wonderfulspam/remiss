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

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in in this project by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>
