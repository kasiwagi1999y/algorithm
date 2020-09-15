pub mod add {
    pub fn half(a: bool, b: bool) -> (bool, bool) {
        (a & b, a ^ b)
    }

    pub fn full(a: bool, b: bool, c: bool) -> (bool, bool) {
        let (c0, s) = half(a, b);
        let (c1, s) = half(s, c);
        (c0 | c1, s)
    }

    pub fn i8(a: i8, b: i8) -> i8 {
        let mut a = a;
        let mut b = b;
        let mut c = false;
        let mut sum = 0;
        for d in 0..8 {
            let p = full(0 != a & 1, 0 != b & 1, c);
            a >>= 1;
            b >>= 1;
            c = p.0;
            if p.1 {
                sum |= 1 << d;
            }
        }
        sum
    }
}

pub mod sub {
    pub fn i8(a: i8, b: i8) -> i8 {
        a.wrapping_add(1i8.wrapping_add(!b))
    }
}

pub mod mul {
    pub fn unsigned(a: u8, b: u8) -> u16 {
        let mut a = a as u16;
        let mut b = b;
        let mut p = 0u16;
        for _ in 0..8 {
            if 0 != (b & 1) {
                p += a;
            }
            b >>= 1;
            a <<= 1;
        }
        p
    }

    pub fn booth(a: i8, b: i8) -> i16 {
        let a = (-a as i16) << 8;
        let mut b = b as u8 as i16;
        let mut x = false;
        for _ in 0..8 {
            let y = 0 != b & 1;
            b >>= 1;
            match (y, x) {
                (false, true) | (true, false) => b += (a >> 1) as i16,
                (false, false) | (true, true) => (),
            }
            x = y;
        }
        b
    }
}

pub mod div {
    pub fn unsigned(n: u8, d: u8) -> (u8, u8) {
        if 0 == d {
            return (0, n);
        }
        let mut q = 0u8;
        let mut r = 0u8;
        for i in (0..8).rev() {
            r <<= 1;
            r |= (n >> i) & 1;
            if r >= d {
                r -= d;
                q |= 1 << i;
            }
        }
        (q, r)
    }

    pub fn restoring(n: u8, d: u8) -> (u8, u8) {
        if 0 == d {
            return (0, n);
        }
        let mut q = 0u8;
        let mut r = n as i16;
        let mut d = (d as i16) << 8;
        for i in (0..8).rev() {
            let t = r;
            d >>= 1;
            r -= d;
            if 0 <= r {
                q |= 1 << i;
            } else {
                r = t;
            }
        }
        (q, r as u8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_half() {
        assert_eq!(add::half(false, false), (false, false));
        assert_eq!(add::half(false, true), (false, true));
        assert_eq!(add::half(true, false), (false, true));
        assert_eq!(add::half(true, true), (true, false));
    }

    #[test]
    fn add_full() {
        assert_eq!(add::full(false, false, false), (false, false));
        assert_eq!(add::full(false, false, true), (false, true));
        assert_eq!(add::full(false, true, false), (false, true));
        assert_eq!(add::full(false, true, true), (true, false));
        assert_eq!(add::full(true, false, false), (false, true));
        assert_eq!(add::full(true, false, true), (true, false));
        assert_eq!(add::full(true, true, false), (true, false));
        assert_eq!(add::full(true, true, true), (true, true));
    }

    #[test]
    fn add_i8() {
        assert_eq!(add::i8(2, 2), 4);
        assert_eq!(add::i8(-2, 4), 2);
        assert_eq!(add::i8(-4, 2), -2);
        assert_eq!(add::i8(-128, -1), 127);
        assert_eq!(add::i8(127, 1), -128);
    }

    #[test]
    fn sub_i8() {
        assert_eq!(sub::i8(2, -2), 4);
        assert_eq!(sub::i8(-2, -4), 2);
        assert_eq!(sub::i8(-4, -2), -2);
        assert_eq!(sub::i8(-128, 1), 127);
        assert_eq!(sub::i8(127, -1), -128);
    }

    #[test]
    fn mul_unsigned() {
        assert_eq!(
            mul::unsigned(u8::MAX, u8::MAX),
            u8::MAX as u16 * u8::MAX as u16
        );
    }

    #[test]
    fn mul_booth() {
        assert_eq!(mul::booth(-1, -1), 1);
        assert_eq!(
            mul::booth(i8::MAX, i8::MIN),
            i8::MAX as i16 * i8::MIN as i16
        );
    }

    #[test]
    fn div_unsigned() {
        assert_eq!(div::unsigned(5, 2), (2, 1));
        assert_eq!(div::unsigned(5, 1), (5, 0));
        assert_eq!(div::unsigned(5, 0), (0, 5));
    }

    #[test]
    fn div_restoring() {
        assert_eq!(div::restoring(5, 2), (2, 1));
        assert_eq!(div::restoring(5, 1), (5, 0));
        assert_eq!(div::restoring(5, 0), (0, 5));
    }
}

