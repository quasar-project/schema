# Raw navigation telemetry protocol

The raw navigation protocol is a fixed-layout alternative to
`quasar.pb.nav.Telemetry` for FPGA and Verilog consumers that cannot implement
protobuf. It exposes only navigation telemetry and its two enum-like fields.

The transport unit is a 136-byte version 1 QNAV frame. It contains a 12-byte
header, the existing 120-byte telemetry payload, and a 4-byte CRC-32C. All
multibyte values are little-endian. Floating-point values use IEEE-754
binary32/binary64 encoding.

## Frame layout

| Offset | Size | Field | Value or meaning |
| ---: | ---: | --- | --- |
| 0 | 4 | `magic` | `0x56414E51`; wire bytes `QNAV` |
| 4 | 1 | `version` | `1` |
| 5 | 1 | `message_type` | `1`, navigation telemetry |
| 6 | 2 | `payload_length` | `120` |
| 8 | 4 | `sequence` | Caller-provided `uint32_t`; wraps naturally |
| 12 | 120 | `payload` | Raw telemetry described below |
| 132 | 4 | `crc32c` | CRC-32C of bytes `[0, 132)` |

CRC parameters are:

- CRC-32C/Castagnoli;
- normal polynomial `0x1EDC6F41`, reflected polynomial `0x82F63B78`;
- initial value `0xFFFFFFFF`;
- reflected input and reflected output;
- final XOR `0xFFFFFFFF`.

The standard check input `123456789` produces `0xE3069283`. The CRC field is
not included in its own calculation.

## Telemetry payload layout

Payload offsets below are relative to byte 12 of the frame. The payload remains
available as an independent low-level representation, but it must not be
written repeatedly to a TTY or other unframed byte stream.

| Offset | C field | Type | Unit or meaning |
| ---: | --- | --- | --- |
| 0 | `coordinate_latitude` | `double` | Degrees, WGS84 |
| 8 | `coordinate_longitude` | `double` | Degrees, WGS84 |
| 16 | `altitude_meters` | `float` | Meters |
| 20 | `altitude_barometric_meters` | `float` | Meters |
| 24 | `velocity_longitudinal_mps` | `float` | Meters per second |
| 28 | `velocity_lateral_mps` | `float` | Meters per second |
| 32 | `velocity_vertical_mps` | `float` | Meters per second |
| 36 | `gyroscope_x` | `float` | Schema-defined sensor value |
| 40 | `gyroscope_y` | `float` | Schema-defined sensor value |
| 44 | `gyroscope_z` | `float` | Schema-defined sensor value |
| 48 | `accelerometer_x` | `float` | Schema-defined sensor value |
| 52 | `accelerometer_y` | `float` | Schema-defined sensor value |
| 56 | `accelerometer_z` | `float` | Schema-defined sensor value |
| 60 | `compass_x` | `float` | Schema-defined sensor value |
| 64 | `compass_y` | `float` | Schema-defined sensor value |
| 68 | `compass_z` | `float` | Schema-defined sensor value |
| 72 | `gps_course_radians` | `float` | Radians |
| 76 | `euler_axes_roll_radians` | `float` | Radians |
| 80 | `euler_axes_pitch_radians` | `float` | Radians |
| 84 | `euler_axes_yaw_radians` | `float` | Radians |
| 88 | `timestamp_seconds` | `int64_t` | UTC seconds since Unix epoch |
| 96 | `timestamp_nanos` | `int32_t` | 0 through 999,999,999 |
| 100 | `pressure` | `float` | Same value as protobuf telemetry |
| 104 | `temperature` | `float` | Same value as protobuf telemetry |
| 108 | `satellites` | `int32_t` | Satellite count |
| 112 | `navigation_source` | `int32_t` | `NavigationSource` discriminant |
| 116 | `fix` | `int32_t` | `GpsFix` discriminant |

Navigation source values are unknown `0`, NMEA file `1`, NMEA serial port `2`,
NMEA TCP `3`, MAVLink `4`, and fake `255`. GPS fix values are no fix `0`, time
only `1`, 2D `2`, 3D `3`, DGPS `4`, RTK float `5`, RTK fixed `6`, and PPP `7`.

## C and C++

```cmake
find_package(QuaSARSchema REQUIRED)
target_link_libraries(fpga_bridge PRIVATE quasar::schema_ffi)
```

```c
#include <quasar/ffi/nav.h>
#include <string.h>

quasar_ffi_nav_telemetry_t telemetry = {0};
quasar_ffi_nav_telemetry_frame_v1_t frame = {0};

frame.header.magic = QUASAR_FFI_NAV_FRAME_MAGIC;
frame.header.version = QUASAR_FFI_NAV_FRAME_VERSION;
frame.header.message_type = QUASAR_FFI_NAV_FRAME_MESSAGE_TYPE_TELEMETRY;
frame.header.payload_length = QUASAR_FFI_NAV_FRAME_PAYLOAD_SIZE;
frame.header.sequence = sequence;
memcpy(frame.payload, &telemetry, sizeof(telemetry));
frame.crc32c = application_crc32c((const uint8_t *)&frame, 132);
write_all(serial_fd, (const uint8_t *)&frame, sizeof(frame));
```

The header-only target does not link the protobuf library. The header rejects
known big-endian and non-IEEE-754 targets at compile time. The C header defines
the layout and constants but deliberately does not provide CRC or I/O routines.

## Rust

```rust
use quasar_schema::raw::nav::NavTelemetryFrameV1;

let frame = NavTelemetryFrameV1::try_from_protobuf(sequence, &protobuf_telemetry)?;
let bytes = frame.to_le_bytes();
serial.write_all(&bytes)?;

let decoded = NavTelemetryFrameV1::from_le_bytes(&bytes)?;
let telemetry = decoded.telemetry()?;
let protobuf = quasar_schema::nav::Telemetry::try_from(telemetry)?;
```

Conversions reject absent protobuf submessages, unknown enum discriminants,
timestamps outside the protobuf range, and invalid nanoseconds. Frame decoding
also rejects an incorrect total length, header metadata, payload length, CRC, or
structurally invalid telemetry. Floating-point bit patterns are preserved.

`Telemetry::to_le_bytes` and `Telemetry::from_le_bytes` remain available for
low-level payload handling. They do not add or validate a QNAV header or CRC.

## TTY receive procedure

TTY buffering and resynchronization belong to the receiver, not this library:

1. Scan for the four wire bytes `QNAV` (`51 4e 41 56` hexadecimal).
2. Starting at that candidate, buffer exactly 136 bytes.
3. Validate the header fields, CRC, and telemetry payload.
4. If validation succeeds, consume the complete frame. If it fails, resume the
   magic scan one byte after the beginning of the failed candidate.

This library intentionally provides no stream decoder. One short read is not a
frame, and one read may contain several frames.

The QNAV version, frame size, offsets, payload types, and enum values are a
stable ABI. Incompatible changes require a new frame/payload version rather
than changing version 1 in place.

## NavSAR local-track frame (version 2)

Version 2 retains the 136-byte QNAV transport size so it fits the existing
68-word RadarSH navigation slot. The header has `version=2`,
`message_type=2`, and `payload_length=120`; magic, sequence, and CRC-32C are
unchanged. A version-1 decoder must reject this distinct payload. FPGA logic
copies either frame verbatim and does not interpret its fields.

The version-2 payload records NavSAR's local north/east/up trajectory even
when GNSS has no fix. All multi-byte values are little-endian. Payload offsets
are relative to byte 12 of the frame.

| Offset | Type | Field | Meaning |
| ---: | --- | --- | --- |
| 0 | `int64` | `host_time_seconds` | Relay UTC publication time, Unix epoch |
| 8 | `int32` | `host_time_nanos` | 0–999999999 |
| 12 | `uint32` | `flags` | Bits below |
| 16 | `uint32` | `fix_quality` | NavSAR GPNAV/GGA quality; 0 means no GNSS fix |
| 20 | `uint32` | `fix_age_ms` | Age of last valid GNSS solution; 999999 if unavailable |
| 24 | `uint32` | `origin_id` | Relay-local epoch, changes when NavSAR origin is reset |
| 28 | `float32` | `d_n_m` | Local displacement north, meters |
| 32 | `float32` | `d_e_m` | Local displacement east, meters |
| 36 | `float32` | `d_h_m` | Local displacement up, meters |
| 40 | `float32` | `v_n_mps` | Local velocity north, m/s |
| 44 | `float32` | `v_e_mps` | Local velocity east, m/s |
| 48 | `float32` | `v_h_mps` | Local velocity up, m/s |
| 52 | `float32` | `yaw_rad` | NavSAR GPINS yaw, radians |
| 56 | `float32` | `pitch_rad` | NavSAR GPINS pitch, radians |
| 60 | `float32` | `roll_rad` | NavSAR GPINS roll, radians |
| 64 | `float32` | `baro_altitude_m` | GPINS barometric altitude |
| 68 | `float32` | `temperature_c` | GPINS temperature |
| 72 | `float64` | `gnss_latitude_deg` | WGS84 latitude; zero if GNSS unavailable |
| 80 | `float64` | `gnss_longitude_deg` | WGS84 longitude; zero if GNSS unavailable |
| 88 | `float32` | `gnss_altitude_m` | GGA altitude; zero if unavailable |
| 92 | `float32` | `gnss_speed_mps` | RMC ground speed; zero if unavailable |
| 96 | `float32` | `gnss_course_rad` | RMC course; zero if unavailable |
| 100 | `uint32` | `satellites` | GGA satellites in use; zero if unavailable |
| 104 | `float32` | `hdop` | GGA HDOP; zero if unavailable |
| 108 | `uint32` | `source` | NavigationSource discriminant from version 1 |
| 112 | `uint32` | `local_update_counter` | Increments for each accepted GPNAV |
| 116 | `uint32` | `attitude_update_counter` | Increments for each accepted GPINS |

Flag bit 0 means a GPNAV sample no more than one second old is available; bit
1 means its `originReady` field is 1; bit 2 means GPINS has been received; bit
3 means the GNSS coordinate fields are valid. Consumers must use the flags and
`fix_quality` rather than inferring validity from numeric zero. When GNSS is
lost, the local displacement and velocity remain recorded as NavSAR reports
them; they are estimates with potentially growing drift. `origin_id` groups
samples from one local origin and must not be treated as a geodetic anchor.
The host time labels publication, not the exact ESP32 sensor epoch. The RadarSH
period index locates each embedded frame on the radar time axis.
