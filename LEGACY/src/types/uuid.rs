use std::{
  fmt::{Display, Formatter},
  str::FromStr,
};

impl TryFrom<&crate::Uuid> for uuid::Uuid {
  type Error = uuid::Error;

  fn try_from(uuid: &crate::Uuid) -> Result<Self, Self::Error> {
    uuid::Uuid::from_str(String::from_utf8_lossy(&uuid.value).as_ref())
  }
}

impl TryFrom<crate::Uuid> for uuid::Uuid {
  type Error = uuid::Error;

  fn try_from(uuid: crate::Uuid) -> Result<Self, Self::Error> {
    uuid::Uuid::from_str(String::from_utf8_lossy(&uuid.value).as_ref())
  }
}

impl From<&uuid::Uuid> for crate::Uuid {
  fn from(value: &uuid::Uuid) -> Self {
    Self {
      value: value.to_string().as_bytes().to_vec(),
      version: value.get_version_num() as u32,
    }
  }
}

impl From<uuid::Uuid> for crate::Uuid {
  fn from(value: uuid::Uuid) -> Self {
    Self {
      value: value.to_string().as_bytes().to_vec(),
      version: value.get_version_num() as u32,
    }
  }
}

impl Display for crate::Uuid {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{{{}}}", String::from_utf8_lossy(&self.value))
  }
}
