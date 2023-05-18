use crate::poly::{self, poly_mul, Poly};

#[derive(Debug)]
pub struct GF256 {
  exp: [u8; 510],
  log: [u8; 256],
  p: Poly,
}

impl GF256 {
  pub fn new(p: Poly) -> Self {
    let mut exp = [0u8; 510];
    let mut log = [0u8; 256];

    let mut x = 1u8;
    for i in 0u8..255 {
      exp[i as usize] = x;
      exp[i as usize + 255] = x;
      log[x as usize] = i;
      x = poly_mul(x, p.generator(), p);
    }
    log[0] = 255;

    Self { exp, log, p }
  }

  pub fn u8(&self, x: u8) -> GFU8 {
    GFU8 {
      gf256: self,
      inner: x,
    }
  }

  pub fn elem(&self, x: u8) -> GFU8 {
    self.u8(x)
  }
}

#[derive(Copy, Clone, Debug)]
struct GFU8<'a> {
  gf256: &'a GF256,
  inner: u8,
}

impl<'a> GFU8<'a> {
  pub fn val(&self) -> u8 {
    self.inner
  }
}

impl<'a> std::ops::Add for GFU8<'a> {
  type Output = GFU8<'a>;

  fn add(self, rhs: Self) -> Self::Output {
    let z = self.inner ^ rhs.inner;
    Self {
      gf256: self.gf256,
      inner: z,
    }
  }
}

impl<'a> std::ops::Sub for GFU8<'a> {
  type Output = GFU8<'a>;

  fn sub(self, rhs: Self) -> Self::Output {
    Self {
      gf256: self.gf256,
      inner: self.inner ^ rhs.inner,
    }
  }
}

impl<'a> std::ops::Mul for GFU8<'a> {
  type Output = GFU8<'a>;

  fn mul(self, rhs: Self) -> Self::Output {
    let a = self.inner;
    let b = rhs.inner;
    let c: u8 = if a == 0 || b == 0 {
      0
    } else {
      let log = self.gf256.log;
      let log_a = log[a as usize];
      let log_b = log[b as usize];

      let exp = self.gf256.exp;
      exp[log_a as usize + log_b as usize]
    };
    Self {
      gf256: self.gf256,
      inner: c,
    }
  }
}

impl<'a> std::ops::Div for GFU8<'a> {
  type Output = GFU8<'a>;
  fn div(self, rhs: Self) -> Self::Output {
    let a = self.inner;
    let b = rhs.inner;

    if b == 0 {
      panic!("divide by zero");
    }

    let c: u8 = if a == 0 {
      0
    } else {
      let log = self.gf256.log;
      let log_a = log[a as usize];
      let log_b = log[b as usize];

      let exp = self.gf256.exp;
      exp[255 + (log_a as usize) - (log_b as usize)]
    };
    Self {
      gf256: self.gf256,
      inner: c,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_field_new() {
    let f = GF256::new(Poly::Poly11b);
    println!("{f:?}");

    let a = f.u8(1);
    let b = f.u8(2);
    // let c = a + b;

    // println!("---> {}", c.val());
    println!("1 * 2 = {}", (a * b).val());
    println!("1 / 2 = {}", (a / b).val());
    println!("1 + 2 = {}", (a + b).val());
    println!("1 - 2 = {}", (a - b).val());
  }
}
