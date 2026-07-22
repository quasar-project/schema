#include <quasar/ffi/nav.h>

_Static_assert(sizeof(quasar_ffi_nav_frame_header_t) == 12, "header size");
_Static_assert(offsetof(quasar_ffi_nav_frame_header_t, magic) == 0, "magic offset");
_Static_assert(offsetof(quasar_ffi_nav_frame_header_t, version) == 4, "version offset");
_Static_assert(offsetof(quasar_ffi_nav_frame_header_t, message_type) == 5, "message type offset");
_Static_assert(
  offsetof(quasar_ffi_nav_frame_header_t, payload_length) == 6,
  "payload length offset"
);
_Static_assert(offsetof(quasar_ffi_nav_frame_header_t, sequence) == 8, "sequence offset");

_Static_assert(sizeof(quasar_ffi_nav_telemetry_frame_v1_t) == 136, "frame size");
_Static_assert(offsetof(quasar_ffi_nav_telemetry_frame_v1_t, header) == 0, "header offset");
_Static_assert(offsetof(quasar_ffi_nav_telemetry_frame_v1_t, payload) == 12, "payload offset");
_Static_assert(offsetof(quasar_ffi_nav_telemetry_frame_v1_t, crc32c) == 132, "CRC offset");

_Static_assert(QUASAR_FFI_NAV_FRAME_MAGIC == UINT32_C(0x56414E51), "magic value");
_Static_assert(QUASAR_FFI_NAV_FRAME_VERSION == 1, "version value");
_Static_assert(QUASAR_FFI_NAV_FRAME_MESSAGE_TYPE_TELEMETRY == 1, "message type value");
_Static_assert(QUASAR_FFI_NAV_FRAME_HEADER_SIZE == 12, "header size constant");
_Static_assert(QUASAR_FFI_NAV_TELEMETRY_SIZE == 120, "payload size constant");
_Static_assert(QUASAR_FFI_NAV_FRAME_PAYLOAD_SIZE == 120, "frame payload size constant");
_Static_assert(QUASAR_FFI_NAV_TELEMETRY_FRAME_V1_SIZE == 136, "frame size constant");

int main(void) {
  quasar_ffi_nav_telemetry_t telemetry = {0};
  telemetry.navigation_source = QUASAR_FFI_NAV_NAVIGATION_SOURCE_FAKE;
  telemetry.fix = QUASAR_FFI_NAV_GPS_FIX_RTK_FIXED;

  quasar_ffi_nav_telemetry_frame_v1_t frame = {0};
  frame.header.magic = QUASAR_FFI_NAV_FRAME_MAGIC;
  frame.header.version = QUASAR_FFI_NAV_FRAME_VERSION;
  frame.header.message_type = QUASAR_FFI_NAV_FRAME_MESSAGE_TYPE_TELEMETRY;
  frame.header.payload_length = (uint16_t)QUASAR_FFI_NAV_TELEMETRY_SIZE;

  return telemetry.navigation_source == 255 && telemetry.fix == 6
          && frame.header.magic == UINT32_C(0x56414E51)
         ? 0
         : 1;
}
