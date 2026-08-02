include!(concat!(env!("OUT_DIR"), "/_includes.rs"));

pub mod raw;

pub use mms_ipc_core::mms::pb::*;
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

  #[test]
  fn image_metadata_round_trips_radar_parameters() {
    let metadata = ImageMetadata {
      image_type: ImageType::Telescopic as i32,
      near_edge: Some(Distance { meters: 100.0 }),
      frame_offset: Some(Distance { meters: 400.0 }),
      angle: Some(Angle {
        radians: 0.785_398_2,
      }),
      drift_angle: Some(Angle { radians: 0.0 }),
      divergence_angle: Some(Angle {
        radians: 0.349_065_84,
      }),
      velocity: Some(Velocity { mps: 12.5 }),
      altitude: Some(Distance { meters: 210.0 }),
      frequency_interpolation_coefficient: 1.0,
      time_offset: Some(prost_types::Duration {
        seconds: 0,
        nanos: 750_000_000,
      }),
      time_duration: Some(prost_types::Duration {
        seconds: 0,
        nanos: 500_000_000,
      }),
      mode: 1,
      coordinate: Some(LatLon {
        latitude: 55.762_339_817_927_06,
        longitude: 37.629_222_749_418_204,
      }),
      pixel_size: Some(Dim2 { x: 1.0, y: 1.0 }),
      size: Some(Dim2 {
        x: 2_000.0,
        y: 400.0,
      }),
      radar: Some(RadarSignalParameters {
        capture_mode: 774,
        sampling_frequency_hz: 25_000_000,
        period_sample_count: 41_000,
        sync_pause_sample_count: 300,
        initial_frequency_hz: 5_248_000_000,
        bandwidth_hz: 300_000_000,
        modulation_duration_ns: 1_500_000,
        ramp_type: RadarRampType::Saw as i32,
      }),
      crc: Some(0xBEEF),
    };
    let image = Image {
      metadata: Some(metadata.clone()),
      image_data: vec![0x89, b'P', b'N', b'G'].into(),
    };

    let encoded = image.encode_to_vec();
    let decoded = Image::decode(encoded.as_slice()).expect("image protobuf must decode");

    assert_eq!(decoded.metadata, Some(metadata));
    assert_eq!(decoded.image_data.as_ref(), &[0x89, b'P', b'N', b'G']);
  }
}
