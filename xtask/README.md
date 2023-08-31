# `extendr` developer commands

```shell
cargo xtask CMD
```

Options for `CMD`:

- [ ] `check_fmt`
- [ ] `test_with_r`: Runs `R CMD check` and `testthat` tests in `tests/extendrtests`.
- [ ] `doc`: Generates documentation as seen on [/extendr.github.io](https://extendr.github.io/extendr/extendr_api/)
- [ ] `headers`: Copy R's C-headers to working directory


## [NA] Windows: Add `libgcc_eh` and `libgcc_s.a`

Hang up is that once a linker is set, and the linker on Windows needing
the presence of certain files to work, then the `xtask` doesn't compile,
thus unable to provide those necessary files.

## Credits

Following [`xtask`](https://github.com/matklad/cargo-xtask) template.