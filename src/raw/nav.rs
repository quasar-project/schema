//! Fixed-layout navigation telemetry protocol.
//!
//! The low-level telemetry payload is 120 bytes. Stream transports should use
//! the 136-byte QNAV frame, which adds a fixed header and CRC-32C.

#[cfg(not(target_endian = "little"))]
compile_error!("quasar_schema::raw requires a little-endian target");

use std::fmt;

use bytemuck::{Pod, Zeroable};

/// Size of a raw navigation telemetry payload in bytes.
pub const TELEMETRY_SIZE: usize = 120;

/// Little-endian integer whose bytes spell `QNAV`.
pub const NAV_FRAME_MAGIC: u32 = 0x5641_4e51;
pub const NAV_FRAME_VERSION: u8 = 1;
pub const NAV_FRAME_MESSAGE_TYPE_TELEMETRY: u8 = 1;
pub const NAV_FRAME_HEADER_SIZE: usize = 12;
pub const NAV_FRAME_PAYLOAD_SIZE: usize = TELEMETRY_SIZE;
pub const NAV_TELEMETRY_FRAME_V1_SIZE: usize = 136;

const CRC32C_REFLECTED_POLYNOMIAL: u32 = 0x82f6_3b78;

const TIMESTAMP_MIN_SECONDS: i64 = -62_135_596_800;
const TIMESTAMP_MAX_SECONDS: i64 = 253_402_300_799;
const TIMESTAMP_MAX_NANOS: i32 = 999_999_999;

/// Fixed-width representation of `quasar.pb.nav.NavigationSource`.
///
/// This is a transparent integer instead of a Rust enum so arbitrary wire
/// values can be decoded safely and rejected by [`Telemetry::validate`].
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, Pod, Zeroable)]
pub struct NavigationSource(i32);

impl NavigationSource {
  pub const UNKNOWN: Self = Self(0);
  pub const NMEA_FILE: Self = Self(1);
  pub const NMEA_SERIAL_PORT: Self = Self(2);
  pub const NMEA_TCP: Self = Self(3);
  pub const MAVLINK: Self = Self(4);
  pub const FAKE: Self = Self(255);

  /// Construct a value without validating its discriminant.
  pub const fn from_raw(value: i32) -> Self {
    Self(value)
  }

  /// Return the raw protobuf-compatible discriminant.
  pub const fn raw(self) -> i32 {
    self.0
  }

  pub const fn is_valid(self) -> bool {
    matches!(self.0, 0..=4 | 255)
  }
}

impl fmt::Debug for NavigationSource {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_tuple("NavigationSource").field(&self.0).finish()
  }
}

impl TryFrom<i32> for NavigationSource {
  type Error = Error;

  fn try_from(value: i32) -> Result<Self, Self::Error> {
    let value = Self(value);
    if value.is_valid() {
      Ok(value)
    } else {
      Err(Error::InvalidNavigationSource(value.0))
    }
  }
}

impl From<NavigationSource> for i32 {
  fn from(value: NavigationSource) -> Self {
    value.raw()
  }
}

/// Fixed-width representation of `quasar.pb.nav.GpsFix`.
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, Pod, Zeroable)]
pub struct GpsFix(i32);

impl GpsFix {
  pub const NO_FIX: Self = Self(0);
  pub const TIME_ONLY: Self = Self(1);
  pub const FIX_2D: Self = Self(2);
  pub const FIX_3D: Self = Self(3);
  pub const DGPS: Self = Self(4);
  pub const RTK_FLOAT: Self = Self(5);
  pub const RTK_FIXED: Self = Self(6);
  pub const PPP: Self = Self(7);

  /// Construct a value without validating its discriminant.
  pub const fn from_raw(value: i32) -> Self {
    Self(value)
  }

  /// Return the raw protobuf-compatible discriminant.
  pub const fn raw(self) -> i32 {
    self.0
  }

  pub const fn is_valid(self) -> bool {
    matches!(self.0, 0..=7)
  }
}

impl fmt::Debug for GpsFix {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_tuple("GpsFix").field(&self.0).finish()
  }
}

impl TryFrom<i32> for GpsFix {
  type Error = Error;

  fn try_from(value: i32) -> Result<Self, Self::Error> {
    let value = Self(value);
    if value.is_valid() {
      Ok(value)
    } else {
      Err(Error::InvalidGpsFix(value.0))
    }
  }
}

impl From<GpsFix> for i32 {
  fn from(value: GpsFix) -> Self {
    value.raw()
  }
}

/// Raw, fixed-layout translation of `quasar.pb.nav.Telemetry`.
///
/// All fields are mandatory. Units are inherited from the protobuf schema.
#[repr(C, packed)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Telemetry {
  pub coordinate_latitude: f64,
  pub coordinate_longitude: f64,
  pub altitude_meters: f32,
  pub altitude_barometric_meters: f32,
  pub velocity_longitudinal_mps: f32,
  pub velocity_lateral_mps: f32,
  pub velocity_vertical_mps: f32,
  pub gyroscope_x: f32,
  pub gyroscope_y: f32,
  pub gyroscope_z: f32,
  pub accelerometer_x: f32,
  pub accelerometer_y: f32,
  pub accelerometer_z: f32,
  pub compass_x: f32,
  pub compass_y: f32,
  pub compass_z: f32,
  pub gps_course_radians: f32,
  pub euler_axes_roll_radians: f32,
  pub euler_axes_pitch_radians: f32,
  pub euler_axes_yaw_radians: f32,
  pub timestamp_seconds: i64,
  pub timestamp_nanos: i32,
  pub pressure: f32,
  pub temperature: f32,
  pub satellites: i32,
  pub navigation_source: NavigationSource,
  pub fix: GpsFix,
}

const _: () = assert!(size_of::<Telemetry>() == TELEMETRY_SIZE);
const _: () = assert!(size_of::<f32>() == 4);
const _: () = assert!(size_of::<f64>() == 8);

impl Telemetry {
  /// Copy this payload into its exact little-endian wire representation.
  pub fn to_le_bytes(self) -> [u8; TELEMETRY_SIZE] {
    let mut bytes = [0; TELEMETRY_SIZE];
    bytes.copy_from_slice(bytemuck::bytes_of(&self));
    bytes
  }

  /// Decode and structurally validate a little-endian wire payload.
  pub fn from_le_bytes(bytes: &[u8]) -> Result<Self, Error> {
    if bytes.len() != TELEMETRY_SIZE {
      return Err(Error::InvalidLength {
        expected: TELEMETRY_SIZE,
        actual: bytes.len(),
      });
    }

    let value: Self = bytemuck::pod_read_unaligned(bytes);
    value.validate()?;
    Ok(value)
  }

  /// Validate fields whose complete value domain is narrower than their wire type.
  pub fn validate(&self) -> Result<(), Error> {
    let navigation_source = self.navigation_source;
    if !navigation_source.is_valid() {
      return Err(Error::InvalidNavigationSource(navigation_source.raw()));
    }

    let fix = self.fix;
    if !fix.is_valid() {
      return Err(Error::InvalidGpsFix(fix.raw()));
    }

    validate_timestamp(self.timestamp_seconds, self.timestamp_nanos)
  }
}

/// Header shared by framed raw navigation messages.
#[repr(C, packed)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct NavFrameHeader {
  pub magic: u32,
  pub version: u8,
  pub message_type: u8,
  pub payload_length: u16,
  pub sequence: u32,
}

const _: () = assert!(size_of::<NavFrameHeader>() == NAV_FRAME_HEADER_SIZE);

impl NavFrameHeader {
  pub const fn new(sequence: u32) -> Self {
    Self {
      magic: NAV_FRAME_MAGIC,
      version: NAV_FRAME_VERSION,
      message_type: NAV_FRAME_MESSAGE_TYPE_TELEMETRY,
      payload_length: NAV_FRAME_PAYLOAD_SIZE as u16,
      sequence,
    }
  }

  pub fn validate(&self) -> Result<(), Error> {
    let magic = self.magic;
    if magic != NAV_FRAME_MAGIC {
      return Err(Error::InvalidFrameMagic(magic));
    }

    let version = self.version;
    if version != NAV_FRAME_VERSION {
      return Err(Error::UnsupportedFrameVersion(version));
    }

    let message_type = self.message_type;
    if message_type != NAV_FRAME_MESSAGE_TYPE_TELEMETRY {
      return Err(Error::UnsupportedMessageType(message_type));
    }

    let payload_length = self.payload_length;
    if payload_length != NAV_FRAME_PAYLOAD_SIZE as u16 {
      return Err(Error::InvalidPayloadLength(payload_length));
    }

    Ok(())
  }
}

/// Version 1 QNAV frame containing a raw navigation telemetry payload.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct NavTelemetryFrameV1 {
  pub header: NavFrameHeader,
  pub payload: [u8; TELEMETRY_SIZE],
  pub crc32c: u32,
}

const _: () = assert!(size_of::<NavTelemetryFrameV1>() == NAV_TELEMETRY_FRAME_V1_SIZE);

impl NavTelemetryFrameV1 {
  /// Construct a validated telemetry frame and calculate its CRC-32C.
  pub fn new(sequence: u32, telemetry: Telemetry) -> Result<Self, Error> {
    telemetry.validate()?;

    let mut frame = Self {
      header: NavFrameHeader::new(sequence),
      payload: telemetry.to_le_bytes(),
      crc32c: 0,
    };
    frame.crc32c = crc32c(&frame.bytes_before_crc());
    Ok(frame)
  }

  /// Convert protobuf telemetry and wrap it in a validated QNAV frame.
  pub fn try_from_protobuf(
    sequence: u32,
    telemetry: &crate::nav::Telemetry,
  ) -> Result<Self, Error> {
    Self::new(sequence, Telemetry::try_from(telemetry)?)
  }

  /// Return the frame sequence number.
  pub fn sequence(&self) -> u32 {
    let header = self.header;
    header.sequence
  }

  /// Validate the fixed header, CRC-32C, and telemetry payload.
  pub fn validate(&self) -> Result<(), Error> {
    let header = self.header;
    header.validate()?;

    let calculated = crc32c(&self.bytes_before_crc());
    let received = self.crc32c;
    if received != calculated {
      return Err(Error::Crc32cMismatch {
        calculated,
        received,
      });
    }

    let payload = self.payload;
    Telemetry::from_le_bytes(&payload)?;
    Ok(())
  }

  /// Decode and validate the telemetry payload contained in this frame.
  pub fn telemetry(&self) -> Result<Telemetry, Error> {
    self.validate()?;
    let payload = self.payload;
    Telemetry::from_le_bytes(&payload)
  }

  /// Copy this frame into its exact 136-byte little-endian representation.
  pub fn to_le_bytes(self) -> [u8; NAV_TELEMETRY_FRAME_V1_SIZE] {
    let mut bytes = [0; NAV_TELEMETRY_FRAME_V1_SIZE];
    bytes[..NAV_FRAME_HEADER_SIZE].copy_from_slice(&header_to_le_bytes(self.header));
    bytes[NAV_FRAME_HEADER_SIZE..NAV_FRAME_HEADER_SIZE + TELEMETRY_SIZE]
      .copy_from_slice(&self.payload);
    bytes[NAV_FRAME_HEADER_SIZE + TELEMETRY_SIZE..].copy_from_slice(&self.crc32c.to_le_bytes());
    bytes
  }

  /// Decode and validate one complete 136-byte QNAV telemetry frame.
  pub fn from_le_bytes(bytes: &[u8]) -> Result<Self, Error> {
    if bytes.len() != NAV_TELEMETRY_FRAME_V1_SIZE {
      return Err(Error::InvalidLength {
        expected: NAV_TELEMETRY_FRAME_V1_SIZE,
        actual: bytes.len(),
      });
    }

    let frame = Self {
      header: NavFrameHeader {
        magic: u32::from_le_bytes(bytes[0..4].try_into().expect("fixed header field")),
        version: bytes[4],
        message_type: bytes[5],
        payload_length: u16::from_le_bytes(bytes[6..8].try_into().expect("fixed header field")),
        sequence: u32::from_le_bytes(bytes[8..12].try_into().expect("fixed header field")),
      },
      payload: bytes[12..132].try_into().expect("validated frame length"),
      crc32c: u32::from_le_bytes(bytes[132..136].try_into().expect("fixed CRC field")),
    };
    frame.validate()?;
    Ok(frame)
  }

  fn bytes_before_crc(&self) -> [u8; NAV_FRAME_HEADER_SIZE + TELEMETRY_SIZE] {
    let mut bytes = [0; NAV_FRAME_HEADER_SIZE + TELEMETRY_SIZE];
    let header = self.header;
    bytes[..NAV_FRAME_HEADER_SIZE].copy_from_slice(&header_to_le_bytes(header));
    bytes[NAV_FRAME_HEADER_SIZE..].copy_from_slice(&self.payload);
    bytes
  }
}

fn header_to_le_bytes(header: NavFrameHeader) -> [u8; NAV_FRAME_HEADER_SIZE] {
  let mut bytes = [0; NAV_FRAME_HEADER_SIZE];
  bytes[0..4].copy_from_slice(&header.magic.to_le_bytes());
  bytes[4] = header.version;
  bytes[5] = header.message_type;
  bytes[6..8].copy_from_slice(&header.payload_length.to_le_bytes());
  bytes[8..12].copy_from_slice(&header.sequence.to_le_bytes());
  bytes
}

fn crc32c(bytes: &[u8]) -> u32 {
  let mut crc = u32::MAX;
  for &byte in bytes {
    crc ^= u32::from(byte);
    for _ in 0..8 {
      let mask = 0_u32.wrapping_sub(crc & 1);
      crc = (crc >> 1) ^ (CRC32C_REFLECTED_POLYNOMIAL & mask);
    }
  }
  !crc
}

/// Conversion or wire-decoding failure for raw navigation telemetry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
  InvalidLength { expected: usize, actual: usize },
  MissingField(&'static str),
  InvalidNavigationSource(i32),
  InvalidGpsFix(i32),
  InvalidTimestamp { seconds: i64, nanos: i32 },
  InvalidFrameMagic(u32),
  UnsupportedFrameVersion(u8),
  UnsupportedMessageType(u8),
  InvalidPayloadLength(u16),
  Crc32cMismatch { calculated: u32, received: u32 },
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::InvalidLength { expected, actual } => {
        write!(
          f,
          "invalid raw navigation length: expected {expected}, got {actual}"
        )
      }
      Self::MissingField(field) => write!(f, "missing telemetry field: {field}"),
      Self::InvalidNavigationSource(value) => {
        write!(f, "invalid navigation source: {value}")
      }
      Self::InvalidGpsFix(value) => write!(f, "invalid GPS fix: {value}"),
      Self::InvalidTimestamp { seconds, nanos } => {
        write!(f, "invalid timestamp: seconds={seconds}, nanos={nanos}")
      }
      Self::InvalidFrameMagic(value) => write!(f, "invalid QNAV frame magic: {value:#010x}"),
      Self::UnsupportedFrameVersion(value) => {
        write!(f, "unsupported QNAV frame version: {value}")
      }
      Self::UnsupportedMessageType(value) => {
        write!(f, "unsupported QNAV message type: {value}")
      }
      Self::InvalidPayloadLength(value) => {
        write!(f, "invalid QNAV payload length: {value}")
      }
      Self::Crc32cMismatch {
        calculated,
        received,
      } => write!(
        f,
        "QNAV CRC-32C mismatch: calculated {calculated:#010x}, received {received:#010x}"
      ),
    }
  }
}

impl std::error::Error for Error {}

impl TryFrom<&crate::nav::Telemetry> for Telemetry {
  type Error = Error;

  fn try_from(value: &crate::nav::Telemetry) -> Result<Self, Self::Error> {
    let coordinate = required(value.coordinate.as_ref(), "coordinate")?;
    let altitude = required(value.altitude.as_ref(), "altitude")?;
    let altitude_barometric = required(value.altitude_barometric.as_ref(), "altitude_barometric")?;
    let velocity_longitudinal = required(
      value.velocity_longitudinal.as_ref(),
      "velocity_longitudinal",
    )?;
    let velocity_lateral = required(value.velocity_lateral.as_ref(), "velocity_lateral")?;
    let velocity_vertical = required(value.velocity_vertical.as_ref(), "velocity_vertical")?;
    let gyroscope = required(value.gyroscope.as_ref(), "gyroscope")?;
    let accelerometer = required(value.accelerometer.as_ref(), "accelerometer")?;
    let compass = required(value.compass.as_ref(), "compass")?;
    let gps_course = required(value.gps_course.as_ref(), "gps_course")?;
    let euler_axes = required(value.euler_axes.as_ref(), "euler_axes")?;
    let roll = required(euler_axes.roll.as_ref(), "euler_axes.roll")?;
    let pitch = required(euler_axes.pitch.as_ref(), "euler_axes.pitch")?;
    let yaw = required(euler_axes.yaw.as_ref(), "euler_axes.yaw")?;
    let timestamp = required(value.timestamp.as_ref(), "timestamp")?;

    validate_timestamp(timestamp.seconds, timestamp.nanos)?;

    Ok(Self {
      coordinate_latitude: coordinate.latitude,
      coordinate_longitude: coordinate.longitude,
      altitude_meters: altitude.meters,
      altitude_barometric_meters: altitude_barometric.meters,
      velocity_longitudinal_mps: velocity_longitudinal.mps,
      velocity_lateral_mps: velocity_lateral.mps,
      velocity_vertical_mps: velocity_vertical.mps,
      gyroscope_x: gyroscope.x,
      gyroscope_y: gyroscope.y,
      gyroscope_z: gyroscope.z,
      accelerometer_x: accelerometer.x,
      accelerometer_y: accelerometer.y,
      accelerometer_z: accelerometer.z,
      compass_x: compass.x,
      compass_y: compass.y,
      compass_z: compass.z,
      gps_course_radians: gps_course.radians,
      euler_axes_roll_radians: roll.radians,
      euler_axes_pitch_radians: pitch.radians,
      euler_axes_yaw_radians: yaw.radians,
      timestamp_seconds: timestamp.seconds,
      timestamp_nanos: timestamp.nanos,
      pressure: value.pressure,
      temperature: value.temperature,
      satellites: value.satellites,
      navigation_source: NavigationSource::try_from(value.navigation_source)?,
      fix: GpsFix::try_from(value.fix)?,
    })
  }
}

impl TryFrom<crate::nav::Telemetry> for Telemetry {
  type Error = Error;

  fn try_from(value: crate::nav::Telemetry) -> Result<Self, Self::Error> {
    Self::try_from(&value)
  }
}

impl TryFrom<&Telemetry> for crate::nav::Telemetry {
  type Error = Error;

  fn try_from(value: &Telemetry) -> Result<Self, Self::Error> {
    value.validate()?;

    let mut result = Self {
      coordinate: Some(crate::LatLon {
        latitude: value.coordinate_latitude,
        longitude: value.coordinate_longitude,
      }),
      altitude: Some(crate::Distance {
        meters: value.altitude_meters,
      }),
      altitude_barometric: Some(crate::Distance {
        meters: value.altitude_barometric_meters,
      }),
      velocity_longitudinal: Some(crate::Velocity {
        mps: value.velocity_longitudinal_mps,
      }),
      velocity_lateral: Some(crate::Velocity {
        mps: value.velocity_lateral_mps,
      }),
      velocity_vertical: Some(crate::Velocity {
        mps: value.velocity_vertical_mps,
      }),
      gyroscope: Some(crate::Dim3 {
        x: value.gyroscope_x,
        y: value.gyroscope_y,
        z: value.gyroscope_z,
      }),
      accelerometer: Some(crate::Dim3 {
        x: value.accelerometer_x,
        y: value.accelerometer_y,
        z: value.accelerometer_z,
      }),
      compass: Some(crate::Dim3 {
        x: value.compass_x,
        y: value.compass_y,
        z: value.compass_z,
      }),
      gps_course: Some(crate::Angle {
        radians: value.gps_course_radians,
      }),
      euler_axes: Some(crate::EulerAngles {
        roll: Some(crate::Angle {
          radians: value.euler_axes_roll_radians,
        }),
        pitch: Some(crate::Angle {
          radians: value.euler_axes_pitch_radians,
        }),
        yaw: Some(crate::Angle {
          radians: value.euler_axes_yaw_radians,
        }),
      }),
      timestamp: Some(Default::default()),
      pressure: value.pressure,
      temperature: value.temperature,
      satellites: value.satellites,
      navigation_source: value.navigation_source.raw(),
      fix: value.fix.raw(),
    };

    let timestamp = result
      .timestamp
      .as_mut()
      .expect("timestamp was initialized");
    timestamp.seconds = value.timestamp_seconds;
    timestamp.nanos = value.timestamp_nanos;

    Ok(result)
  }
}

impl TryFrom<Telemetry> for crate::nav::Telemetry {
  type Error = Error;

  fn try_from(value: Telemetry) -> Result<Self, Self::Error> {
    Self::try_from(&value)
  }
}

fn required<'a, T>(value: Option<&'a T>, field: &'static str) -> Result<&'a T, Error> {
  value.ok_or(Error::MissingField(field))
}

fn validate_timestamp(seconds: i64, nanos: i32) -> Result<(), Error> {
  if !(TIMESTAMP_MIN_SECONDS..=TIMESTAMP_MAX_SECONDS).contains(&seconds)
    || !(0..=TIMESTAMP_MAX_NANOS).contains(&nanos)
  {
    return Err(Error::InvalidTimestamp { seconds, nanos });
  }
  Ok(())
}

#[cfg(test)]
mod tests {
  use std::mem::offset_of;

  use super::*;

  #[test]
  fn telemetry_layout_matches_the_c_contract() {
    assert_eq!(size_of::<Telemetry>(), TELEMETRY_SIZE);
    assert_eq!(align_of::<Telemetry>(), 1);
    assert_eq!(offset_of!(Telemetry, coordinate_latitude), 0);
    assert_eq!(offset_of!(Telemetry, coordinate_longitude), 8);
    assert_eq!(offset_of!(Telemetry, altitude_meters), 16);
    assert_eq!(offset_of!(Telemetry, altitude_barometric_meters), 20);
    assert_eq!(offset_of!(Telemetry, velocity_longitudinal_mps), 24);
    assert_eq!(offset_of!(Telemetry, velocity_lateral_mps), 28);
    assert_eq!(offset_of!(Telemetry, velocity_vertical_mps), 32);
    assert_eq!(offset_of!(Telemetry, gyroscope_x), 36);
    assert_eq!(offset_of!(Telemetry, gyroscope_y), 40);
    assert_eq!(offset_of!(Telemetry, gyroscope_z), 44);
    assert_eq!(offset_of!(Telemetry, accelerometer_x), 48);
    assert_eq!(offset_of!(Telemetry, accelerometer_y), 52);
    assert_eq!(offset_of!(Telemetry, accelerometer_z), 56);
    assert_eq!(offset_of!(Telemetry, compass_x), 60);
    assert_eq!(offset_of!(Telemetry, compass_y), 64);
    assert_eq!(offset_of!(Telemetry, compass_z), 68);
    assert_eq!(offset_of!(Telemetry, gps_course_radians), 72);
    assert_eq!(offset_of!(Telemetry, euler_axes_roll_radians), 76);
    assert_eq!(offset_of!(Telemetry, euler_axes_pitch_radians), 80);
    assert_eq!(offset_of!(Telemetry, euler_axes_yaw_radians), 84);
    assert_eq!(offset_of!(Telemetry, timestamp_seconds), 88);
    assert_eq!(offset_of!(Telemetry, timestamp_nanos), 96);
    assert_eq!(offset_of!(Telemetry, pressure), 100);
    assert_eq!(offset_of!(Telemetry, temperature), 104);
    assert_eq!(offset_of!(Telemetry, satellites), 108);
    assert_eq!(offset_of!(Telemetry, navigation_source), 112);
    assert_eq!(offset_of!(Telemetry, fix), 116);
  }

  #[test]
  fn frame_layout_matches_the_c_contract() {
    assert_eq!(size_of::<NavFrameHeader>(), NAV_FRAME_HEADER_SIZE);
    assert_eq!(align_of::<NavFrameHeader>(), 1);
    assert_eq!(offset_of!(NavFrameHeader, magic), 0);
    assert_eq!(offset_of!(NavFrameHeader, version), 4);
    assert_eq!(offset_of!(NavFrameHeader, message_type), 5);
    assert_eq!(offset_of!(NavFrameHeader, payload_length), 6);
    assert_eq!(offset_of!(NavFrameHeader, sequence), 8);

    assert_eq!(
      size_of::<NavTelemetryFrameV1>(),
      NAV_TELEMETRY_FRAME_V1_SIZE
    );
    assert_eq!(align_of::<NavTelemetryFrameV1>(), 1);
    assert_eq!(offset_of!(NavTelemetryFrameV1, header), 0);
    assert_eq!(offset_of!(NavTelemetryFrameV1, payload), 12);
    assert_eq!(offset_of!(NavTelemetryFrameV1, crc32c), 132);
  }

  #[test]
  fn crc32c_matches_castagnoli_check_value() {
    assert_eq!(crc32c(b"123456789"), 0xe306_9283);
  }

  #[test]
  fn frame_encoding_has_the_fixed_little_endian_layout() {
    let telemetry = Telemetry::try_from(&complete_protobuf()).unwrap();
    let payload = telemetry.to_le_bytes();
    let frame = NavTelemetryFrameV1::new(0x0403_0201, telemetry).unwrap();
    let bytes = frame.to_le_bytes();

    assert_eq!(&bytes[0..4], b"QNAV");
    assert_eq!(bytes[4], NAV_FRAME_VERSION);
    assert_eq!(bytes[5], NAV_FRAME_MESSAGE_TYPE_TELEMETRY);
    assert_eq!(&bytes[6..8], &120_u16.to_le_bytes());
    assert_eq!(&bytes[8..12], &[1, 2, 3, 4]);
    assert_eq!(&bytes[12..132], &payload);
    assert_eq!(
      u32::from_le_bytes(bytes[132..136].try_into().unwrap()),
      crc32c(&bytes[..132])
    );

    let decoded = NavTelemetryFrameV1::from_le_bytes(&bytes).unwrap();
    assert_eq!(decoded.to_le_bytes(), bytes);
  }

  #[test]
  fn protobuf_frame_round_trip_preserves_all_fields() {
    let protobuf = complete_protobuf();
    let frame = NavTelemetryFrameV1::try_from_protobuf(42, &protobuf).unwrap();
    let decoded = NavTelemetryFrameV1::from_le_bytes(&frame.to_le_bytes()).unwrap();
    let round_trip = crate::nav::Telemetry::try_from(decoded.telemetry().unwrap()).unwrap();

    assert_eq!(decoded.sequence(), 42);
    assert_eq!(round_trip, protobuf);
  }

  #[test]
  fn frame_rejects_bad_length_and_metadata() {
    assert!(matches!(
      NavTelemetryFrameV1::from_le_bytes(&[0; NAV_TELEMETRY_FRAME_V1_SIZE - 1]),
      Err(Error::InvalidLength {
        expected: NAV_TELEMETRY_FRAME_V1_SIZE,
        actual,
      }) if actual == NAV_TELEMETRY_FRAME_V1_SIZE - 1
    ));

    let valid = NavTelemetryFrameV1::try_from_protobuf(7, &complete_protobuf())
      .unwrap()
      .to_le_bytes();

    let mut bytes = valid;
    bytes[0] ^= 1;
    assert!(matches!(
      NavTelemetryFrameV1::from_le_bytes(&bytes),
      Err(Error::InvalidFrameMagic(_))
    ));

    let mut bytes = valid;
    bytes[4] = 2;
    assert!(matches!(
      NavTelemetryFrameV1::from_le_bytes(&bytes),
      Err(Error::UnsupportedFrameVersion(2))
    ));

    let mut bytes = valid;
    bytes[5] = 2;
    assert!(matches!(
      NavTelemetryFrameV1::from_le_bytes(&bytes),
      Err(Error::UnsupportedMessageType(2))
    ));

    let mut bytes = valid;
    bytes[6..8].copy_from_slice(&119_u16.to_le_bytes());
    assert!(matches!(
      NavTelemetryFrameV1::from_le_bytes(&bytes),
      Err(Error::InvalidPayloadLength(119))
    ));
  }

  #[test]
  fn frame_rejects_corruption_and_invalid_telemetry() {
    let valid = NavTelemetryFrameV1::try_from_protobuf(7, &complete_protobuf())
      .unwrap()
      .to_le_bytes();

    let mut bytes = valid;
    bytes[12] ^= 1;
    assert!(matches!(
      NavTelemetryFrameV1::from_le_bytes(&bytes),
      Err(Error::Crc32cMismatch { .. })
    ));

    let mut bytes = valid;
    bytes[132] ^= 1;
    assert!(matches!(
      NavTelemetryFrameV1::from_le_bytes(&bytes),
      Err(Error::Crc32cMismatch { .. })
    ));

    let mut bytes = valid;
    bytes[124..128].copy_from_slice(&5_i32.to_le_bytes());
    let checksum = crc32c(&bytes[..132]);
    bytes[132..136].copy_from_slice(&checksum.to_le_bytes());
    assert!(matches!(
      NavTelemetryFrameV1::from_le_bytes(&bytes),
      Err(Error::InvalidNavigationSource(5))
    ));
  }

  #[test]
  fn maximum_and_wrapped_sequences_are_preserved() {
    let telemetry = Telemetry::try_from(&complete_protobuf()).unwrap();
    let maximum = NavTelemetryFrameV1::new(u32::MAX, telemetry).unwrap();
    assert_eq!(maximum.sequence(), u32::MAX);

    let wrapped = maximum.sequence().wrapping_add(1);
    let zero = NavTelemetryFrameV1::new(wrapped, telemetry).unwrap();
    assert_eq!(zero.sequence(), 0);
  }

  #[test]
  fn byte_encoding_is_little_endian_at_fixed_offsets() {
    let value = Telemetry {
      coordinate_latitude: 1.0,
      coordinate_longitude: -2.0,
      altitude_meters: f32::from_bits(0x0403_0201),
      timestamp_seconds: 0x0000_0000_6765_4321,
      timestamp_nanos: 123_456_789,
      navigation_source: NavigationSource::FAKE,
      fix: GpsFix::PPP,
      ..Telemetry::zeroed()
    };

    let bytes = value.to_le_bytes();
    assert_eq!(&bytes[0..8], &1.0_f64.to_le_bytes());
    assert_eq!(&bytes[8..16], &(-2.0_f64).to_le_bytes());
    assert_eq!(&bytes[16..20], &[1, 2, 3, 4]);
    assert_eq!(&bytes[88..96], &[0x21, 0x43, 0x65, 0x67, 0, 0, 0, 0]);
    assert_eq!(&bytes[96..100], &123_456_789_i32.to_le_bytes());
    assert_eq!(&bytes[112..116], &[255, 0, 0, 0]);
    assert_eq!(&bytes[116..120], &[7, 0, 0, 0]);

    let decoded = Telemetry::from_le_bytes(&bytes).unwrap();
    assert_eq!(decoded.to_le_bytes(), bytes);
  }

  #[test]
  fn protobuf_round_trip_preserves_all_fields() {
    let protobuf = complete_protobuf();
    let raw = Telemetry::try_from(&protobuf).unwrap();
    let decoded = Telemetry::from_le_bytes(&raw.to_le_bytes()).unwrap();
    let round_trip = crate::nav::Telemetry::try_from(decoded).unwrap();

    assert_eq!(round_trip, protobuf);
  }

  #[test]
  fn conversion_preserves_non_finite_float_bits() {
    let mut protobuf = complete_protobuf();
    protobuf.pressure = f32::from_bits(0x7fc0_1234);
    protobuf.temperature = f32::NEG_INFINITY;

    let raw = Telemetry::try_from(&protobuf).unwrap();
    let round_trip = crate::nav::Telemetry::try_from(raw).unwrap();

    assert_eq!(round_trip.pressure.to_bits(), 0x7fc0_1234);
    assert_eq!(round_trip.temperature, f32::NEG_INFINITY);
  }

  #[test]
  fn conversion_rejects_missing_nested_fields() {
    let mut protobuf = complete_protobuf();
    protobuf.coordinate = None;
    assert!(matches!(
      Telemetry::try_from(&protobuf),
      Err(Error::MissingField("coordinate"))
    ));

    let mut protobuf = complete_protobuf();
    protobuf.altitude = None;
    assert!(matches!(
      Telemetry::try_from(&protobuf),
      Err(Error::MissingField("altitude"))
    ));

    let mut protobuf = complete_protobuf();
    protobuf.altitude_barometric = None;
    assert!(matches!(
      Telemetry::try_from(&protobuf),
      Err(Error::MissingField("altitude_barometric"))
    ));

    let mut protobuf = complete_protobuf();
    protobuf.velocity_longitudinal = None;
    assert!(matches!(
      Telemetry::try_from(&protobuf),
      Err(Error::MissingField("velocity_longitudinal"))
    ));

    let mut protobuf = complete_protobuf();
    protobuf.velocity_lateral = None;
    assert!(matches!(
      Telemetry::try_from(&protobuf),
      Err(Error::MissingField("velocity_lateral"))
    ));

    let mut protobuf = complete_protobuf();
    protobuf.velocity_vertical = None;
    assert!(matches!(
      Telemetry::try_from(&protobuf),
      Err(Error::MissingField("velocity_vertical"))
    ));

    let mut protobuf = complete_protobuf();
    protobuf.gyroscope = None;
    assert!(matches!(
      Telemetry::try_from(&protobuf),
      Err(Error::MissingField("gyroscope"))
    ));

    let mut protobuf = complete_protobuf();
    protobuf.accelerometer = None;
    assert!(matches!(
      Telemetry::try_from(&protobuf),
      Err(Error::MissingField("accelerometer"))
    ));

    let mut protobuf = complete_protobuf();
    protobuf.compass = None;
    assert!(matches!(
      Telemetry::try_from(&protobuf),
      Err(Error::MissingField("compass"))
    ));

    let mut protobuf = complete_protobuf();
    protobuf.gps_course = None;
    assert!(matches!(
      Telemetry::try_from(&protobuf),
      Err(Error::MissingField("gps_course"))
    ));

    let mut protobuf = complete_protobuf();
    protobuf.euler_axes = None;
    assert!(matches!(
      Telemetry::try_from(&protobuf),
      Err(Error::MissingField("euler_axes"))
    ));

    let mut protobuf = complete_protobuf();
    protobuf.euler_axes.as_mut().unwrap().roll = None;
    assert!(matches!(
      Telemetry::try_from(&protobuf),
      Err(Error::MissingField("euler_axes.roll"))
    ));

    let mut protobuf = complete_protobuf();
    protobuf.euler_axes.as_mut().unwrap().pitch = None;
    assert!(matches!(
      Telemetry::try_from(&protobuf),
      Err(Error::MissingField("euler_axes.pitch"))
    ));

    let mut protobuf = complete_protobuf();
    protobuf.euler_axes.as_mut().unwrap().yaw = None;
    assert!(matches!(
      Telemetry::try_from(&protobuf),
      Err(Error::MissingField("euler_axes.yaw"))
    ));

    let mut protobuf = complete_protobuf();
    protobuf.timestamp = None;
    assert!(matches!(
      Telemetry::try_from(&protobuf),
      Err(Error::MissingField("timestamp"))
    ));
  }

  #[test]
  fn validation_rejects_bad_length_enums_and_timestamps() {
    assert!(matches!(
      Telemetry::from_le_bytes(&[0; TELEMETRY_SIZE - 1]),
      Err(Error::InvalidLength {
        expected: TELEMETRY_SIZE,
        actual,
      })
      if actual == TELEMETRY_SIZE - 1
    ));

    let mut value = Telemetry {
      navigation_source: NavigationSource::from_raw(5),
      ..Telemetry::zeroed()
    };
    assert_eq!(value.validate(), Err(Error::InvalidNavigationSource(5)));

    value.navigation_source = NavigationSource::UNKNOWN;
    value.fix = GpsFix::from_raw(8);
    assert_eq!(value.validate(), Err(Error::InvalidGpsFix(8)));

    value.fix = GpsFix::NO_FIX;
    value.timestamp_nanos = 1_000_000_000;
    assert_eq!(
      value.validate(),
      Err(Error::InvalidTimestamp {
        seconds: 0,
        nanos: 1_000_000_000,
      })
    );

    value.timestamp_nanos = 0;
    value.timestamp_seconds = TIMESTAMP_MAX_SECONDS + 1;
    assert_eq!(
      value.validate(),
      Err(Error::InvalidTimestamp {
        seconds: TIMESTAMP_MAX_SECONDS + 1,
        nanos: 0,
      })
    );
  }

  fn complete_protobuf() -> crate::nav::Telemetry {
    let mut value = crate::nav::Telemetry {
      coordinate: Some(crate::LatLon {
        latitude: 55.75,
        longitude: 37.62,
      }),
      altitude: Some(crate::Distance { meters: 123.5 }),
      altitude_barometric: Some(crate::Distance { meters: 120.25 }),
      velocity_longitudinal: Some(crate::Velocity { mps: 10.0 }),
      velocity_lateral: Some(crate::Velocity { mps: -2.0 }),
      velocity_vertical: Some(crate::Velocity { mps: 0.5 }),
      gyroscope: Some(crate::Dim3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
      }),
      accelerometer: Some(crate::Dim3 {
        x: 4.0,
        y: 5.0,
        z: 6.0,
      }),
      compass: Some(crate::Dim3 {
        x: 7.0,
        y: 8.0,
        z: 9.0,
      }),
      gps_course: Some(crate::Angle { radians: 1.25 }),
      euler_axes: Some(crate::EulerAngles {
        roll: Some(crate::Angle { radians: 0.1 }),
        pitch: Some(crate::Angle { radians: 0.2 }),
        yaw: Some(crate::Angle { radians: 0.3 }),
      }),
      timestamp: Some(Default::default()),
      pressure: 101_325.0,
      temperature: 20.5,
      satellites: 14,
      navigation_source: crate::nav::NavigationSource::Mavlink as i32,
      fix: crate::nav::GpsFix::RtkFixed as i32,
    };
    let timestamp = value.timestamp.as_mut().unwrap();
    timestamp.seconds = 1_750_000_000;
    timestamp.nanos = 123_456_789;
    value
  }
}
