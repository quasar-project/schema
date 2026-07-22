#include <quasar/ffi/nav.h>

int main(void) {
  quasar_ffi_nav_telemetry_t telemetry = {0};
  telemetry.navigation_source = QUASAR_FFI_NAV_NAVIGATION_SOURCE_MAVLINK;
  telemetry.fix = QUASAR_FFI_NAV_GPS_FIX_RTK_FIXED;

  quasar_ffi_nav_telemetry_frame_v1_t frame = {0};
  frame.header.magic = QUASAR_FFI_NAV_FRAME_MAGIC;
  frame.header.version = QUASAR_FFI_NAV_FRAME_VERSION;
  frame.header.message_type = QUASAR_FFI_NAV_FRAME_MESSAGE_TYPE_TELEMETRY;
  frame.header.payload_length = (uint16_t)sizeof(frame.payload);
  frame.header.sequence = UINT32_MAX;

  return sizeof(frame) == QUASAR_FFI_NAV_TELEMETRY_FRAME_V1_SIZE
          && frame.header.magic == UINT32_C(0x56414E51)
          && sizeof(frame.payload) == sizeof(telemetry) && telemetry.navigation_source == 4
          && telemetry.fix == 6
         ? 0
         : 1;
}
