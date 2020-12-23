# Guide for Maintainers

## Publishing the crate

### Prerequisite

Install [cargo-workspaces](https://github.com/pksunkara/cargo-workspaces)

``` sh
cargo install cargo-workspaces
```

### Bump version without publish

We use `cargo ws version` for this. This command does:

1. Bump all the versions of the crates in this workspace.
2. Commit.
3. Tag with the bumped version.
4. Push to the repo immediately. (Be careful! If you want to review the change manually before pushing, add `--no-git-push` option as well.)

``` sh
cargo ws version --force='*' --no-individual-tags --pre-id alpha prerelease
```

The meanings of the options and arguments are

* `--force='*'`: By default, `cargo ws version` skips the crates unchange since the last version. This option makes them included in the targets. 
* `--no-individual-tags`: By default, `cargo ws version` creates a tag for each crates (e.g. `crateA@v0.0.1`) in addition to the usual version tag (e.g. `v0.0.1`). This option skips the individual tags.
* `--pre-id alpha`: Specify the identifier prepended to the version.
* `prerelease`: Increase the version with prerelease identifier (e.g. `v0.1.10 -> v0.1.10-alpha.0`, `v0.1.10-alpha.0 -> v0.1.10-alpha.1`). We can also specify `patch`, `minor`, or `major` to increment the corresponding part of the version. Alternatively, we can omit this and choose the version interactively.

<details>

<summary>console output</summary>

If you are asked to select a new version interactively, move the cursor with <kbd>↑</kbd><kbd>↓</kbd>, and press <kbd>Enter</kbd> to choose.

``` console
info looking for changes since v0.1.10
info current common version 0.1.11
? Select a new version (currently 0.1.11) ›
❯ Patch (0.1.12)
  Minor (0.2.0)
  Major (1.0.0)
  Prepatch (0.1.12-alpha.0)
  Preminor (0.2.0-alpha.0)
  Premajor (1.0.0-alpha.0)
  Custom Prerelease
  Custom Version
```

Then, you will be asked to confirm the change. Press `y` to proceed.

``` console
Changes:
 - extendr-api: 0.1.12-alpha.0 => 0.1.12-alpha.1
 - extendr-engine: 0.1.12-alpha.0 => 0.1.12-alpha.1
 - extendr-macros: 0.1.12-alpha.0 => 0.1.12-alpha.1

? Are you sure you want to create these versions? (y/N) › no
```

</details>

### Bump version and publish

When we publish, we use `cargo ws publish`. This command does:

1. Run `cargo ws version`.
2. Publish all the crates within the workspace.

``` sh
# publish
cargo ws publish --force='*' --no-individual-tags patch

# change the version for further development
cargo ws version --force='*' --no-individual-tags --pre-id alpha prerelease
```

This command will publish to Crates.io immediately.
If we want to review the change manually before publishing, we can do it step by step.

``` sh
# bump version
cargo ws version --force='*' --no-individual-tags --no-git-push patch

# review the changes
git show HEAD^

# push all the changes
git push --tags

# publish without modifying the current version
cargo ws publish --force='*' --from-git

# change the version for further development
cargo ws version --force='*' --no-individual-tags --pre-id alpha prerelease
```
