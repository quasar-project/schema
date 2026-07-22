#include <quasar/ffi/nav.h>

#include <type_traits>

static_assert(std::is_standard_layout_v<quasar_ffi_nav_telemetry_t>);
static_assert(std::is_trivially_copyable_v<quasar_ffi_nav_telemetry_t>);
static_assert(std::is_standard_layout_v<quasar_ffi_nav_frame_header_t>);
static_assert(std::is_trivially_copyable_v<quasar_ffi_nav_frame_header_t>);
static_assert(std::is_standard_layout_v<quasar_ffi_nav_telemetry_frame_v1_t>);
static_assert(std::is_trivially_copyable_v<quasar_ffi_nav_telemetry_frame_v1_t>);

static_assert(sizeof(quasar_ffi_nav_frame_header_t) == 12);
static_assert(offsetof(quasar_ffi_nav_frame_header_t, magic) == 0);
static_assert(offsetof(quasar_ffi_nav_frame_header_t, version) == 4);
static_assert(offsetof(quasar_ffi_nav_frame_header_t, message_type) == 5);
static_assert(offsetof(quasar_ffi_nav_frame_header_t, payload_length) == 6);
static_assert(offsetof(quasar_ffi_nav_frame_header_t, sequence) == 8);

static_assert(sizeof(quasar_ffi_nav_telemetry_frame_v1_t) == 136);
static_assert(offsetof(quasar_ffi_nav_telemetry_frame_v1_t, header) == 0);
static_assert(offsetof(quasar_ffi_nav_telemetry_frame_v1_t, payload) == 12);
static_assert(offsetof(quasar_ffi_nav_telemetry_frame_v1_t, crc32c) == 132);

static_assert(QUASAR_FFI_NAV_FRAME_MAGIC == UINT32_C(0x56414E51));
static_assert(QUASAR_FFI_NAV_FRAME_VERSION == 1);
static_assert(QUASAR_FFI_NAV_FRAME_MESSAGE_TYPE_TELEMETRY == 1);
static_assert(QUASAR_FFI_NAV_FRAME_HEADER_SIZE == 12);
static_assert(QUASAR_FFI_NAV_TELEMETRY_SIZE == 120);
static_assert(QUASAR_FFI_NAV_FRAME_PAYLOAD_SIZE == 120);
static_assert(QUASAR_FFI_NAV_TELEMETRY_FRAME_V1_SIZE == 136);

int main() {
  quasar_ffi_nav_telemetry_t telemetry {};
  telemetry.navigation_source = QUASAR_FFI_NAV_NAVIGATION_SOURCE_MAVLINK;
  telemetry.fix = QUASAR_FFI_NAV_GPS_FIX_3D;

  quasar_ffi_nav_telemetry_frame_v1_t frame {};
  frame.header.magic = QUASAR_FFI_NAV_FRAME_MAGIC;
  frame.header.version = QUASAR_FFI_NAV_FRAME_VERSION;
  frame.header.message_type = QUASAR_FFI_NAV_FRAME_MESSAGE_TYPE_TELEMETRY;
  frame.header.payload_length = QUASAR_FFI_NAV_TELEMETRY_SIZE;

  return telemetry.navigation_source == 4 && telemetry.fix == 3
          && frame.header.magic == UINT32_C(0x56414E51)
         ? 0
         : 1;
}
