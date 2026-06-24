use std::fmt::{Display, Formatter, Result};

impl Display for crate::Dim2 {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "({0}, {1})", self.x, self.y)
  }
}

impl From<mint::Vector2<i32>> for crate::Dim2 {
  fn from(v: mint::Vector2<i32>) -> Self {
    crate::Dim2 { x: v.x, y: v.y }
  }
}

impl From<crate::Dim2> for mint::Vector2<i32> {
  fn from(v: crate::Dim2) -> Self {
    mint::Vector2 { x: v.x, y: v.y }
  }
}

impl mint::IntoMint for crate::Dim2 {
  type MintType = mint::Vector2<i32>;
}

impl crate::Dim2 {
  pub fn new(x: i32, y: i32) -> Self {
    Self { x, y }
  }
}

impl Display for crate::Dim2F {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(
      f,
      "({0:.p$}, {1:.p$})",
      self.x,
      self.y,
      p = f.precision().unwrap_or(1)
    )
  }
}

impl From<mint::Vector2<f32>> for crate::Dim2F {
  fn from(v: mint::Vector2<f32>) -> Self {
    crate::Dim2F { x: v.x, y: v.y }
  }
}

impl From<crate::Dim2F> for mint::Vector2<f32> {
  fn from(v: crate::Dim2F) -> Self {
    mint::Vector2 { x: v.x, y: v.y }
  }
}

impl mint::IntoMint for crate::Dim2F {
  type MintType = mint::Vector2<f32>;
}

impl crate::Dim2F {
  pub fn new(x: f32, y: f32) -> Self {
    Self { x, y }
  }
}