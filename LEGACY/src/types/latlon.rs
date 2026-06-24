use std::fmt::{Display, Formatter, Result};

impl Display for crate::LatLon {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(
      f,
      "({0:.p$}°, {1:.p$}°)",
      self.latitude,
      self.longitude,
      p = f.precision().unwrap_or(1)
    )
  }
}

impl crate::LatLon {
  pub fn new(latitude: f64, longitude: f64) -> Self {
    Self {
      latitude,
      longitude,
    }
  }
}
