# `extendr` developer commands

```shell
cargo xtask CMD
```

Options for `CMD`:

- [ ] `doc`: Generates documentation as seen on [/extendr.github.io](https://extendr.github.io/extendr/extendr_api/)
- 

## [NA] Addd `libgcc_eh` for Windows

Hang up is that once a linker is set, and the linker on Windows needing
the presence of certain files to work, then the `xtask` doesn't compile,
thus unable to provide those necessary files.

## Credits

Following [`xtask`](https://github.com/matklad/cargo-xtask) template.