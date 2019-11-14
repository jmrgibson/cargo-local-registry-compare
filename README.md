# cargo-local-registry-compare

Compares the crate index between two local registries on the file system.

This is important if you want to ensure that you didn't accidentally remove any crates from your local registry.

It was created to handle the differences between various versions index metadata formats, and also various sorting mechanisms that will cause textual diffs.

Created from [https://github.com/ChrisGreenaway/cargo-local-registry](https://github.com/ChrisGreenaway/cargo-local-registry)

## Intro

Run with `cargo run --  $OLD_INDEX_DIR $NEW_INDEX_DIR`.

A status message will be printed to stderr, and the differences (if any) will be sent to stdout.

## Version History

### 0.1.1

Added sorting for the vector in `features: BTreeMap<String, Vec<String>>,`

### 0.1.0

Initial release
