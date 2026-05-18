# Clawpilot Patches

This branch (`clawpilot-patches`) carries downstream patches used by the
[Clawpilot Dreaming](https://github.com/anthonyshaw_microsoft/m) feature.

Currently based on upstream `v0.31.1`.

## Patches

### 1. `lance-encoding/build.rs` uses CARGO_MANIFEST_DIR

Upstream's `rust/lance-encoding/build.rs` passes `./protos/encodings.proto`
as a relative path to prost. When the crate is compiled from a workspace
sibling (e.g. `lance/python` triggering a cargo build that drags in
`rust/lance-encoding`), the working directory is wrong and the build fails
with `Could not make proto path relative`. We rebuild the path from
`CARGO_MANIFEST_DIR`.

### 2. Drop `abi3-py39` pyo3 feature

Upstream pyo3 bindings target the limited Python 3.9 ABI for forward
compatibility. Clawpilot embeds a specific CPython 3.12 (via CSnakes) and
wants the native cp312 ABI - better pyo3 perf and access to APIs not in the
limited surface.

## Maintenance

Rebase onto upstream tags periodically. The branch will be force-pushed
whenever we re-rebase, so consumers should pin a specific commit SHA in
their requirements rather than the branch tip.
