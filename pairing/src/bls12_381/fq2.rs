use super::fq::{Fq, FROBENIUS_COEFF_FQ2_C1, NEGATIVE_ONE};
use ff::{Field, PowVartime, SqrtField};
use rand_core::RngCore;
use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use subtle::{Choice, ConditionallySelectable, CtOption};

/// An element of Fq2, represented by c0 + c1 * u.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Fq2 {
    pub c0: Fq,
    pub c1: Fq,
}

impl ::std::fmt::Display for Fq2 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "Fq2({} + {} * u)", self.c0, self.c1)
    }
}

/// `Fq2` elements are ordered lexicographically.
impl Ord for Fq2 {
    #[inline(always)]
    fn cmp(&self, other: &Fq2) -> Ordering {
        match self.c1.cmp(&other.c1) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => self.c0.cmp(&other.c0),
        }
    }
}

impl PartialOrd for Fq2 {
    #[inline(always)]
    fn partial_cmp(&self, other: &Fq2) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Fq2 {
    /// Multiply this element by the cubic and quadratic nonresidue 1 + u.
    pub fn mul_by_nonresidue(&mut self) {
        let t0 = self.c0;
        self.c0.sub_assign(&self.c1);
        self.c1.add_assign(&t0);
    }

    /// Norm of Fq2 as extension field in i over Fq
    pub fn norm(&self) -> Fq {
        let t0 = self.c0.square();
        let mut t1 = self.c1.square();
        t1.add_assign(&t0);

        t1
    }
}

impl ConditionallySelectable for Fq2 {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        Fq2 {
            c0: Fq::conditional_select(&a.c0, &b.c0, choice),
            c1: Fq::conditional_select(&a.c1, &b.c1, choice),
        }
    }
}

impl Neg for Fq2 {
    type Output = Self;

    fn neg(self) -> Self {
        Fq2 {
            c0: self.c0.neg(),
            c1: self.c1.neg(),
        }
    }
}

impl<'r> Add<&'r Fq2> for Fq2 {
    type Output = Self;

    fn add(self, other: &Self) -> Self {
        Fq2 {
            c0: self.c0 + other.c0,
            c1: self.c1 + other.c1,
        }
    }
}

impl Add for Fq2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        self.add(&other)
    }
}

impl<'r> AddAssign<&'r Fq2> for Fq2 {
    fn add_assign(&mut self, other: &'r Self) {
        self.c0.add_assign(&other.c0);
        self.c1.add_assign(&other.c1);
    }
}

impl AddAssign for Fq2 {
    fn add_assign(&mut self, other: Self) {
        self.add_assign(&other);
    }
}

impl<'r> Sub<&'r Fq2> for Fq2 {
    type Output = Self;

    fn sub(self, other: &Self) -> Self {
        Fq2 {
            c0: self.c0 - other.c0,
            c1: self.c1 - other.c1,
        }
    }
}

impl Sub for Fq2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self.sub(&other)
    }
}

impl<'r> SubAssign<&'r Fq2> for Fq2 {
    fn sub_assign(&mut self, other: &'r Self) {
        self.c0.sub_assign(&other.c0);
        self.c1.sub_assign(&other.c1);
    }
}

impl SubAssign for Fq2 {
    fn sub_assign(&mut self, other: Self) {
        self.sub_assign(&other);
    }
}

impl<'r> Mul<&'r Fq2> for Fq2 {
    type Output = Self;

    fn mul(self, other: &Self) -> Self {
        let mut ret = self;
        ret.mul_assign(other);
        ret
    }
}

impl Mul for Fq2 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        self.mul(&other)
    }
}

impl<'r> MulAssign<&'r Fq2> for Fq2 {
    fn mul_assign(&mut self, other: &Self) {
        let mut aa = self.c0;
        aa.mul_assign(&other.c0);
        let mut bb = self.c1;
        bb.mul_assign(&other.c1);
        let mut o = other.c0;
        o.add_assign(&other.c1);
        self.c1.add_assign(&self.c0);
        self.c1.mul_assign(&o);
        self.c1.sub_assign(&aa);
        self.c1.sub_assign(&bb);
        self.c0 = aa;
        self.c0.sub_assign(&bb);
    }
}

impl MulAssign for Fq2 {
    fn mul_assign(&mut self, other: Self) {
        self.mul_assign(&other);
    }
}

impl Field for Fq2 {
    fn random<R: RngCore + ?std::marker::Sized>(rng: &mut R) -> Self {
        Fq2 {
            c0: Fq::random(rng),
            c1: Fq::random(rng),
        }
    }

    fn zero() -> Self {
        Fq2 {
            c0: Fq::zero(),
            c1: Fq::zero(),
        }
    }

    fn one() -> Self {
        Fq2 {
            c0: Fq::one(),
            c1: Fq::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        self.c0.is_zero() && self.c1.is_zero()
    }

    fn square(&self) -> Self {
        let mut ab = self.c0;
        ab.mul_assign(&self.c1);
        let mut c0c1 = self.c0;
        c0c1.add_assign(&self.c1);
        let mut c0 = self.c1.neg();
        c0.add_assign(&self.c0);
        c0.mul_assign(&c0c1);
        c0.sub_assign(&ab);
        let mut c1 = ab;
        c1.add_assign(&ab);
        c0.add_assign(&ab);
        Fq2 { c0, c1 }
    }

    fn double(&self) -> Self {
        Fq2 {
            c0: self.c0.double(),
            c1: self.c1.double(),
        }
    }

    fn invert(&self) -> CtOption<Self> {
        let t1 = self.c1.square();
        let mut t0 = self.c0.square();
        t0.add_assign(&t1);
        t0.invert().map(|t| Fq2 {
            c0: self.c0.mul(&t),
            c1: self.c1.mul(&t).neg(),
        })
    }

    fn frobenius_map(&mut self, power: usize) {
        self.c1.mul_assign(&FROBENIUS_COEFF_FQ2_C1[power % 2]);
    }
}

impl SqrtField for Fq2 {
    /// WARNING: THIS IS NOT ACTUALLY CONSTANT TIME YET!
    /// THIS WILL BE REPLACED BY THE bls12_381 CRATE, WHICH IS CONSTANT TIME!
    fn sqrt(&self) -> CtOption<Self> {
        // Algorithm 9, https://eprint.iacr.org/2012/685.pdf

        if self.is_zero() {
            CtOption::new(Self::zero(), Choice::from(1))
        } else {
            // a1 = self^((q - 3) / 4)
            let mut a1 = self.pow_vartime([
                0xee7fbfffffffeaaau64,
                0x7aaffffac54ffff,
                0xd9cc34a83dac3d89,
                0xd91dd2e13ce144af,
                0x92c6e9ed90d2eb35,
                0x680447a8e5ff9a6,
            ]);
            let mut alpha = a1.square();
            alpha.mul_assign(self);
            let mut a0 = alpha;
            a0.frobenius_map(1);
            a0.mul_assign(&alpha);

            let neg1 = Fq2 {
                c0: NEGATIVE_ONE,
                c1: Fq::zero(),
            };

            if a0 == neg1 {
                CtOption::new(Self::zero(), Choice::from(0))
            } else {
                a1.mul_assign(self);

                if alpha == neg1 {
                    a1.mul_assign(&Fq2 {
                        c0: Fq::zero(),
                        c1: Fq::one(),
                    });
                } else {
                    alpha.add_assign(&Fq2::one());
                    // alpha = alpha^((q - 1) / 2)
                    alpha = alpha.pow_vartime([
                        0xdcff7fffffffd555u64,
                        0xf55ffff58a9ffff,
                        0xb39869507b587b12,
                        0xb23ba5c279c2895f,
                        0x258dd3db21a5d66b,
                        0xd0088f51cbff34d,
                    ]);
                    a1.mul_assign(&alpha);
                }

                CtOption::new(a1, Choice::from(1))
            }
        }
    }
}

#[cfg(test)]
use super::fq::FqRepr;
#[cfg(test)]
use ff::PrimeField;

#[test]
fn test_fq2_ordering() {
    let mut a = Fq2 {
        c0: Fq::zero(),
        c1: Fq::zero(),
    };

    let mut b = a;

    assert!(a.cmp(&b) == Ordering::Equal);
    b.c0.add_assign(&Fq::one());
    assert!(a.cmp(&b) == Ordering::Less);
    a.c0.add_assign(&Fq::one());
    assert!(a.cmp(&b) == Ordering::Equal);
    b.c1.add_assign(&Fq::one());
    assert!(a.cmp(&b) == Ordering::Less);
    a.c0.add_assign(&Fq::one());
    assert!(a.cmp(&b) == Ordering::Less);
    a.c1.add_assign(&Fq::one());
    assert!(a.cmp(&b) == Ordering::Greater);
    b.c0.add_assign(&Fq::one());
    assert!(a.cmp(&b) == Ordering::Equal);
}

#[test]
fn test_fq2_basics() {
    assert_eq!(
        Fq2 {
            c0: Fq::zero(),
            c1: Fq::zero(),
        },
        Fq2::zero()
    );
    assert_eq!(
        Fq2 {
            c0: Fq::one(),
            c1: Fq::zero(),
        },
        Fq2::one()
    );
    assert!(Fq2::zero().is_zero());
    assert!(!Fq2::one().is_zero());
    assert!(!Fq2 {
        c0: Fq::zero(),
        c1: Fq::one(),
    }
    .is_zero());
}

#[test]
fn test_fq2_squaring() {
    let a = Fq2 {
        c0: Fq::one(),
        c1: Fq::one(),
    }; // u + 1
    assert_eq!(
        a.square(),
        Fq2 {
            c0: Fq::zero(),
            c1: Fq::from(2),
        }
    ); // 2u

    let a = Fq2 {
        c0: Fq::zero(),
        c1: Fq::one(),
    }; // u
    assert_eq!(a.square(), {
        Fq2 {
            c0: Fq::one().neg(),
            c1: Fq::zero(),
        }
    }); // -1

    let a = Fq2 {
        c0: Fq::from_repr(FqRepr([
            0x07, 0x08, 0x0c, 0x5f, 0xa1, 0xd8, 0xe0, 0x42, 0x41, 0xb7, 0x6d, 0xcc, 0x1c, 0x3f,
            0xbe, 0x5e, 0xf7, 0xf2, 0x95, 0xa9, 0x4e, 0x58, 0xae, 0x7c, 0x90, 0xe3, 0x4a, 0xab,
            0x6f, 0xb6, 0xa6, 0xbd, 0x4e, 0xef, 0x5c, 0x94, 0x65, 0x36, 0xf6, 0x02, 0x9c, 0x2c,
            0x63, 0x09, 0xbb, 0xf8, 0xb5, 0x98,
        ]))
        .unwrap(),
        c1: Fq::from_repr(FqRepr([
            0x10, 0xd1, 0x61, 0x5e, 0x75, 0x25, 0x0a, 0x21, 0xfc, 0x58, 0xa7, 0xb7, 0xbe, 0x81,
            0x54, 0x07, 0xbf, 0xb9, 0x90, 0x20, 0x60, 0x41, 0x37, 0xa0, 0xda, 0xc5, 0xa4, 0xc9,
            0x11, 0xa4, 0x35, 0x3e, 0x6a, 0xd3, 0x29, 0x11, 0x77, 0xc8, 0xc7, 0xe5, 0x38, 0xf4,
            0x73, 0xb3, 0xc8, 0x70, 0xa4, 0xab,
        ]))
        .unwrap(),
    };
    assert_eq!(
        a.square(),
        Fq2 {
            c0: Fq::from_repr(FqRepr([
                0x07, 0xea, 0xc8, 0x13, 0x69, 0xc4, 0x33, 0x61, 0x4c, 0xf1, 0x7b, 0x58, 0x93, 0xc3,
                0xd3, 0x27, 0xcb, 0x67, 0x41, 0x57, 0x61, 0x8d, 0xa1, 0x76, 0x0d, 0xc4, 0x6a, 0xb8,
                0xfa, 0xd6, 0x7a, 0xe0, 0xb9, 0xf2, 0xa6, 0x6e, 0xae, 0x10, 0x73, 0xba, 0xf2, 0x62,
                0xc2, 0x8c, 0x53, 0x8b, 0xcf, 0x68,
            ]))
            .unwrap(),
            c1: Fq::from_repr(FqRepr([
                0x15, 0x42, 0xa6, 0x1c, 0x8a, 0x8d, 0xb9, 0x94, 0x73, 0x9c, 0x98, 0x30, 0x42, 0x77,
                0x9a, 0x65, 0x38, 0xd0, 0xd7, 0x27, 0x5a, 0x96, 0x89, 0xe1, 0xe7, 0x51, 0x38, 0xbc,
                0xe4, 0xce, 0xc7, 0xaa, 0xa2, 0x3e, 0xb7, 0xe1, 0x2d, 0xd5, 0x4d, 0x98, 0xc1, 0x57,
                0x9c, 0xf5, 0x8e, 0x98, 0x0c, 0xf8,
            ]))
            .unwrap(),
        }
    );
}

#[test]
fn test_fq2_mul() {
    use super::fq::FqRepr;
    use ff::PrimeField;

    let mut a = Fq2 {
        c0: Fq::from_repr(FqRepr([
            0x05, 0x1d, 0x3f, 0x92, 0x53, 0xe2, 0x51, 0x6f, 0x1c, 0x20, 0x2d, 0x8e, 0xd9, 0x7a,
            0xfb, 0x45, 0x9e, 0xe5, 0x3e, 0x7e, 0x84, 0xd7, 0x53, 0x2e, 0x41, 0xe4, 0x61, 0x15,
            0x4a, 0x73, 0x54, 0xa3, 0xa2, 0xe3, 0x3c, 0x33, 0x34, 0x49, 0xa1, 0xd6, 0x85, 0xc9,
            0xf9, 0x89, 0xe1, 0x46, 0x1f, 0x03,
        ]))
        .unwrap(),
        c1: Fq::from_repr(FqRepr([
            0x18, 0x0c, 0x3e, 0xe4, 0x66, 0x56, 0xb0, 0x08, 0x7a, 0x5e, 0x1e, 0xcb, 0x67, 0x6d,
            0x65, 0xf9, 0x09, 0x53, 0x3e, 0x4a, 0x9a, 0x51, 0x58, 0xbe, 0x4c, 0xc4, 0x80, 0x81,
            0xc0, 0x9b, 0x89, 0x03, 0x14, 0x3c, 0x21, 0x5d, 0x81, 0x76, 0xb3, 0x19, 0xa7, 0x34,
            0x8a, 0x8b, 0x51, 0x1a, 0xed, 0xcf,
        ]))
        .unwrap(),
    };
    a.mul_assign(&Fq2 {
        c0: Fq::from_repr(FqRepr([
            0x02, 0xc9, 0x3a, 0x72, 0xeb, 0x8a, 0xf8, 0x3e, 0x06, 0xc9, 0x11, 0x02, 0x92, 0xbf,
            0xa4, 0x09, 0xcd, 0x46, 0x0f, 0x9f, 0x0c, 0x23, 0xe4, 0x30, 0x27, 0xec, 0xe1, 0x75,
            0xbe, 0x07, 0xa5, 0x31, 0xfc, 0x87, 0xe6, 0x2e, 0x17, 0x9c, 0x28, 0x5d, 0xe2, 0x1f,
            0x91, 0x69, 0x80, 0x5f, 0x53, 0x7e,
        ]))
        .unwrap(),
        c1: Fq::from_repr(FqRepr([
            0x19, 0xe1, 0x73, 0x34, 0xd4, 0xe9, 0x35, 0x58, 0x63, 0x4c, 0xd3, 0xc6, 0xc5, 0x65,
            0x09, 0x6d, 0x57, 0xa0, 0x6d, 0x31, 0x35, 0xa7, 0x52, 0xae, 0x88, 0x71, 0xc5, 0x08,
            0x65, 0x8d, 0x1e, 0x5f, 0x1d, 0x2a, 0x72, 0x91, 0x6d, 0xba, 0x4c, 0x8a, 0x4b, 0x1c,
            0x3f, 0x93, 0x6d, 0x89, 0x92, 0xd4,
        ]))
        .unwrap(),
    });
    assert_eq!(
        a,
        Fq2 {
            c0: Fq::from_repr(FqRepr([
                0x17, 0x51, 0xaf, 0xbe, 0x16, 0x6e, 0x53, 0x99, 0x53, 0x10, 0xa2, 0x02, 0xd9, 0x2f,
                0x99, 0x63, 0x55, 0x11, 0xfe, 0x4d, 0x84, 0xee, 0x5f, 0x78, 0xf6, 0x1a, 0x96, 0xda,
                0xcf, 0x5a, 0x39, 0xbc, 0xde, 0x29, 0xc3, 0x1a, 0x19, 0xa6, 0x93, 0x7e, 0x95, 0xb5,
                0x12, 0x7e, 0x63, 0x60, 0xc7, 0xe4,
            ]))
            .unwrap(),
            c1: Fq::from_repr(FqRepr([
                0x01, 0xef, 0x1a, 0x36, 0xc2, 0x01, 0x58, 0x9d, 0x33, 0xa9, 0xac, 0x82, 0xce, 0x4c,
                0x50, 0x83, 0xc9, 0x75, 0x10, 0x65, 0x79, 0xc2, 0x75, 0xee, 0x5b, 0xa6, 0xe5, 0x43,
                0x0e, 0x88, 0x3d, 0x40, 0x06, 0xc6, 0x3c, 0xd4, 0xda, 0x2c, 0x2a, 0xa7, 0x84, 0xaf,
                0x0e, 0x1b, 0xd6, 0x30, 0x11, 0x7a,
            ]))
            .unwrap(),
        }
    );
}

#[test]
fn test_fq2_invert() {
    use super::fq::FqRepr;
    use ff::PrimeField;

    assert!(bool::from(Fq2::zero().invert().is_none()));

    let a = Fq2 {
        c0: Fq::from_repr(FqRepr([
            0x05, 0x1d, 0x3f, 0x92, 0x53, 0xe2, 0x51, 0x6f, 0x1c, 0x20, 0x2d, 0x8e, 0xd9, 0x7a,
            0xfb, 0x45, 0x9e, 0xe5, 0x3e, 0x7e, 0x84, 0xd7, 0x53, 0x2e, 0x41, 0xe4, 0x61, 0x15,
            0x4a, 0x73, 0x54, 0xa3, 0xa2, 0xe3, 0x3c, 0x33, 0x34, 0x49, 0xa1, 0xd6, 0x85, 0xc9,
            0xf9, 0x89, 0xe1, 0x46, 0x1f, 0x03,
        ]))
        .unwrap(),
        c1: Fq::from_repr(FqRepr([
            0x18, 0x0c, 0x3e, 0xe4, 0x66, 0x56, 0xb0, 0x08, 0x7a, 0x5e, 0x1e, 0xcb, 0x67, 0x6d,
            0x65, 0xf9, 0x09, 0x53, 0x3e, 0x4a, 0x9a, 0x51, 0x58, 0xbe, 0x4c, 0xc4, 0x80, 0x81,
            0xc0, 0x9b, 0x89, 0x03, 0x14, 0x3c, 0x21, 0x5d, 0x81, 0x76, 0xb3, 0x19, 0xa7, 0x34,
            0x8a, 0x8b, 0x51, 0x1a, 0xed, 0xcf,
        ]))
        .unwrap(),
    };
    let a = a.invert().unwrap();
    assert_eq!(
        a,
        Fq2 {
            c0: Fq::from_repr(FqRepr([
                0x13, 0x51, 0xef, 0x01, 0x94, 0x1b, 0x70, 0xc4, 0xa6, 0xc3, 0xd8, 0xf9, 0x58, 0x6f,
                0x26, 0x36, 0xdf, 0xba, 0x70, 0x32, 0x93, 0x94, 0x1c, 0x30, 0x64, 0xbe, 0xf6, 0x17,
                0xd2, 0x91, 0x5a, 0x8f, 0xe5, 0xec, 0xda, 0x5f, 0xda, 0xfd, 0xdb, 0xb2, 0x07, 0x03,
                0x00, 0xf9, 0xbc, 0xb9, 0xe5, 0x94,
            ]))
            .unwrap(),
            c1: Fq::from_repr(FqRepr([
                0x10, 0x3b, 0xdf, 0x24, 0x1a, 0xfb, 0x00, 0x19, 0xdf, 0x4e, 0x54, 0xf0, 0xd3, 0xef,
                0x15, 0xa6, 0xcb, 0xf6, 0x51, 0xa0, 0xf3, 0x67, 0xaf, 0xb2, 0x94, 0x71, 0x43, 0xf8,
                0x9f, 0xae, 0xde, 0xe9, 0x15, 0xd7, 0xb6, 0xb9, 0x5d, 0xef, 0xbf, 0xf0, 0x8c, 0x39,
                0xfd, 0x76, 0xa8, 0x31, 0x2c, 0xb4,
            ]))
            .unwrap(),
        }
    );
}

#[test]
fn test_fq2_addition() {
    use super::fq::FqRepr;
    use ff::PrimeField;

    let mut a = Fq2 {
        c0: Fq::from_repr(FqRepr([
            0x00, 0xf8, 0xd2, 0x95, 0xb2, 0xde, 0xd9, 0xdc, 0xcc, 0xc6, 0x49, 0xc4, 0xb9, 0x53,
            0x2b, 0xf3, 0xb9, 0x66, 0xce, 0x3b, 0xc2, 0x10, 0x8b, 0x13, 0x8b, 0x1a, 0x52, 0xe0,
            0xa9, 0x0f, 0x59, 0xed, 0x11, 0xe5, 0x9e, 0xa2, 0x21, 0xa3, 0xb6, 0xd2, 0x2d, 0x00,
            0x78, 0x03, 0x69, 0x23, 0xff, 0xc7,
        ]))
        .unwrap(),
        c1: Fq::from_repr(FqRepr([
            0x01, 0x2d, 0x11, 0x37, 0xb8, 0xa6, 0xa8, 0x37, 0x4e, 0x46, 0x4d, 0xea, 0x5b, 0xcf,
            0xd4, 0x1e, 0xb3, 0xf8, 0xaf, 0xc0, 0xee, 0x24, 0x8c, 0xad, 0xbe, 0x20, 0x34, 0x11,
            0xc6, 0x6f, 0xb3, 0xa5, 0x94, 0x6a, 0xe5, 0x2d, 0x68, 0x4f, 0xa7, 0xed, 0x97, 0x7d,
            0xf6, 0xef, 0xcd, 0xae, 0xe0, 0xdb,
        ]))
        .unwrap(),
    };
    a.add_assign(&Fq2 {
        c0: Fq::from_repr(FqRepr([
            0x13, 0xce, 0x43, 0x3f, 0xa2, 0x60, 0x27, 0xf5, 0x98, 0x6a, 0x4a, 0x62, 0xfa, 0x82,
            0xa4, 0x9d, 0x3b, 0x88, 0x89, 0x9a, 0x42, 0xa6, 0x31, 0x8f, 0x4b, 0xf0, 0xb9, 0x9a,
            0x9f, 0x0d, 0xca, 0x12, 0xb9, 0x3a, 0xdf, 0xc9, 0x11, 0x9e, 0x33, 0xe8, 0x61, 0x9a,
            0x02, 0xd7, 0x8d, 0xc7, 0x0e, 0xf2,
        ]))
        .unwrap(),
        c1: Fq::from_repr(FqRepr([
            0x11, 0xd6, 0xe2, 0x0e, 0x98, 0x6c, 0x20, 0x85, 0x4c, 0x8c, 0x18, 0x00, 0xeb, 0x10,
            0x45, 0x66, 0x22, 0x36, 0xf5, 0x52, 0x46, 0xd0, 0xd4, 0x4d, 0x40, 0x2a, 0xef, 0x1f,
            0xb7, 0x97, 0xe3, 0x2f, 0xa1, 0x37, 0x9b, 0x6f, 0xac, 0xf6, 0xe5, 0x96, 0x66, 0x32,
            0x3b, 0xf8, 0x0b, 0x58, 0xb9, 0xb9,
        ]))
        .unwrap(),
    });
    assert_eq!(
        a,
        Fq2 {
            c0: Fq::from_repr(FqRepr([
                0x14, 0xc7, 0x15, 0xd5, 0x55, 0x3f, 0x01, 0xd2, 0x65, 0x30, 0x94, 0x27, 0xb3, 0xd5,
                0xd0, 0x90, 0xf4, 0xef, 0x57, 0xd6, 0x04, 0xb6, 0xbc, 0xa2, 0xd7, 0x0b, 0x0c, 0x7b,
                0x48, 0x1d, 0x23, 0xff, 0xcb, 0x20, 0x7e, 0x6b, 0x33, 0x41, 0xea, 0xba, 0x8e, 0x9a,
                0x7a, 0xda, 0xf6, 0xeb, 0x0e, 0xb9,
            ]))
            .unwrap(),
            c1: Fq::from_repr(FqRepr([
                0x13, 0x03, 0xf3, 0x46, 0x51, 0x12, 0xc8, 0xbc, 0x9a, 0xd2, 0x65, 0xeb, 0x46, 0xe0,
                0x19, 0x84, 0xd6, 0x2f, 0xa5, 0x13, 0x34, 0xf5, 0x60, 0xfa, 0xfe, 0x4b, 0x23, 0x31,
                0x7e, 0x07, 0x96, 0xd5, 0x35, 0xa2, 0x80, 0x9d, 0x15, 0x46, 0x8d, 0x83, 0xfd, 0xb0,
                0x32, 0xe7, 0xd9, 0x07, 0x9a, 0x94,
            ]))
            .unwrap(),
        }
    );
}

#[test]
fn test_fq2_subtraction() {
    use super::fq::FqRepr;
    use ff::PrimeField;

    let mut a = Fq2 {
        c0: Fq::from_repr(FqRepr([
            0x00, 0xf8, 0xd2, 0x95, 0xb2, 0xde, 0xd9, 0xdc, 0xcc, 0xc6, 0x49, 0xc4, 0xb9, 0x53,
            0x2b, 0xf3, 0xb9, 0x66, 0xce, 0x3b, 0xc2, 0x10, 0x8b, 0x13, 0x8b, 0x1a, 0x52, 0xe0,
            0xa9, 0x0f, 0x59, 0xed, 0x11, 0xe5, 0x9e, 0xa2, 0x21, 0xa3, 0xb6, 0xd2, 0x2d, 0x00,
            0x78, 0x03, 0x69, 0x23, 0xff, 0xc7,
        ]))
        .unwrap(),
        c1: Fq::from_repr(FqRepr([
            0x01, 0x2d, 0x11, 0x37, 0xb8, 0xa6, 0xa8, 0x37, 0x4e, 0x46, 0x4d, 0xea, 0x5b, 0xcf,
            0xd4, 0x1e, 0xb3, 0xf8, 0xaf, 0xc0, 0xee, 0x24, 0x8c, 0xad, 0xbe, 0x20, 0x34, 0x11,
            0xc6, 0x6f, 0xb3, 0xa5, 0x94, 0x6a, 0xe5, 0x2d, 0x68, 0x4f, 0xa7, 0xed, 0x97, 0x7d,
            0xf6, 0xef, 0xcd, 0xae, 0xe0, 0xdb,
        ]))
        .unwrap(),
    };
    a.sub_assign(&Fq2 {
        c0: Fq::from_repr(FqRepr([
            0x13, 0xce, 0x43, 0x3f, 0xa2, 0x60, 0x27, 0xf5, 0x98, 0x6a, 0x4a, 0x62, 0xfa, 0x82,
            0xa4, 0x9d, 0x3b, 0x88, 0x89, 0x9a, 0x42, 0xa6, 0x31, 0x8f, 0x4b, 0xf0, 0xb9, 0x9a,
            0x9f, 0x0d, 0xca, 0x12, 0xb9, 0x3a, 0xdf, 0xc9, 0x11, 0x9e, 0x33, 0xe8, 0x61, 0x9a,
            0x02, 0xd7, 0x8d, 0xc7, 0x0e, 0xf2,
        ]))
        .unwrap(),
        c1: Fq::from_repr(FqRepr([
            0x11, 0xd6, 0xe2, 0x0e, 0x98, 0x6c, 0x20, 0x85, 0x4c, 0x8c, 0x18, 0x00, 0xeb, 0x10,
            0x45, 0x66, 0x22, 0x36, 0xf5, 0x52, 0x46, 0xd0, 0xd4, 0x4d, 0x40, 0x2a, 0xef, 0x1f,
            0xb7, 0x97, 0xe3, 0x2f, 0xa1, 0x37, 0x9b, 0x6f, 0xac, 0xf6, 0xe5, 0x96, 0x66, 0x32,
            0x3b, 0xf8, 0x0b, 0x58, 0xb9, 0xb9,
        ]))
        .unwrap(),
    });
    assert_eq!(
        a,
        Fq2 {
            c0: Fq::from_repr(FqRepr([
                0x07, 0x2b, 0xa1, 0x40, 0x49, 0xfe, 0x98, 0x81, 0x7f, 0x77, 0xa7, 0x18, 0x02, 0x1c,
                0x34, 0x2d, 0xe2, 0x55, 0x90, 0x26, 0x72, 0xef, 0x6c, 0x43, 0xa6, 0x5a, 0x6b, 0xe7,
                0x00, 0xb2, 0x85, 0xfe, 0x77, 0x56, 0xbe, 0xd7, 0xc1, 0x59, 0x82, 0xe9, 0x85, 0x65,
                0x75, 0x2b, 0xdb, 0x5c, 0x9b, 0x80,
            ]))
            .unwrap(),
            c1: Fq::from_repr(FqRepr([
                0x09, 0x57, 0x41, 0x13, 0x59, 0xba, 0x6e, 0x4c, 0x4c, 0xd5, 0xdd, 0x9f, 0xb4, 0x0b,
                0x3b, 0x8f, 0xf6, 0x39, 0x05, 0xf3, 0x9a, 0xd8, 0xcb, 0x1f, 0xe5, 0x26, 0x17, 0x93,
                0x05, 0x88, 0xc6, 0x9a, 0x11, 0xdf, 0x49, 0xbc, 0x6c, 0xac, 0xc2, 0x56, 0xeb, 0x4a,
                0xba, 0xf7, 0xc2, 0x55, 0xd1, 0xcd,
            ]))
            .unwrap(),
        }
    );
}

#[test]
fn test_fq2_negation() {
    use super::fq::FqRepr;
    use ff::PrimeField;

    let a = Fq2 {
        c0: Fq::from_repr(FqRepr([
            0x00, 0xf8, 0xd2, 0x95, 0xb2, 0xde, 0xd9, 0xdc, 0xcc, 0xc6, 0x49, 0xc4, 0xb9, 0x53,
            0x2b, 0xf3, 0xb9, 0x66, 0xce, 0x3b, 0xc2, 0x10, 0x8b, 0x13, 0x8b, 0x1a, 0x52, 0xe0,
            0xa9, 0x0f, 0x59, 0xed, 0x11, 0xe5, 0x9e, 0xa2, 0x21, 0xa3, 0xb6, 0xd2, 0x2d, 0x00,
            0x78, 0x03, 0x69, 0x23, 0xff, 0xc7,
        ]))
        .unwrap(),
        c1: Fq::from_repr(FqRepr([
            0x01, 0x2d, 0x11, 0x37, 0xb8, 0xa6, 0xa8, 0x37, 0x4e, 0x46, 0x4d, 0xea, 0x5b, 0xcf,
            0xd4, 0x1e, 0xb3, 0xf8, 0xaf, 0xc0, 0xee, 0x24, 0x8c, 0xad, 0xbe, 0x20, 0x34, 0x11,
            0xc6, 0x6f, 0xb3, 0xa5, 0x94, 0x6a, 0xe5, 0x2d, 0x68, 0x4f, 0xa7, 0xed, 0x97, 0x7d,
            0xf6, 0xef, 0xcd, 0xae, 0xe0, 0xdb,
        ]))
        .unwrap(),
    }
    .neg();
    assert_eq!(
        a,
        Fq2 {
            c0: Fq::from_repr(FqRepr([
                0x19, 0x08, 0x3f, 0x54, 0x86, 0xa1, 0x0c, 0xbd, 0x7e, 0x55, 0x5d, 0xf1, 0x89, 0xf8,
                0x80, 0xe3, 0xab, 0x10, 0x7d, 0x49, 0x31, 0x74, 0x87, 0xab, 0xdc, 0x16, 0x7f, 0xc0,
                0x4d, 0xa1, 0x9c, 0x37, 0x0c, 0xc6, 0x61, 0x5c, 0x8f, 0xb0, 0x49, 0x2d, 0x8c, 0xfe,
                0x87, 0xfc, 0x96, 0xdb, 0xaa, 0xe4,
            ]))
            .unwrap(),
            c1: Fq::from_repr(FqRepr([
                0x18, 0xd4, 0x00, 0xb2, 0x80, 0xd9, 0x3e, 0x62, 0xfc, 0xd5, 0x59, 0xcb, 0xe7, 0x7b,
                0xd8, 0xb8, 0xb0, 0x7e, 0x9b, 0xc4, 0x05, 0x60, 0x86, 0x11, 0xa9, 0x10, 0x9e, 0x8f,
                0x30, 0x41, 0x42, 0x7e, 0x8a, 0x41, 0x1a, 0xd1, 0x49, 0x04, 0x58, 0x12, 0x22, 0x81,
                0x09, 0x10, 0x32, 0x50, 0xc9, 0xd0,
            ]))
            .unwrap(),
        }
    );
}

#[test]
fn test_fq2_doubling() {
    use super::fq::FqRepr;
    use ff::PrimeField;

    let a = Fq2 {
        c0: Fq::from_repr(FqRepr([
            0x00, 0xf8, 0xd2, 0x95, 0xb2, 0xde, 0xd9, 0xdc, 0xcc, 0xc6, 0x49, 0xc4, 0xb9, 0x53,
            0x2b, 0xf3, 0xb9, 0x66, 0xce, 0x3b, 0xc2, 0x10, 0x8b, 0x13, 0x8b, 0x1a, 0x52, 0xe0,
            0xa9, 0x0f, 0x59, 0xed, 0x11, 0xe5, 0x9e, 0xa2, 0x21, 0xa3, 0xb6, 0xd2, 0x2d, 0x00,
            0x78, 0x03, 0x69, 0x23, 0xff, 0xc7,
        ]))
        .unwrap(),
        c1: Fq::from_repr(FqRepr([
            0x01, 0x2d, 0x11, 0x37, 0xb8, 0xa6, 0xa8, 0x37, 0x4e, 0x46, 0x4d, 0xea, 0x5b, 0xcf,
            0xd4, 0x1e, 0xb3, 0xf8, 0xaf, 0xc0, 0xee, 0x24, 0x8c, 0xad, 0xbe, 0x20, 0x34, 0x11,
            0xc6, 0x6f, 0xb3, 0xa5, 0x94, 0x6a, 0xe5, 0x2d, 0x68, 0x4f, 0xa7, 0xed, 0x97, 0x7d,
            0xf6, 0xef, 0xcd, 0xae, 0xe0, 0xdb,
        ]))
        .unwrap(),
    };
    assert_eq!(
        a.double(),
        Fq2 {
            c0: Fq::from_repr(FqRepr([
                0x01, 0xf1, 0xa5, 0x2b, 0x65, 0xbd, 0xb3, 0xb9, 0x99, 0x8c, 0x93, 0x89, 0x72, 0xa6,
                0x57, 0xe7, 0x72, 0xcd, 0x9c, 0x77, 0x84, 0x21, 0x16, 0x27, 0x16, 0x34, 0xa5, 0xc1,
                0x52, 0x1e, 0xb3, 0xda, 0x23, 0xcb, 0x3d, 0x44, 0x43, 0x47, 0x6d, 0xa4, 0x5a, 0x00,
                0xf0, 0x06, 0xd2, 0x47, 0xff, 0x8e,
            ]))
            .unwrap(),
            c1: Fq::from_repr(FqRepr([
                0x02, 0x5a, 0x22, 0x6f, 0x71, 0x4d, 0x50, 0x6e, 0x9c, 0x8c, 0x9b, 0xd4, 0xb7, 0x9f,
                0xa8, 0x3d, 0x67, 0xf1, 0x5f, 0x81, 0xdc, 0x49, 0x19, 0x5b, 0x7c, 0x40, 0x68, 0x23,
                0x8c, 0xdf, 0x67, 0x4b, 0x28, 0xd5, 0xca, 0x5a, 0xd0, 0x9f, 0x4f, 0xdb, 0x2e, 0xfb,
                0xed, 0xdf, 0x9b, 0x5d, 0xc1, 0xb6,
            ]))
            .unwrap(),
        }
    );
}

#[test]
fn test_fq2_frobenius_map() {
    use super::fq::FqRepr;
    use ff::PrimeField;

    let mut a = Fq2 {
        c0: Fq::from_repr(FqRepr([
            0x00, 0xf8, 0xd2, 0x95, 0xb2, 0xde, 0xd9, 0xdc, 0xcc, 0xc6, 0x49, 0xc4, 0xb9, 0x53,
            0x2b, 0xf3, 0xb9, 0x66, 0xce, 0x3b, 0xc2, 0x10, 0x8b, 0x13, 0x8b, 0x1a, 0x52, 0xe0,
            0xa9, 0x0f, 0x59, 0xed, 0x11, 0xe5, 0x9e, 0xa2, 0x21, 0xa3, 0xb6, 0xd2, 0x2d, 0x00,
            0x78, 0x03, 0x69, 0x23, 0xff, 0xc7,
        ]))
        .unwrap(),
        c1: Fq::from_repr(FqRepr([
            0x01, 0x2d, 0x11, 0x37, 0xb8, 0xa6, 0xa8, 0x37, 0x4e, 0x46, 0x4d, 0xea, 0x5b, 0xcf,
            0xd4, 0x1e, 0xb3, 0xf8, 0xaf, 0xc0, 0xee, 0x24, 0x8c, 0xad, 0xbe, 0x20, 0x34, 0x11,
            0xc6, 0x6f, 0xb3, 0xa5, 0x94, 0x6a, 0xe5, 0x2d, 0x68, 0x4f, 0xa7, 0xed, 0x97, 0x7d,
            0xf6, 0xef, 0xcd, 0xae, 0xe0, 0xdb,
        ]))
        .unwrap(),
    };
    a.frobenius_map(0);
    assert_eq!(
        a,
        Fq2 {
            c0: Fq::from_repr(FqRepr([
                0x00, 0xf8, 0xd2, 0x95, 0xb2, 0xde, 0xd9, 0xdc, 0xcc, 0xc6, 0x49, 0xc4, 0xb9, 0x53,
                0x2b, 0xf3, 0xb9, 0x66, 0xce, 0x3b, 0xc2, 0x10, 0x8b, 0x13, 0x8b, 0x1a, 0x52, 0xe0,
                0xa9, 0x0f, 0x59, 0xed, 0x11, 0xe5, 0x9e, 0xa2, 0x21, 0xa3, 0xb6, 0xd2, 0x2d, 0x00,
                0x78, 0x03, 0x69, 0x23, 0xff, 0xc7,
            ]))
            .unwrap(),
            c1: Fq::from_repr(FqRepr([
                0x01, 0x2d, 0x11, 0x37, 0xb8, 0xa6, 0xa8, 0x37, 0x4e, 0x46, 0x4d, 0xea, 0x5b, 0xcf,
                0xd4, 0x1e, 0xb3, 0xf8, 0xaf, 0xc0, 0xee, 0x24, 0x8c, 0xad, 0xbe, 0x20, 0x34, 0x11,
                0xc6, 0x6f, 0xb3, 0xa5, 0x94, 0x6a, 0xe5, 0x2d, 0x68, 0x4f, 0xa7, 0xed, 0x97, 0x7d,
                0xf6, 0xef, 0xcd, 0xae, 0xe0, 0xdb,
            ]))
            .unwrap(),
        }
    );
    a.frobenius_map(1);
    assert_eq!(
        a,
        Fq2 {
            c0: Fq::from_repr(FqRepr([
                0x00, 0xf8, 0xd2, 0x95, 0xb2, 0xde, 0xd9, 0xdc, 0xcc, 0xc6, 0x49, 0xc4, 0xb9, 0x53,
                0x2b, 0xf3, 0xb9, 0x66, 0xce, 0x3b, 0xc2, 0x10, 0x8b, 0x13, 0x8b, 0x1a, 0x52, 0xe0,
                0xa9, 0x0f, 0x59, 0xed, 0x11, 0xe5, 0x9e, 0xa2, 0x21, 0xa3, 0xb6, 0xd2, 0x2d, 0x00,
                0x78, 0x03, 0x69, 0x23, 0xff, 0xc7,
            ]))
            .unwrap(),
            c1: Fq::from_repr(FqRepr([
                0x18, 0xd4, 0x00, 0xb2, 0x80, 0xd9, 0x3e, 0x62, 0xfc, 0xd5, 0x59, 0xcb, 0xe7, 0x7b,
                0xd8, 0xb8, 0xb0, 0x7e, 0x9b, 0xc4, 0x05, 0x60, 0x86, 0x11, 0xa9, 0x10, 0x9e, 0x8f,
                0x30, 0x41, 0x42, 0x7e, 0x8a, 0x41, 0x1a, 0xd1, 0x49, 0x04, 0x58, 0x12, 0x22, 0x81,
                0x09, 0x10, 0x32, 0x50, 0xc9, 0xd0,
            ]))
            .unwrap(),
        }
    );
    a.frobenius_map(1);
    assert_eq!(
        a,
        Fq2 {
            c0: Fq::from_repr(FqRepr([
                0x00, 0xf8, 0xd2, 0x95, 0xb2, 0xde, 0xd9, 0xdc, 0xcc, 0xc6, 0x49, 0xc4, 0xb9, 0x53,
                0x2b, 0xf3, 0xb9, 0x66, 0xce, 0x3b, 0xc2, 0x10, 0x8b, 0x13, 0x8b, 0x1a, 0x52, 0xe0,
                0xa9, 0x0f, 0x59, 0xed, 0x11, 0xe5, 0x9e, 0xa2, 0x21, 0xa3, 0xb6, 0xd2, 0x2d, 0x00,
                0x78, 0x03, 0x69, 0x23, 0xff, 0xc7,
            ]))
            .unwrap(),
            c1: Fq::from_repr(FqRepr([
                0x01, 0x2d, 0x11, 0x37, 0xb8, 0xa6, 0xa8, 0x37, 0x4e, 0x46, 0x4d, 0xea, 0x5b, 0xcf,
                0xd4, 0x1e, 0xb3, 0xf8, 0xaf, 0xc0, 0xee, 0x24, 0x8c, 0xad, 0xbe, 0x20, 0x34, 0x11,
                0xc6, 0x6f, 0xb3, 0xa5, 0x94, 0x6a, 0xe5, 0x2d, 0x68, 0x4f, 0xa7, 0xed, 0x97, 0x7d,
                0xf6, 0xef, 0xcd, 0xae, 0xe0, 0xdb,
            ]))
            .unwrap(),
        }
    );
    a.frobenius_map(2);
    assert_eq!(
        a,
        Fq2 {
            c0: Fq::from_repr(FqRepr([
                0x00, 0xf8, 0xd2, 0x95, 0xb2, 0xde, 0xd9, 0xdc, 0xcc, 0xc6, 0x49, 0xc4, 0xb9, 0x53,
                0x2b, 0xf3, 0xb9, 0x66, 0xce, 0x3b, 0xc2, 0x10, 0x8b, 0x13, 0x8b, 0x1a, 0x52, 0xe0,
                0xa9, 0x0f, 0x59, 0xed, 0x11, 0xe5, 0x9e, 0xa2, 0x21, 0xa3, 0xb6, 0xd2, 0x2d, 0x00,
                0x78, 0x03, 0x69, 0x23, 0xff, 0xc7,
            ]))
            .unwrap(),
            c1: Fq::from_repr(FqRepr([
                0x01, 0x2d, 0x11, 0x37, 0xb8, 0xa6, 0xa8, 0x37, 0x4e, 0x46, 0x4d, 0xea, 0x5b, 0xcf,
                0xd4, 0x1e, 0xb3, 0xf8, 0xaf, 0xc0, 0xee, 0x24, 0x8c, 0xad, 0xbe, 0x20, 0x34, 0x11,
                0xc6, 0x6f, 0xb3, 0xa5, 0x94, 0x6a, 0xe5, 0x2d, 0x68, 0x4f, 0xa7, 0xed, 0x97, 0x7d,
                0xf6, 0xef, 0xcd, 0xae, 0xe0, 0xdb,
            ]))
            .unwrap(),
        }
    );
}

#[test]
fn test_fq2_sqrt() {
    use super::fq::FqRepr;
    use ff::PrimeField;

    assert_eq!(
        Fq2 {
            c0: Fq::from_repr(FqRepr([
                0x07, 0xca, 0x7d, 0xa1, 0xf1, 0x36, 0x06, 0xac, 0x1e, 0x58, 0xb2, 0x15, 0x9d, 0xfe,
                0x10, 0xe2, 0xdb, 0x4a, 0x11, 0x6b, 0x5b, 0xf7, 0x4a, 0xa1, 0xa5, 0x7e, 0x6f, 0xc1,
                0xba, 0xb5, 0x1f, 0xd9, 0x03, 0x4c, 0x2d, 0x04, 0xfa, 0xff, 0xda, 0xb6, 0x47, 0x6b,
                0x4c, 0x30, 0x97, 0x20, 0xe2, 0x27,
            ]))
            .unwrap(),
            c1: Fq::from_repr(FqRepr([
                0x0e, 0xc9, 0x23, 0x36, 0x65, 0x0e, 0x49, 0xd5, 0x08, 0xee, 0x53, 0x94, 0xd7, 0x7a,
                0xfb, 0x3d, 0x21, 0x26, 0x11, 0xbc, 0xa4, 0xe9, 0x91, 0x21, 0x4c, 0xec, 0x2d, 0xca,
                0x57, 0x7a, 0x3e, 0xb6, 0x37, 0x1a, 0x75, 0xed, 0x14, 0xf4, 0x16, 0x29, 0xfa, 0x8d,
                0xe8, 0x8b, 0x75, 0x16, 0xd2, 0xc3,
            ]))
            .unwrap(),
        }
        .sqrt()
        .unwrap(),
        Fq2 {
            c0: Fq::from_repr(FqRepr([
                0x10, 0xf6, 0x96, 0x3b, 0xba, 0xd2, 0xeb, 0xc5, 0x88, 0x1b, 0x3e, 0x01, 0xb6, 0x11,
                0xc0, 0x70, 0x8d, 0x7f, 0x1f, 0x72, 0x3d, 0x02, 0xc1, 0xd3, 0x6d, 0x2d, 0xdb, 0xe5,
                0x52, 0x20, 0x3e, 0x82, 0x6e, 0xf7, 0xde, 0x92, 0xe8, 0xc6, 0x8b, 0x63, 0x40, 0xb2,
                0x99, 0xb2, 0x70, 0x42, 0x58, 0xc5,
            ]))
            .unwrap(),
            c1: Fq::from_repr(FqRepr([
                0x08, 0xd7, 0xcf, 0xff, 0x94, 0x21, 0x63, 0x30, 0xa4, 0xc9, 0x3b, 0x08, 0x10, 0x5d,
                0x71, 0xa9, 0x6b, 0x85, 0x2a, 0xea, 0xf2, 0xaf, 0xcb, 0x1b, 0x28, 0xa2, 0x0f, 0xae,
                0xd2, 0x11, 0xef, 0xe7, 0x76, 0x70, 0x59, 0x46, 0x65, 0x67, 0x64, 0x47, 0xc0, 0x99,
                0x53, 0x4f, 0xc2, 0x09, 0xe7, 0x52,
            ]))
            .unwrap(),
        }
    );

    assert_eq!(
        Fq2 {
            c0: Fq::from_repr(FqRepr([
                0x1a, 0x01, 0x11, 0xea, 0x39, 0x7f, 0xe6, 0x9a, 0x4b, 0x1b, 0xa7, 0xb6, 0x43, 0x4b,
                0xac, 0xd7, 0x64, 0x77, 0x4b, 0x84, 0xf3, 0x85, 0x12, 0xbf, 0x67, 0x30, 0xd2, 0xa0,
                0xf6, 0xb0, 0xf6, 0x24, 0x1e, 0xab, 0xff, 0xfe, 0xb1, 0x53, 0xff, 0xff, 0xb9, 0xf7,
                0x84, 0x29, 0xd1, 0x51, 0x7a, 0x6b,
            ]))
            .unwrap(),
            c1: Fq::zero(),
        }
        .sqrt()
        .unwrap(),
        Fq2 {
            c0: Fq::zero(),
            c1: Fq::from_repr(FqRepr([
                0x1a, 0x01, 0x11, 0xea, 0x39, 0x7f, 0xe6, 0x9a, 0x4b, 0x1b, 0xa7, 0xb6, 0x43, 0x4b,
                0xac, 0xd7, 0x64, 0x77, 0x4b, 0x84, 0xf3, 0x85, 0x12, 0xbf, 0x67, 0x30, 0xd2, 0xa0,
                0xf6, 0xb0, 0xf6, 0x24, 0x1e, 0xab, 0xff, 0xfe, 0xb1, 0x53, 0xff, 0xff, 0xb9, 0xfe,
                0xff, 0xff, 0xfd, 0x43, 0x57, 0xa3,
            ]))
            .unwrap(),
        }
    );
}

#[cfg(test)]
use rand_core::SeedableRng;
#[cfg(test)]
use rand_xorshift::XorShiftRng;

#[test]
fn test_fq2_mul_nonresidue() {
    let mut rng = XorShiftRng::from_seed([
        0x59, 0x62, 0xbe, 0x5d, 0x76, 0x3d, 0x31, 0x8d, 0x17, 0xdb, 0x37, 0x32, 0x54, 0x06, 0xbc,
        0xe5,
    ]);

    let nqr = Fq2 {
        c0: Fq::one(),
        c1: Fq::one(),
    };

    for _ in 0..1000 {
        let mut a = Fq2::random(&mut rng);
        let mut b = a;
        a.mul_by_nonresidue();
        b.mul_assign(&nqr);

        assert_eq!(a, b);
    }
}

#[test]
fn fq2_field_tests() {
    use ff::PrimeField;

    crate::tests::field::random_field_tests::<Fq2>();
    crate::tests::field::random_sqrt_tests::<Fq2>();
    crate::tests::field::random_frobenius_tests::<Fq2, _>(super::fq::Fq::char(), 13);
}
