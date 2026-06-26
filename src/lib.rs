include!(concat!(env!("OUT_DIR"), "/_includes.rs"));

pub use mms_protocol::mms::pb::*;
pub use quasar::pb::*;

/// Include path containing the QuaSAR Schema `.proto` files.
///
/// Build scripts of downstream schema crates can add this directory to their
/// protobuf include paths and import files such as `quasar/route.proto`.
pub const PROTO_INCLUDE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/schema");

/// QuaSAR Schema protobuf files, relative to [`PROTO_INCLUDE_DIR`].
pub const PROTO_FILES: &[&str] = &[
  "quasar/args.proto",
  "quasar/error.proto",
  "quasar/image.proto",
  "quasar/nav.proto",
  "quasar/power_switch.proto",
  "quasar/route.proto",
  "quasar/sar.proto",
  "quasar/status.proto",
  "quasar/zmq.proto",
];

pub const PROTO_FILE_DESCRIPTOR_SET: &[u8] =
  include_bytes!(concat!(env!("OUT_DIR"), "/quasar_schema_descriptor.bin"));

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn exposes_proto_files_for_downstream_schema_builds() {
    for file in PROTO_FILES {
      let path = std::path::Path::new(PROTO_INCLUDE_DIR).join(file);
      assert!(path.is_file(), "{} is missing", path.display());
    }
    assert!(!PROTO_FILE_DESCRIPTOR_SET.is_empty());
  }
}
