#include <quasar/nav.pb.h>
#include <quasar/image.pb.h>

int main() {
  quasar::pb::nav::Telemetry telemetry;
  telemetry.set_satellites(12);

  quasar::pb::Image image;
  auto* metadata = image.mutable_metadata();
  metadata->set_image_type(quasar::pb::IMAGE_TYPE_TELESCOPIC);
  metadata->mutable_coordinate()->set_latitude(55.75);
  metadata->mutable_coordinate()->set_longitude(37.62);
  metadata->mutable_radar()->set_initial_frequency_hz(5'248'000'000ULL);
  metadata->mutable_radar()->set_ramp_type(quasar::pb::RADAR_RAMP_TYPE_SAW);
  metadata->set_crc(0xBEEF);

  return telemetry.satellites() == 12 && metadata->radar().initial_frequency_hz() == 5'248'000'000ULL
             && metadata->has_crc()
           ? 0
           : 1;
}
