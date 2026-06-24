use std::fmt::{Display, Formatter, Result};

impl Display for crate::Dim3 {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "({0}, {1}, {2})", self.x, self.y, self.z)
  }
}

impl From<mint::Vector3<i32>> for crate::Dim3 {
  fn from(v: mint::Vector3<i32>) -> Self {
    crate::Dim3 { x: v.x, y: v.y, z: v.z }
  }
}

impl From<crate::Dim3> for mint::Vector3<i32> {
  fn from(v: crate::Dim3) -> Self {
    mint::Vector3 { x: v.x, y: v.y, z: v.z }
  }
}

impl mint::IntoMint for crate::Dim3 {
  type MintType = mint::Vector3<i32>;
}

impl crate::Dim3 {
  pub fn new(x: i32, y: i32, z: i32) -> Self {
    Self { x, y, z }
  }
}

impl Display for crate::Dim3F {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(
      f,
      "({0:.p$}, {1:.p$}, {2:.p$})",
      self.x,
      self.y,
      self.z,
      p = f.precision().unwrap_or(1)
    )
  }
}

impl From<mint::Vector3<f32>> for crate::Dim3F {
  fn from(v: mint::Vector3<f32>) -> Self {
    crate::Dim3F { x: v.x, y: v.y, z: v.z }
  }
}

impl From<crate::Dim3F> for mint::Vector3<f32> {
  fn from(v: crate::Dim3F) -> Self {
    mint::Vector3 { x: v.x, y: v.y, z: v.z }
  }
}

impl mint::IntoMint for crate::Dim3F {
  type MintType = mint::Vector3<f32>;
}

impl crate::Dim3F {
  pub fn new(x: f32, y: f32, z: f32) -> Self {
    Self { x, y, z }
  }
}