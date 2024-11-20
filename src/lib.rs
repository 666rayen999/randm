pub trait RandomT {
    fn random(r: &mut Random) -> Self;
}

macro_rules! rand {
    (bool) => {
        impl RandomT for bool {
            #[inline]
            fn random(r: &mut Random) -> Self {
                (r.next() & 1) > 0
            }
        }
    };
    (char) => {
        impl RandomT for char {
            #[inline]
            fn random(r: &mut Random) -> Self {
                char::from_u32(r.get::<u8>() as u32).unwrap_or(' ')
            }
        }
    };
    (u64) => {
        impl RandomT for u64 {
            #[inline]
            fn random(r: &mut Random) -> Self {
                r.next()
            }
        }
    };
    (_ $t: ty) => {
        impl RandomT for $t {
            #[inline]
            fn random(r: &mut Random) -> Self {
                ((r.next() as $t) << 64) | (r.next() as $t)
            }
        }
    };
    (f32) => {
        impl RandomT for f32 {
            #[inline]
            fn random(r: &mut Random) -> Self {
                Self::from_bits((r.get::<u32>() & 0x7fffff) | 0x3f800000) - 1.0
            }
        }
    };
    (f64) => {
        impl RandomT for f64 {
            #[inline]
            fn random(r: &mut Random) -> Self {
                Self::from_bits((r.get::<u64>() & 0xfffffffffffff) | 0x3ff0000000000000) - 1.0
            }
        }
    };
    ($t: ty) => {
        impl RandomT for $t {
            #[inline]
            fn random(r: &mut Random) -> Self {
                r.next() as Self
            }
        }
    };
}

rand!(char);
rand!(bool);
rand!(u8);
rand!(u16);
rand!(u32);
rand!(u64);
rand!(_ u128);
rand!(usize);
rand!(i8);
rand!(i16);
rand!(i32);
rand!(i64);
rand!(_ i128);
rand!(isize);
rand!(f32);
rand!(f64);

macro_rules! tpl {
    ($($name:ident),*) => {
        impl<$($name: RandomT),*> RandomT for ($($name,)*) {
            fn random(r: &mut Random) -> Self {
                ($($name::random(r),)*)
            }
        }
    };
}

tpl!(A);
tpl!(A, B);
tpl!(A, B, C);
tpl!(A, B, C, D);
tpl!(A, B, C, D, E);
tpl!(A, B, C, D, E, F);
tpl!(A, B, C, D, E, F, G);
tpl!(A, B, C, D, E, F, G, H);
tpl!(A, B, C, D, E, F, G, H, I);
tpl!(A, B, C, D, E, F, G, H, I, J);
tpl!(A, B, C, D, E, F, G, H, I, J, K);
tpl!(A, B, C, D, E, F, G, H, I, J, K, L);

impl<T: RandomT, const N: usize> RandomT for [T; N] {
    fn random(r: &mut Random) -> Self {
        std::array::from_fn(|_| T::random(r))
    }
}

pub struct Random(u64);

impl Random {
    #[inline]
    pub fn seed(seed: u64) -> Self {
        Self(seed)
    }
    #[inline]
    pub fn new() -> Self {
        Self(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(6123),
        )
    }
    #[inline]
    fn next(&mut self) -> u64 {
        self.0 += 1;
        self.0 ^= self.0 << 7;
        self.0 ^= self.0 >> 9;
        self.0
    }
    #[inline]
    pub fn get<T: RandomT>(&mut self) -> T {
        T::random(self)
    }
}
