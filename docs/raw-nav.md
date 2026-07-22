# Raw navigation telemetry protocol

The raw navigation protocol is a fixed-layout alternative to
`quasar.pb.nav.Telemetry` for FPGA and Verilog consumers that cannot implement
protobuf. It exposes only navigation telemetry and its two enum-like fields.

The payload is exactly 120 bytes. It uses little-endian integers and IEEE-754
binary32/binary64 floating-point values, with no padding, framing, magic,
version, presence bitmap, or checksum. Every field is mandatory.

## Layout

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

quasar_ffi_nav_telemetry_t telemetry = {0};
```

The header-only target does not link the protobuf library. The header rejects
known big-endian and non-IEEE-754 targets at compile time.

## Rust

```rust
use quasar_schema::raw::nav::Telemetry;

let raw = Telemetry::try_from(&protobuf_telemetry)?;
let bytes = raw.to_le_bytes();
let decoded = Telemetry::from_le_bytes(&bytes)?;
let protobuf = quasar_schema::nav::Telemetry::try_from(decoded)?;
```

Conversions reject absent protobuf submessages, unknown enum discriminants,
timestamps outside the protobuf range, invalid nanoseconds, and payloads whose
length is not exactly 120 bytes. Floating-point bit patterns are preserved.

This protocol is unversioned. Its size, offsets, types, and enum values are a
stable ABI; incompatible changes require a new raw protocol type.
