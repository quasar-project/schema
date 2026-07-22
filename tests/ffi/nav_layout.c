#include <quasar/ffi/nav.h>

int main(void) {
  quasar_ffi_nav_telemetry_t telemetry = {0};
  telemetry.navigation_source = QUASAR_FFI_NAV_NAVIGATION_SOURCE_FAKE;
  telemetry.fix = QUASAR_FFI_NAV_GPS_FIX_RTK_FIXED;

  return telemetry.navigation_source == 255 && telemetry.fix == 6 ? 0 : 1;
}
