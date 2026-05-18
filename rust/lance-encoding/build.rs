// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: Copyright The Lance Authors

use std::io::Result;
use std::path::PathBuf;

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=protos");

    #[cfg(feature = "protoc")]
    // Use vendored protobuf compiler if requested.
    std::env::set_var("PROTOC", protobuf_src::protoc());

    // clawpilot-patches: resolve proto paths via CARGO_MANIFEST_DIR rather than
    // the current working directory. Without this, building from a workspace
    // member (e.g. `lance/python` invoking `cargo build` from one level up)
    // fails with "Could not make proto path relative: ./protos/...".
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR must be set by cargo");
    let proto_dir: PathBuf = [&manifest_dir, "protos"].iter().collect();
    let proto_file = proto_dir.join("encodings.proto");

    let mut prost_build = prost_build::Config::new();
    prost_build.protoc_arg("--experimental_allow_proto3_optional");
    prost_build.enable_type_names();
    prost_build.bytes(["."]); // Enable Bytes type for all messages to avoid Vec clones.
    prost_build.compile_protos(&[proto_file], &[proto_dir])?;

    Ok(())
}
