use anyhow::Result;
use std::time::{Duration, SystemTime};

pub fn ts_from_millis(millis: u64) -> prost_types::Timestamp {
  prost_types::Timestamp {
    seconds: (millis / 1000) as i64,
    nanos: ((millis % 1000) * 1_000_000) as i32,
  }
}

pub fn ts_from_now() -> prost_types::Timestamp {
  let now = chrono::Utc::now();
  prost_types::Timestamp {
    seconds: now.timestamp(),
    nanos: now.timestamp_subsec_nanos() as i32,
  }
}

/// Converts a `prost_types::Timestamp` to a `SystemTime`.
///
/// This will panic if the `Timestamp` is before the Unix epoch.
///
/// # Errors
///
/// If the `Timestamp`'s seconds cannot be converted to a `u64` (i.e. if it is
/// negative), an error is returned.
pub fn system_time_from_ts(ts: &prost_types::Timestamp) -> Result<SystemTime> {
  let duration = Duration::new(ts.seconds.try_into()?, ts.nanos as u32);
  Ok(SystemTime::UNIX_EPOCH + duration)
}

pub fn duration_since_ts(ts: &prost_types::Timestamp) -> Result<Duration> {
  Ok(SystemTime::now().duration_since(system_time_from_ts(ts)?)?)
}

pub fn uuid_from_pb(s: &crate::Uuid) -> Result<uuid::Uuid> {
  Ok(uuid::Uuid::parse_str(String::from_utf8_lossy(&s.value).as_ref())?)
}
