#ifndef QUASAR_FFI_NAV_H
#define QUASAR_FFI_NAV_H

#include <float.h>
#include <limits.h>
#include <stddef.h>
#include <stdint.h>

#if defined(__BYTE_ORDER__) && defined(__ORDER_LITTLE_ENDIAN__) \
  && __BYTE_ORDER__ != __ORDER_LITTLE_ENDIAN__
#  error "quasar/ffi/nav.h requires a little-endian target"
#endif

#if defined(__cplusplus)
#  define QUASAR_FFI_STATIC_ASSERT(condition, message) static_assert(condition, message)
#else
#  define QUASAR_FFI_STATIC_ASSERT(condition, message) _Static_assert(condition, message)
#endif

QUASAR_FFI_STATIC_ASSERT(CHAR_BIT == 8, "quasar FFI requires 8-bit bytes");
QUASAR_FFI_STATIC_ASSERT(sizeof(float) == 4, "quasar FFI requires 32-bit float");
QUASAR_FFI_STATIC_ASSERT(sizeof(double) == 8, "quasar FFI requires 64-bit double");
QUASAR_FFI_STATIC_ASSERT(
  FLT_RADIX == 2 && FLT_MANT_DIG == 24 && FLT_MAX_EXP == 128,
  "quasar FFI requires IEEE-754 binary32"
);
QUASAR_FFI_STATIC_ASSERT(
  DBL_MANT_DIG == 53 && DBL_MAX_EXP == 0x400,
  "quasar FFI requires IEEE-754 binary64"
);

/** Fixed-width representation of quasar.pb.nav.NavigationSource. */
typedef int32_t quasar_ffi_nav_navigation_source_t;

enum {
  QUASAR_FFI_NAV_NAVIGATION_SOURCE_UNKNOWN = 0,
  QUASAR_FFI_NAV_NAVIGATION_SOURCE_NMEA_FILE = 1,
  QUASAR_FFI_NAV_NAVIGATION_SOURCE_NMEA_SERIAL_PORT = 2,
  QUASAR_FFI_NAV_NAVIGATION_SOURCE_NMEA_TCP = 3,
  QUASAR_FFI_NAV_NAVIGATION_SOURCE_MAVLINK = 4,
  QUASAR_FFI_NAV_NAVIGATION_SOURCE_FAKE = 255,
};

/** Fixed-width representation of quasar.pb.nav.GpsFix. */
typedef int32_t quasar_ffi_nav_gps_fix_t;

enum {
  QUASAR_FFI_NAV_GPS_FIX_NO_FIX = 0,
  QUASAR_FFI_NAV_GPS_FIX_TIME_ONLY = 1,
  QUASAR_FFI_NAV_GPS_FIX_2D = 2,
  QUASAR_FFI_NAV_GPS_FIX_3D = 3,
  QUASAR_FFI_NAV_GPS_FIX_DGPS = 4,
  QUASAR_FFI_NAV_GPS_FIX_RTK_FLOAT = 5,
  QUASAR_FFI_NAV_GPS_FIX_RTK_FIXED = 6,
  QUASAR_FFI_NAV_GPS_FIX_PPP = 7,
};

#define QUASAR_FFI_NAV_TELEMETRY_SIZE ((size_t)120)
#define QUASAR_FFI_NAV_FRAME_MAGIC UINT32_C(0x56414E51)
#define QUASAR_FFI_NAV_FRAME_VERSION UINT8_C(1)
#define QUASAR_FFI_NAV_FRAME_MESSAGE_TYPE_TELEMETRY UINT8_C(1)
#define QUASAR_FFI_NAV_FRAME_HEADER_SIZE ((size_t)12)
#define QUASAR_FFI_NAV_FRAME_PAYLOAD_SIZE QUASAR_FFI_NAV_TELEMETRY_SIZE
#define QUASAR_FFI_NAV_TELEMETRY_FRAME_V1_SIZE ((size_t)136)

#if defined(_MSC_VER)
#  pragma pack(push, 1)
#  define QUASAR_FFI_PACKED
#elif defined(__GNUC__) || defined(__clang__)
#  define QUASAR_FFI_PACKED __attribute__((packed))
#else
#  error "quasar/ffi/nav.h requires a compiler with packed-structure support"
#endif

/**
 * Fixed-layout, little-endian translation of quasar.pb.nav.Telemetry.
 *
 * Every value is mandatory. This low-level payload has no framing, version,
 * presence mask, or checksum; stream transports should carry it inside
 * quasar_ffi_nav_telemetry_frame_v1_t. Copy unaligned input bytes into a
 * suitably declared instance before accessing fields on platforms that
 * restrict unaligned loads.
 */
typedef struct QUASAR_FFI_PACKED quasar_ffi_nav_telemetry {
  double coordinate_latitude;
  double coordinate_longitude;
  float altitude_meters;
  float altitude_barometric_meters;
  float velocity_longitudinal_mps;
  float velocity_lateral_mps;
  float velocity_vertical_mps;
  float gyroscope_x;
  float gyroscope_y;
  float gyroscope_z;
  float accelerometer_x;
  float accelerometer_y;
  float accelerometer_z;
  float compass_x;
  float compass_y;
  float compass_z;
  float gps_course_radians;
  float euler_axes_roll_radians;
  float euler_axes_pitch_radians;
  float euler_axes_yaw_radians;
  int64_t timestamp_seconds;
  int32_t timestamp_nanos;
  float pressure;
  float temperature;
  int32_t satellites;
  quasar_ffi_nav_navigation_source_t navigation_source;
  quasar_ffi_nav_gps_fix_t fix;
} quasar_ffi_nav_telemetry_t;

/** Version 1 QNAV frame header. All multibyte fields are little-endian. */
typedef struct QUASAR_FFI_PACKED quasar_ffi_nav_frame_header {
  uint32_t magic;
  uint8_t version;
  uint8_t message_type;
  uint16_t payload_length;
  uint32_t sequence;
} quasar_ffi_nav_frame_header_t;

/**
 * Version 1 QNAV navigation telemetry frame.
 *
 * crc32c covers header and payload bytes [0, 132), excluding the CRC field.
 */
typedef struct QUASAR_FFI_PACKED quasar_ffi_nav_telemetry_frame_v1 {
  quasar_ffi_nav_frame_header_t header;
  uint8_t payload[QUASAR_FFI_NAV_FRAME_PAYLOAD_SIZE];
  uint32_t crc32c;
} quasar_ffi_nav_telemetry_frame_v1_t;

#if defined(_MSC_VER)
#  pragma pack(pop)
#endif

#undef QUASAR_FFI_PACKED

QUASAR_FFI_STATIC_ASSERT(
  sizeof(quasar_ffi_nav_telemetry_t) == QUASAR_FFI_NAV_TELEMETRY_SIZE,
  "quasar nav telemetry must be 120 bytes"
);
QUASAR_FFI_STATIC_ASSERT(
  offsetof(quasar_ffi_nav_telemetry_t, coordinate_latitude) == 0,
  "invalid coordinate_latitude offset"
);
QUASAR_FFI_STATIC_ASSERT(
  offsetof(quasar_ffi_nav_telemetry_t, coordinate_longitude) == 8,
  "invalid coordinate_longitude offset"
);
QUASAR_FFI_STATIC_ASSERT(
  offsetof(quasar_ffi_nav_telemetry_t, altitude_meters) == 16,
  "invalid altitude_meters offset"
);
QUASAR_FFI_STATIC_ASSERT(
  offsetof(quasar_ffi_nav_telemetry_t, altitude_barometric_meters) == 20,
  "invalid altitude_barometric_meters offset"
);
QUASAR_FFI_STATIC_ASSERT(
  offsetof(quasar_ffi_nav_telemetry_t, velocity_longitudinal_mps) == 24,
  "invalid velocity_longitudinal_mps offset"
);
QUASAR_FFI_STATIC_ASSERT(
  offsetof(quasar_ffi_nav_telemetry_t, velocity_lateral_mps) == 28,
  "invalid velocity_lateral_mps offset"
);
QUASAR_FFI_STATIC_ASSERT(
  offsetof(quasar_ffi_nav_telemetry_t, velocity_vertical_mps) == 32,
  "invalid velocity_vertical_mps offset"
);
QUASAR_FFI_STATIC_ASSERT(
  offsetof(quasar_ffi_nav_telemetry_t, gyroscope_x) == 36,
  "invalid gyroscope_x offset"
);
QUASAR_FFI_STATIC_ASSERT(
  offsetof(quasar_ffi_nav_telemetry_t, gyroscope_y) == 40,
  "invalid gyroscope_y offset"
);
QUASAR_FFI_STATIC_ASSERT(
  offsetof(quasar_ffi_nav_telemetry_t, gyroscope_z) == 44,
  "invalid gyroscope_z offset"
);
QUASAR_FFI_STATIC_ASSERT(
  offsetof(quasar_ffi_nav_telemetry_t, accelerometer_x) == 48,
  "invalid accelerometer_x offset"
);
QUASAR_FFI_STATIC_ASSERT(
  offsetof(quasar_ffi_nav_telemetry_t, accelerometer_y) == 52,
  "invalid accelerometer_y offset"
);
QUASAR_FFI_STATIC_ASSERT(
  offsetof(quasar_ffi_nav_telemetry_t, accelerometer_z) == 56,
  "invalid accelerometer_z offset"
);
QUASAR_FFI_STATIC_ASSERT(
  offsetof(quasar_ffi_nav_telemetry_t, compass_x) == 60,
  "invalid compass_x offset"
);
QUASAR_FFI_STATIC_ASSERT(
  offsetof(quasar_ffi_nav_telemetry_t, compass_y) == 64,
  "invalid compass_y offset"
);
QUASAR_FFI_STATIC_ASSERT(
  offsetof(quasar_ffi_nav_telemetry_t, compass_z) == 68,
  "invalid compass_z offset"
);
QUASAR_FFI_STATIC_ASSERT(
  offsetof(quasar_ffi_nav_telemetry_t, gps_course_radians) == 72,
  "invalid gps_course_radians offset"
);
QUASAR_FFI_STATIC_ASSERT(
  offsetof(quasar_ffi_nav_telemetry_t, euler_axes_roll_radians) == 76,
  "invalid euler_axes_roll_radians offset"
);
QUASAR_FFI_STATIC_ASSERT(
  offsetof(quasar_ffi_nav_telemetry_t, euler_axes_pitch_radians) == 80,
  "invalid euler_axes_pitch_radians offset"
);
QUASAR_FFI_STATIC_ASSERT(
  offsetof(quasar_ffi_nav_telemetry_t, euler_axes_yaw_radians) == 84,
  "invalid euler_axes_yaw_radians offset"
);
QUASAR_FFI_STATIC_ASSERT(
  offsetof(quasar_ffi_nav_telemetry_t, timestamp_seconds) == 88,
  "invalid timestamp_seconds offset"
);
QUASAR_FFI_STATIC_ASSERT(
  offsetof(quasar_ffi_nav_telemetry_t, timestamp_nanos) == 96,
  "invalid timestamp_nanos offset"
);
QUASAR_FFI_STATIC_ASSERT(
  offsetof(quasar_ffi_nav_telemetry_t, pressure) == 100,
  "invalid pressure offset"
);
QUASAR_FFI_STATIC_ASSERT(
  offsetof(quasar_ffi_nav_telemetry_t, temperature) == 104,
  "invalid temperature offset"
);
QUASAR_FFI_STATIC_ASSERT(
  offsetof(quasar_ffi_nav_telemetry_t, satellites) == 108,
  "invalid satellites offset"
);
QUASAR_FFI_STATIC_ASSERT(
  offsetof(quasar_ffi_nav_telemetry_t, navigation_source) == 112,
  "invalid navigation_source offset"
);
QUASAR_FFI_STATIC_ASSERT(offsetof(quasar_ffi_nav_telemetry_t, fix) == 116, "invalid fix offset");

QUASAR_FFI_STATIC_ASSERT(
  sizeof(quasar_ffi_nav_frame_header_t) == QUASAR_FFI_NAV_FRAME_HEADER_SIZE,
  "quasar nav frame header must be 12 bytes"
);
QUASAR_FFI_STATIC_ASSERT(
  offsetof(quasar_ffi_nav_frame_header_t, magic) == 0,
  "invalid frame magic offset"
);
QUASAR_FFI_STATIC_ASSERT(
  offsetof(quasar_ffi_nav_frame_header_t, version) == 4,
  "invalid frame version offset"
);
QUASAR_FFI_STATIC_ASSERT(
  offsetof(quasar_ffi_nav_frame_header_t, message_type) == 5,
  "invalid frame message_type offset"
);
QUASAR_FFI_STATIC_ASSERT(
  offsetof(quasar_ffi_nav_frame_header_t, payload_length) == 6,
  "invalid frame payload_length offset"
);
QUASAR_FFI_STATIC_ASSERT(
  offsetof(quasar_ffi_nav_frame_header_t, sequence) == 8,
  "invalid frame sequence offset"
);

QUASAR_FFI_STATIC_ASSERT(
  sizeof(quasar_ffi_nav_telemetry_frame_v1_t) == QUASAR_FFI_NAV_TELEMETRY_FRAME_V1_SIZE,
  "quasar nav telemetry frame v1 must be 136 bytes"
);
QUASAR_FFI_STATIC_ASSERT(
  offsetof(quasar_ffi_nav_telemetry_frame_v1_t, header) == 0,
  "invalid telemetry frame header offset"
);
QUASAR_FFI_STATIC_ASSERT(
  offsetof(quasar_ffi_nav_telemetry_frame_v1_t, payload) == 12,
  "invalid telemetry frame payload offset"
);
QUASAR_FFI_STATIC_ASSERT(
  offsetof(quasar_ffi_nav_telemetry_frame_v1_t, crc32c) == 132,
  "invalid telemetry frame crc32c offset"
);

#undef QUASAR_FFI_STATIC_ASSERT

#endif
