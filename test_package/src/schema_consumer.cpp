#include <quasar/nav.pb.h>

int main() {
  quasar::pb::nav::Telemetry telemetry;
  telemetry.set_satellites(12);
  return telemetry.satellites() == 12 ? 0 : 1;
}
