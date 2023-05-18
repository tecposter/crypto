// https://github.com/chronos-tachyon/gf256/blob/v0.5.0/poly.go

const POLY_COUNT: usize = 30;

// const PRIMITIVES: [u8; 1] = [0x1b];

#[derive(Copy, Clone, Debug)]
pub enum Poly {
  Poly11b = 0, // 11b = 1 0001 1011 = x^8 + x^4 + x^3 + x + 1
  Poly11d,     // 11d = 1 0001 1101 = x^8 + x^4 + x^3 + x^2 + 1
  Poly12b,     // 12b = 1 0010 1011 = x^8 + x^5 + x^3 + x + 1
  Poly12d,     // 12d = 1 0010 1101 = x^8 + x^5 + x^3 + x^2 + 1
  Poly139,     // 139 = 1 0011 1001 = x^8 + x^5 + x^4 + x^3 + 1
  Poly13f,     // 13f = 1 0011 1111 = x^8 + x^5 + x^4 + x^3 + x^2 + x + 1
  Poly14d,     // 14d = 1 0100 1101 = x^8 + x^6 + x^3 + x^2 + 1
  Poly15f,     // 15f = 1 0101 1111 = x^8 + x^6 + x^4 + x^3 + x^2 + x + 1
  Poly163,     // 163 = 1 0110 0011 = x^8 + x^6 + x^5 + x + 1
  Poly165,     // 165 = 1 0110 0101 = x^8 + x^6 + x^5 + x^2 + 1
  Poly169,     // 169 = 1 0110 1001 = x^8 + x^6 + x^5 + x^3 + 1
  Poly171,     // 171 = 1 0111 0001 = x^8 + x^6 + x^5 + x^4 + 1
  Poly177,     // 177 = 1 0111 0111 = x^8 + x^6 + x^5 + x^4 + x^2 + x + 1
  Poly17b,     // 17b = 1 0111 1011 = x^8 + x^6 + x^5 + x^4 + x^3 + x + 1
  Poly187,     // 187 = 1 1000 0111 = x^8 + x^7 + x^2 + x + 1
  Poly18b,     // 18b = 1 1000 1011 = x^8 + x^7 + x^3 + x + 1
  Poly18d,     // 18d = 1 1000 1101 = x^8 + x^7 + x^3 + x^2 + 1
  Poly19f,     // 19f = 1 1001 1111 = x^8 + x^7 + x^4 + x^3 + x^2 + x + 1
  Poly1a3,     // 1a3 = 1 1010 0011 = x^8 + x^7 + x^5 + x + 1
  Poly1a9,     // 1a9 = 1 1010 1001 = x^8 + x^7 + x^5 + x^3 + 1
  Poly1b1,     // 1b1 = 1 1011 0001 = x^8 + x^7 + x^5 + x^4 + 1
  Poly1bd,     // 1bd = 1 1011 1101 = x^8 + x^7 + x^5 + x^4 + x^3 + x^2 + 1
  Poly1c3,     // 1c3 = 1 1100 0011 = x^8 + x^7 + x^6 + x + 1
  Poly1cf,     // 1cf = 1 1100 1111 = x^8 + x^7 + x^6 + x^3 + x^2 + x + 1
  Poly1d7,     // 1d7 = 1 1101 0111 = x^8 + x^7 + x^6 + x^4 + x^2 + x + 1
  Poly1dd,     // 1dd = 1 1101 1101 = x^8 + x^7 + x^6 + x^4 + x^3 + x^2 + 1
  Poly1e7,     // 1e7 = 1 1110 0111 = x^8 + x^7 + x^6 + x^5 + x^2 + x + 1
  Poly1f3,     // 1f3 = 1 1111 0011 = x^8 + x^7 + x^6 + x^5 + x^4 + x + 1
  Poly1f5,     // 1f5 = 1 1111 0101 = x^8 + x^7 + x^6 + x^5 + x^4 + x^2 + 1
  Poly1f9,     // 1f9 = 1 1111 1001 = x^8 + x^7 + x^6 + x^5 + x^4 + x^3 + 1
}

const PRIMITIVES: [u8; POLY_COUNT] = [
  0x1b, 0x1d, 0x2b, 0x2d, 0x39, 0x3f, 0x4d, 0x5f, 0x63, 0x65, 0x69, 0x71, 0x77, 0x7b, 0x87, 0x8b,
  0x8d, 0x9f, 0xa3, 0xa9, 0xb1, 0xbd, 0xc3, 0xcf, 0xd7, 0xdd, 0xe7, 0xf3, 0xf5, 0xf9,
];

const GENERATORS: [u8; POLY_COUNT] = [
  3, 2, 2, 2, 3, 3, 2, 2, 2, 2, 2, 2, 3, 9, 2, 6, 2, 3, 3, 2, 6, 7, 2, 2, 7, 6, 2, 6, 2, 3,
];

impl Poly {
  pub fn generator(&self) -> u8 {
    let index = *self as usize;
    GENERATORS[index]
  }
}

pub fn poly_mul(mut x: u8, mut y: u8, p: Poly) -> u8 {
  let index = p as usize;
  let p = PRIMITIVES[index];

  let mut z: u8 = 0;
  while y != 0 {
    let low_y = (y & 1) != 0;
    let high_x = (x & 0x80) != 0;
    if low_y {
      z ^= x;
    }
    y >>= 1;
    x <<= 1;

    if high_x {
      x ^= p;
    }
  }

  z
}
