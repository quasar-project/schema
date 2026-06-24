const SERDE_DERIVES: &str = "serde::Serialize, serde::Deserialize";
const SERDE_ATTRIBUTES: &str = "#[serde(rename_all = \"camelCase\")]";
const PROTO_FILES: &[&str] = &[
  "quasar/args.proto",
  "quasar/error.proto",
  "quasar/image.proto",
  "quasar/nav.proto",
  "quasar/route.proto",
  "quasar/sar.proto",
  "quasar/status.proto",
  "quasar/zmq.proto",
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
  #[cfg(feature = "vendored-protobuf")]
  unsafe {
    std::env::set_var("PROTOC", protobuf_src::protoc());
  }

  let out_dir = std::path::PathBuf::from(std::env::var_os("OUT_DIR").ok_or("OUT_DIR is unset")?);
  let mut b = prost_build::Config::new();
  b.include_file("_includes.rs");
  b.file_descriptor_set_path(out_dir.join("quasar_schema_descriptor.bin"));
  b.extern_path(".mms.pb", "::mms_protocol");
  let attributes = if cfg!(feature = "serde") {
    Some(format!("#[derive({SERDE_DERIVES})] {SERDE_ATTRIBUTES}"))
  } else {
    None
  };
  if let Some(attributes) = attributes {
    b.message_attribute(".", &attributes)
      .enum_attribute(".", &attributes);
  }
  if cfg!(feature = "legacy") {
    b.protoc_arg("--experimental_allow_proto3_optional");
  }
  if cfg!(feature = "wkt") {
    b.compile_well_known_types();
  }
  let proto_files = PROTO_FILES
    .iter()
    .map(|file| std::path::Path::new("schema").join(file))
    .collect::<Vec<_>>();
  b.compile_protos(&proto_files, &["schema", mms_protocol::PROTO_INCLUDE_DIR])?;
  Ok(())
}
