#include <quasar/ffi/nav.h>

#include <type_traits>

static_assert(std::is_standard_layout_v<quasar_ffi_nav_telemetry_t>);
static_assert(std::is_trivially_copyable_v<quasar_ffi_nav_telemetry_t>);

int main() {
  quasar_ffi_nav_telemetry_t telemetry {};
  telemetry.navigation_source = QUASAR_FFI_NAV_NAVIGATION_SOURCE_MAVLINK;
  telemetry.fix = QUASAR_FFI_NAV_GPS_FIX_3D;

  return telemetry.navigation_source == 4 && telemetry.fix == 3 ? 0 : 1;
}
