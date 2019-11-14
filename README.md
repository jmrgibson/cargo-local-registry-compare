# cargo-local-registry-compare

Compares the crate index between two local registries on the file system.

This is important if you want to ensure that you didn't accidentally remove any crates from your local registry.

It was created to handle the differences between various versions index metadata formats, and also various sorting mechanisms that will cause textual diffs.

Created from [https://github.com/ChrisGreenaway/cargo-local-registry](https://github.com/ChrisGreenaway/cargo-local-registry)

## Intro

Run with `cargo run --  $OLD_INDEX_DIR $NEW_INDEX_DIR`.

A status message will be printed to stderr, and the differences (if any) will be sent to stdout.

## Examples

```sh
old_index_dir=./2019-01-01/index
new_index_dir=./2019-05-25/index

cargo run -- $old_index_dir $new_index_dir > out.txt
Success! "./2019-05-25/index" contains all 567 packages in "./2019-01-01/index", and 52 new packages


cargo run -- $new_index_dir $old_index_dir > out.txt
Failure! there are 52 packages that are in "./2019-01-01" (619 total) but not in "./2019-05-25/index" (567 total)
```

## Version History

### 0.1.1

Added sorting for the vector in `features: BTreeMap<String, Vec<String>>,`

### 0.1.0

Initial release
