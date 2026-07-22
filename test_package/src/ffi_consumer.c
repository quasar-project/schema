#include <quasar/ffi/nav.h>

int main(void) {
  quasar_ffi_nav_telemetry_t telemetry = {0};
  telemetry.navigation_source = QUASAR_FFI_NAV_NAVIGATION_SOURCE_MAVLINK;
  telemetry.fix = QUASAR_FFI_NAV_GPS_FIX_RTK_FIXED;

  return sizeof(telemetry) == QUASAR_FFI_NAV_TELEMETRY_SIZE && telemetry.navigation_source == 4
          && telemetry.fix == 6
         ? 0
         : 1;
}
