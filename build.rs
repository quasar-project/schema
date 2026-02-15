fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut b = tonic_prost_build::configure().include_file("_includes.rs");
  if cfg!(feature = "serde") {
    const ATTR: &str =
      "#[derive(serde::Serialize, serde::Deserialize)] #[serde(rename_all = \"camelCase\")]";
    b = b
      .message_attribute(".", ATTR)
      .enum_attribute(".", ATTR);
  }
  b = b
    .build_client(cfg!(feature = "client"))
    .build_server(cfg!(feature = "server"))
    .use_arc_self(true)
    .protoc_arg("--experimental_allow_proto3_optional");

  if cfg!(feature = "wkt") {
    b = b.compile_well_known_types(true)
  }
  b.compile_protos(
    &[
      "share/proto/quasar/dim2.proto",
      "share/proto/quasar/dim3.proto",
      "share/proto/quasar/euler_angles.proto",
      "share/proto/quasar/latlon.proto",
      "share/proto/quasar/uuid.proto",
      "share/proto/quasar/image.proto",
      "share/proto/quasar/relay/datagrams/nav.proto",
      "share/proto/quasar/relay/datagrams/shell.proto",
      "share/proto/quasar/relay/datagrams/status.proto",
      "share/proto/quasar/relay/diagnostics/diagnostics.proto",
      #[cfg(feature = "grpc")]
      "share/proto/quasar/relay/services/license.proto",
      #[cfg(feature = "grpc")]
      "share/proto/quasar/relay/services/nav.proto",
      #[cfg(feature = "grpc")]
      "share/proto/quasar/relay/services/shell.proto",
      #[cfg(feature = "grpc")]
      "share/proto/quasar/relay/services/status.proto",
    ],
    &["share/proto"],
  )?;
  Ok(())
}
